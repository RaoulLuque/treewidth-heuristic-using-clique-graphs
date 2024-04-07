use super::find_path_in_tree::find_path_in_tree;
use itertools::Itertools;
use petgraph::{
    graph::NodeIndex,
    visit::{IntoNodeReferences, NodeRef},
    Graph,
};
use std::collections::HashSet;

/// Given a tree graph with bags (HashSets) as Vertices, checks all 2-combinations of bags for non-empty-intersection
/// and inserts the intersecting nodes in all bags that are along the (unique) path of the two bags in the tree.
pub fn fill_bags_along_path<E>(
    mut graph: Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
) {
    let mut vec_of_bags_that_need_to_be_connected: Vec<(NodeIndex, NodeIndex, Vec<NodeIndex>)> =
        Vec::new();

    // Finding out which paths between bags have to be checked
    for mut vec in graph.node_references().combinations(2) {
        let first_tuple = vec.pop().expect("Vec should contain two items");
        let second_tuple = vec.pop().expect("Vec should contain two items");
        let (first_id, first_weight, second_id, second_weight) = (
            first_tuple.id(),
            first_tuple.weight(),
            second_tuple.id(),
            second_tuple.weight(),
        );

        let mut intersection_iterator = first_weight.intersection(second_weight).cloned();
        if let Some(vertex_in_both_bags) = intersection_iterator.next() {
            let mut intersection_vec: Vec<NodeIndex> = intersection_iterator.collect();
            intersection_vec.push(vertex_in_both_bags);
            vec_of_bags_that_need_to_be_connected.push((first_id, second_id, intersection_vec));
        }
    }

    // Filling up the bags along the paths of the vertices
    for (first_id, second_id, intersection_vec) in vec_of_bags_that_need_to_be_connected {
        let mut path = find_path_in_tree::<
            &Graph<HashSet<NodeIndex>, E, petgraph::prelude::Undirected>,
            Vec<_>,
        >(&graph, first_id, second_id)
        .expect("Paths should exist between all 2 vertices in a tree");

        // Last element is the given end node
        path.pop();

        // Add the elements that are in both the bag of the starting and end vertex to all bags
        // of the vertices on the path between them
        for node_index in path {
            if node_index != first_id {
                graph
                    .node_weight_mut(node_index)
                    .expect("Bag for the vertex should exist")
                    .extend(intersection_vec.iter().cloned());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;

    use super::*;

    #[test]
    fn test() {
        let mut graph: Graph<u32, u32, petgraph::prelude::Undirected> =
            petgraph::Graph::new_undirected();

        let nodes = [
            graph.add_node(0),
            graph.add_node(0),
            graph.add_node(0),
            graph.add_node(0),
            graph.add_node(0),
            graph.add_node(0),
            graph.add_node(0),
        ];

        graph.add_edge(nodes[0], nodes[1], 0);
        graph.add_edge(nodes[0], nodes[2], 0);
        graph.add_edge(nodes[0], nodes[5], 0);
        graph.add_edge(nodes[1], nodes[2], 0);
        graph.add_edge(nodes[1], nodes[3], 0);
        graph.add_edge(nodes[1], nodes[5], 0);
        graph.add_edge(nodes[2], nodes[5], 0);
        graph.add_edge(nodes[3], nodes[4], 0);
        graph.add_edge(nodes[3], nodes[5], 0);
        graph.add_edge(nodes[3], nodes[6], 0);
        graph.add_edge(nodes[4], nodes[6], 0);

        let min_spanning_tree_graph: Graph<u32, u32, petgraph::prelude::Undirected> =
            petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(&graph));
    }
}
