use itertools::Itertools;
use log::{debug, info};
use petgraph::{algo::Measure, graph::NodeIndex, Graph};
use std::{
    cmp::Ordering,
    collections::{BTreeSet, HashMap, HashSet},
    fmt::Debug,
};

#[derive(PartialEq, Eq, Debug)]
struct Predecessor {
    node_index: NodeIndex,
    level_index: usize,
}

impl Ord for Predecessor {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::*;
        match self.level_index.cmp(&other.level_index) {
            Less => Less,
            Greater => Greater,
            Equal => self.node_index.cmp(&other.node_index),
        }
    }
}

impl PartialOrd for Predecessor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        match self.level_index.partial_cmp(&other.level_index) {
            Some(Equal) => self.node_index.partial_cmp(&other.node_index),
            any => any,
        }
    }
}

/// Given a tree graph with bags (HashSets) as Vertices, checks all 2-combinations of bags for non-empty-intersection
/// and inserts the intersecting nodes in all bags that are along the (unique) path of the two bags in the tree.
pub fn fill_bags_along_paths<E: Copy + Measure + Default>(
    graph: &mut Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
) {
    // Finding out which paths between bags have to be checked
    for mut vec in graph.node_indices().combinations(2) {
        let first_index = vec.pop().expect("Vec should contain two items");
        let second_index = vec.pop().expect("Vec should contain two items");

        let first_weight = graph
            .node_weight(first_index)
            .expect("Node weight should exist");
        let second_weight = graph
            .node_weight(second_index)
            .expect("Node weight should exist");

        let mut intersection_iterator = first_weight.intersection(second_weight).cloned();
        if let Some(vertex_in_both_bags) = intersection_iterator.next() {
            // Bags of vertices in clique graph intersect and path between them needs to be filled up / checked
            let mut intersection_vec: Vec<NodeIndex> = intersection_iterator.collect();
            intersection_vec.push(vertex_in_both_bags);

            let mut path: Vec<_> = petgraph::algo::simple_paths::all_simple_paths::<
                Vec<NodeIndex>,
                _,
            >(&*graph, first_index, second_index, 0, None)
            .next()
            .expect("There should be a path in the tree");

            // Last element is the given end node
            path.pop();

            // Add the elements that are in both the bag of the starting and the end vertex to all bags
            // of the vertices on the path between them
            for node_index in path {
                if node_index != first_index {
                    graph
                        .node_weight_mut(node_index)
                        .expect("Bag for the vertex should exist")
                        .extend(intersection_vec.iter().cloned());
                }
            }
        }
    }
}

/// Given a tree graph with bags (HashSets) as Vertices, checks all 2-combinations of bags for non-empty-intersection
/// and inserts the intersecting nodes in all bags that are along the (unique) path of the two bags in the tree.
pub fn fill_bags_along_paths_abusing_structure<E: Copy + Default + Debug>(
    graph: &mut Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
    map: &HashMap<NodeIndex, HashSet<NodeIndex>>,
) -> HashMap<NodeIndex, (NodeIndex, usize)> {
    info!("Building tree structure");

    let mut tree_predecessor_map: HashMap<NodeIndex, (NodeIndex, usize)> = HashMap::new();
    let root = graph
        .node_indices()
        .max_by_key(|v| graph.neighbors(*v).collect::<Vec<_>>().len())
        .expect("Graph shouldn't be empty");
    setup_predecessors(&graph, &mut tree_predecessor_map, root);

    // DEBUG
    debug!(
        "Clique Tree Graph currently looks like this: {:?} \n",
        graph
    );
    debug!(
        "Predecessor map looks like this: {:?}",
        tree_predecessor_map
    );

    for vertex_in_initial_graph in map.keys() {
        info!("Filling up bags");
        fill_bags_until_common_predecessor(
            graph,
            &tree_predecessor_map,
            &vertex_in_initial_graph,
            &map.get(vertex_in_initial_graph)
                .expect("key should exist by loop invariant"),
        )
    }

    // DEBUG
    debug!(
        "Clique Tree Graph looks like this after filling up: {:?} \n",
        graph
    );

    tree_predecessor_map
}

