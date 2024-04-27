use std::collections::{HashMap, HashSet};

use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// Constructs a clique graph given cliques of a graph.
/// The clique graph consists of vertices which represent the cliques (bags)
/// and edges that connect two vertices if the intersection of the corresponding cliques is not empty.
pub fn construct_clique_graph<InnerCollection, OuterIterator>(
    cliques: OuterIterator,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
) -> Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected>
where
    OuterIterator: IntoIterator<Item = InnerCollection>,
    InnerCollection: IntoIterator<Item = NodeIndex>,
{
    let mut result_graph: Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected> =
        Graph::new_undirected();
    for clique in cliques {
        let vertex_index = result_graph.add_node(HashSet::from_iter(clique.into_iter()));
        for other_vertex_index in result_graph.node_indices() {
            if other_vertex_index == vertex_index {
                continue;
            } else {
                let other_vertex_weight = result_graph
                    .node_weight(other_vertex_index)
                    .expect("Node weight should exist");
                let this_vertex_weight = result_graph
                    .node_weight(vertex_index)
                    .expect("Node weight should exist");

                if let Some(_) = this_vertex_weight.intersection(other_vertex_weight).next() {
                    // Add edge, if cliques (that are the nodes of result graph) have nodes in common
                    result_graph.add_edge(
                        vertex_index,
                        other_vertex_index,
                        edge_weight_heuristic(this_vertex_weight, other_vertex_weight),
                    );
                }
            }
        }
    }

    result_graph
}

/// Constructs a clique graph given cliques of a graph.
/// The clique graph consists of vertices which represent the cliques (bags)
/// and edges that connect two vertices if the intersection of the corresponding cliques is not empty.
///
/// Returns a tuple of the clique graph and a HashMap mapping the vertices in the original graph (the
/// vertices from the bags) to HashSets containing the NodeIndices of all the Bags in the Clique Graph
/// that contain the vertex from the original graph.
pub fn construct_clique_graph_with_bags<InnerCollection, OuterIterator>(
    cliques: OuterIterator,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
) -> (
    Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected>,
    HashMap<NodeIndex, HashSet<NodeIndex>>,
)
where
    OuterIterator: IntoIterator<Item = InnerCollection>,
    InnerCollection: IntoIterator<Item = NodeIndex>,
    InnerCollection: Clone,
{
    let mut result_graph: Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected> =
        Graph::new_undirected();
    let mut result_map: HashMap<NodeIndex, HashSet<NodeIndex>> = HashMap::new();

    for clique in cliques {
        let vertex_index = result_graph.add_node(HashSet::from_iter(clique.clone().into_iter()));
        for vertex_in_clique in clique {
            add_node_index_to_bag_in_hashmap(&mut result_map, vertex_in_clique, vertex_index);
        }
        for other_vertex_index in result_graph.node_indices() {
            if other_vertex_index == vertex_index {
                continue;
            } else {
                let other_vertex_weight = result_graph
                    .node_weight(other_vertex_index)
                    .expect("Node weight should exist");
                let vertex_weight = result_graph
                    .node_weight(vertex_index)
                    .expect("Node weight - in this case the nodes in the clique - should exist");

                if let Some(_) = vertex_weight.intersection(other_vertex_weight).next() {
                    // Add edge, if cliques (that are the nodes of result graph) have nodes in common
                    result_graph.add_edge(
                        vertex_index,
                        other_vertex_index,
                        edge_weight_heuristic(
                            vertex_weight,
                            other_vertex_weight,
                        ),
                    );
                }
            }
        }
    }

    (result_graph, result_map)
}

fn add_node_index_to_bag_in_hashmap(
    map: &mut HashMap<NodeIndex, HashSet<NodeIndex>>,
    vertex_in_graph: NodeIndex,
    vertex_in_clique_graph: NodeIndex,
) {
    if let Some(set) = map.get_mut(&vertex_in_graph) {
        set.insert(vertex_in_clique_graph);
    } else {
        let mut set = HashSet::new();
        set.insert(vertex_in_clique_graph);
        map.insert(vertex_in_graph, set);
    }
}
