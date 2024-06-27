use std::collections::{HashMap, HashSet};
use std::hash::BuildHasher;

use petgraph::graph::NodeIndex;
use petgraph::Graph;

/// Constructs the intersection graph of the given cliques (aka the clique graph if the set of
/// cliques is the set of maximal cliques). The edge weights are determined according to the edge
/// weight function.
pub fn construct_clique_graph<InnerCollection, OuterIterator, O, S: Default + BuildHasher>(
    cliques: OuterIterator,
    edge_weight_function: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
) -> Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected>
where
    OuterIterator: IntoIterator<Item = InnerCollection>,
    InnerCollection: IntoIterator<Item = NodeIndex>,
{
    let mut result_graph: Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected> =
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
                        edge_weight_function(this_vertex_weight, other_vertex_weight),
                    );
                }
            }
        }
    }

    result_graph
}

/// Constructs the same graph as [construct_clique_graph].
///
/// Additionally returns a HashMap mapping the vertices in the original graph (the
/// vertices from the cliques) to HashSets containing the NodeIndices of all the Bags in the Clique Graph
/// that contain the vertex from the original graph.
pub fn construct_clique_graph_with_bags<
    InnerCollection,
    OuterIterator,
    O,
    S: Default + BuildHasher,
>(
    cliques: OuterIterator,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
) -> (
    Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected>,
    HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
)
where
    OuterIterator: IntoIterator<Item = InnerCollection>,
    InnerCollection: IntoIterator<Item = NodeIndex>,
    InnerCollection: Clone,
{
    let mut result_graph: Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected> =
        Graph::new_undirected();
    let mut result_map: HashMap<NodeIndex, HashSet<NodeIndex, S>, S> = Default::default();

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
                        edge_weight_heuristic(vertex_weight, other_vertex_weight),
                    );
                }
            }
        }
    }

    (result_graph, result_map)
}

/// Given a node from the original graph and a bag/vertex in the clique graph, adds this connection
/// to the hashmap (node from original graph -> HashSet containing node from clique graph).
fn add_node_index_to_bag_in_hashmap<S: Default + std::hash::BuildHasher>(
    map: &mut HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    vertex_in_graph: NodeIndex,
    vertex_in_clique_graph: NodeIndex,
) {
    if let Some(set) = map.get_mut(&vertex_in_graph) {
        set.insert(vertex_in_clique_graph);
    } else {
        let mut set: HashSet<NodeIndex, S> = Default::default();
        set.insert(vertex_in_clique_graph);
        map.insert(vertex_in_graph, set);
    }
}