/// Sets up the predecessor map such that each node has a predecessor going back to the root node.
/// Additionally there is an index, indicating the depth level at which the predecessor is (root is 0)
fn setup_predecessors<E>(
    graph: &Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
    predecessors_map: &mut HashMap<NodeIndex, (NodeIndex, usize)>,
    root: NodeIndex,
) {
    let mut stack: Vec<(NodeIndex, usize)> = Vec::new();
    stack.push((root, 0));

    while !stack.is_empty() {
        let (current_vertex, current_index) =
            stack.pop().expect("Stack is not empty by loop invariant");

        for next_vertex in graph.neighbors(current_vertex) {
            if !predecessors_map.contains_key(&next_vertex) && next_vertex != root {
                predecessors_map.insert(next_vertex, (current_vertex, current_index));
                stack.push((next_vertex, current_index + 1));
            }
        }
    }

    assert_eq!(
        predecessors_map.len(),
        graph.node_count() - 1,
        "Predecessor Map doesn't contain predecessors for all vertices (root is excluded)"
    );
    assert!(
        !predecessors_map.contains_key(&root),
        "Root shouldn't have predecessor in predecessor map"
    );
}

fn fill_bags_until_common_predecessor<E>(
    clique_graph: &mut Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
    predecessors_map: &HashMap<NodeIndex, (NodeIndex, usize)>,
    vertex_in_initial_graph: &NodeIndex,
    vertices_in_clique_graph: &HashSet<NodeIndex>,
) {
    let mut predecessors: BTreeSet<Predecessor> = BTreeSet::new();
    if vertices_in_clique_graph.len() > 1 {
        for vertex_in_clique_graph in vertices_in_clique_graph {
            if let Some((predecessor, index)) = predecessors_map.get(vertex_in_clique_graph) {
                // Skip the vertex in clique graph since it already contains the vertex in initial graph
                predecessors.insert(Predecessor {
                    node_index: *predecessor,
                    level_index: *index,
                });
            } else {
                // If there is no predecessor, vertex_in_clique_graph is the root node and as such
                // is the common predecessors and path's need to be filled up until there
                predecessors.insert(Predecessor {
                    node_index: *vertex_in_clique_graph,
                    level_index: 0,
                });
            }
        }
    }

    // DEBUG
    debug!("Currently filling in {:?}", vertex_in_initial_graph);

    // Loop that looks at ancestor of vertex with highest level index in tree. Inserts the ancestors
    // in the predecessors, not inserting duplicates. If only one ancestor is left, the common ancestor is found.
    while predecessors.len() > 1 {
        // DEBUG
        debug!("Predecessors: {:?}", predecessors);
        // Current vertex should be the one with the highest level index in the tree
        let current_vertex_in_clique_graph = predecessors
            .pop_last()
            .expect("Collection shouldn't be empty by loop invariant");
        //DEBUG
        debug!("Current vertex: {:?}", current_vertex_in_clique_graph);

        //DEBUG
        debug!(
            "Filling in {:?} into {:?}",
            vertex_in_initial_graph, current_vertex_in_clique_graph
        );
        // Insert the vertex from the original graph in the bag of the current vertex in the clique graph
        // that is on the path to the common ancestor
        clique_graph
            .node_weight_mut(current_vertex_in_clique_graph.node_index)
            .expect("Bag for the vertex should exist")
            .insert(*vertex_in_initial_graph);

        if let Some((predecessor_clique_graph_vertex, index)) =
            predecessors_map.get(&current_vertex_in_clique_graph.node_index)
        {
            let predecessor = Predecessor {
                node_index: *predecessor_clique_graph_vertex,
                level_index: *index,
            };
            // DEBUG
            debug!(
                "Current vertex is: {:?}, predecessor is: {:?}",
                current_vertex_in_clique_graph, predecessor
            );
            predecessors.insert(predecessor);
            debug!(
                "After inserting predecessor the predecessors look like this: {:?} \n \n",
                predecessors
            );
        }
    }
    // This is reached once the common ancestor is found and the only element left in the collection
    if let Some(common_predecessor) = predecessors.first() {
        // DEBUG
        debug!(
            "Filling in vertex from initial graph: {:?} into common ancestor: {:?}",
            vertex_in_initial_graph, common_predecessor
        );
        clique_graph
            .node_weight_mut(common_predecessor.node_index)
            .expect("Bag for the vertex should exist")
            .insert(*vertex_in_initial_graph);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predecessor_eq() {
        let predecessor_one = Predecessor {
            node_index: NodeIndex::new(1),
            level_index: 1,
        };
        let predecessor_two = Predecessor {
            node_index: NodeIndex::new(5),
            level_index: 1,
        };

        let mut predecessors: BTreeSet<Predecessor> = BTreeSet::new();
        predecessors.insert(predecessor_one);
        predecessors.insert(predecessor_two);

        assert_eq!(predecessors.len(), 2);
    }
}
