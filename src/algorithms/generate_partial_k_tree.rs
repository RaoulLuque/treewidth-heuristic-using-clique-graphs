use petgraph::{
    graph::NodeIndex,
    visit::{IntoEdgeReferences, IntoEdges, IntoNodeIdentifiers},
    Graph, Undirected,
};
use rand::{seq::IteratorRandom, Rng};

use crate::algorithms::minimum_maximum_degree_heuristic::minimum_maximum_degree_heuristic;

/// Generates a [k-tree](https://en.wikipedia.org/wiki/K-tree) and then randomly removes p percent of the edges
/// to get a [partial k-tree](https://en.wikipedia.org/wiki/Partial_k-tree). To guarantee a treewidth of k,
/// this procedure is repeated until the treewidth of the graph is at least k according to the minimum
/// maximum degree heuristic.
///
/// **Caution!**: Due to the randomness involved, this function could in theory take indefinitely to generate
/// a partial k-tree with the wished treewidth.
///
/// If p > 100 all edges will be removed. The Rng is passed in to increase performance when calling the function multiple times in a row.
///
/// Returns None if k > n
pub fn generate_partial_k_tree_with_guaranteed_treewidth(
    k: usize,
    n: usize,
    p: usize,
    rng: &mut impl Rng,
) -> Option<Graph<i32, i32, Undirected>> {
    loop {
        if let Some(mut graph) = generate_partial_k_tree(k, n, p, rng) {
            if minimum_maximum_degree_heuristic(&graph) == k {
                return Some(graph);
            }
        } else {
            return None;
        }
    }
}

/// Generates a [k-tree](https://en.wikipedia.org/wiki/K-tree) and then randomly removes p percent of the edges
/// to get a [partial k-tree](https://en.wikipedia.org/wiki/Partial_k-tree).
/// If p > 100 all edges will be removed. The Rng is passed in to increase performance when calling the function multiple times in a row.
///
/// Returns None if k > n
pub fn generate_partial_k_tree(
    k: usize,
    n: usize,
    p: usize,
    rng: &mut impl Rng,
) -> Option<Graph<i32, i32, Undirected>> {
    if let Some(mut graph) = generate_k_tree(k, n) {
        // The number of edges in a k-tree
        let number_of_edges = k * (k - 1) / 2 + k * (n - k);

        // Remove p percent of nodes
        for edge_to_be_removed in graph
            .edge_indices()
            .choose_multiple(rng, ((number_of_edges * p) / 100).min(number_of_edges))
        {
            graph.remove_edge(edge_to_be_removed);
        }

        Some(graph)
    } else {
        None
    }
}

/// Generates a [k-tree](https://en.wikipedia.org/wiki/K-tree) with n vertices and k in the definition.
/// Returns None if k > n
fn generate_k_tree(k: usize, n: usize) -> Option<Graph<i32, i32, Undirected>> {
    if k > n {
        None
    } else {
        let mut graph = generate_complete_graph(k);

        // Add the missing n-k vertices
        for _ in k..n {
            let new_vertex = graph.add_node(0);
            for old_vertex_index in graph
                .node_identifiers()
                .choose_multiple(&mut rand::thread_rng(), k)
            {
                graph.add_edge(new_vertex, old_vertex_index, 0);
            }
        }

        Some(graph)
    }
}

fn generate_complete_graph(k: usize) -> Graph<i32, i32, Undirected> {
    let mut graph: Graph<i32, i32, petgraph::prelude::Undirected> =
        petgraph::Graph::new_undirected();

    // Add k nodes to the graph
    let nodes: Vec<NodeIndex> = (0..k + 1).map(|_| graph.add_node(0)).collect();

    // Connect each node to every other node
    for i in 0..k + 1 {
        for j in i + 1..k + 1 {
            graph.add_edge(nodes[i], nodes[j], 0);
        }
    }

    graph
}
