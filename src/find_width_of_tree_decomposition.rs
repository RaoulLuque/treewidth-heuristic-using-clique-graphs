use petgraph::{graph::NodeIndex, Graph};
use std::collections::HashSet;

/// Returns the maximum size of one of the bags in the tree decomposition graph.
/// This equals the highest len of one of the vertices in the graph. Returns 0 if the graph has no vertices
///
/// Returns 0 if the graph is empty
pub fn find_width_of_tree_decomposition<E, S>(
    graph: &Graph<HashSet<NodeIndex, S>, E, petgraph::prelude::Undirected>,
) -> usize {
    if let Some(bag) = graph.node_weights().max_by_key(|b| b.len()) {
        bag.len() - 1
    } else {
        0
    }
}
