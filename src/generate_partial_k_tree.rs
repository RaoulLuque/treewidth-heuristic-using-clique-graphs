use log::info;
use petgraph::{graph::NodeIndex, visit::IntoNodeIdentifiers, Graph, Undirected};
use rand::prelude::SliceRandom;
use rand::{seq::IteratorRandom, Rng};

use crate::maximum_minimum_degree_plus;

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
        if let Some(graph) = generate_partial_k_tree(k, n, p, rng) {
            if maximum_minimum_degree_plus(&graph) == k {
                return Some(graph);
            } else {
                info!("Random partial-k-tree graph was just discarded");
            }
        } else {
            return None;
        }
    }
}

/// Generates a [k-tree](https://en.wikipedia.org/wiki/K-tree) and then randomly removes p percent
/// of the edges to get a [partial k-tree](https://en.wikipedia.org/wiki/Partial_k-tree).
/// If p > 100 all edges will be removed. The Rng is passed in to increase performance when calling
/// the function multiple times in a row.
///
/// Returns None if k > n
///
/// The number of edges in a k_tree are k * (k - 1) / 2 + k * (n - k). So the number of removed edges in a
/// partial_k_tree will be (k * (k - 1) / 2 + k * (n - k)) * p / 100 rounded down
pub fn generate_partial_k_tree(
    k: usize,
    n: usize,
    p: usize,
    rng: &mut impl Rng,
) -> Option<Graph<i32, i32, Undirected>> {
    if let Some(mut graph) = generate_k_tree(k, n) {
        // The number of edges in a k-tree
        let number_of_edges = k * (k - 1) / 2 + k * (n - k);
        assert_eq!(number_of_edges, graph.edge_count());
        let number_of_edges_to_be_removed = ((number_of_edges * p) / 100).min(number_of_edges);
        // Remove p percent of nodes
        for edge_to_be_removed in graph
            .edge_indices()
            .choose_multiple(rng, number_of_edges_to_be_removed)
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
pub(crate) fn generate_k_tree(k: usize, n: usize) -> Option<Graph<i32, i32, Undirected>> {
    if k > n {
        None
    } else {
        let mut graph = generate_complete_graph(k);
        let mut potential_cliques: Vec<Vec<_>> = vec![graph.node_identifiers().collect()];

        // Add the missing n-k vertices
        for i in k..n {
            let new_vertex = graph.add_node(i.try_into().unwrap());
            let chosen_k_clique = potential_cliques
                .choose(&mut rand::thread_rng())
                .expect("There should be potential cliques")
                .clone();
            for old_vertex_index in chosen_k_clique.clone() {
                graph.add_edge(new_vertex, old_vertex_index, 0);
                let mut potential_new_clique = chosen_k_clique.clone();
                potential_new_clique.retain(|v| v != &old_vertex_index);
                potential_new_clique.push(new_vertex);
                potential_cliques.push(potential_new_clique);
            }
        }

        Some(graph)
    }
}

/// Generates a [complete graph](https://en.wikipedia.org/wiki/Complete_graph) with k vertices
/// and k * (k-1) / 2 edges
fn generate_complete_graph(k: usize) -> Graph<i32, i32, Undirected> {
    let mut graph: Graph<i32, i32, petgraph::prelude::Undirected> =
        petgraph::Graph::new_undirected();

    // Add k nodes to the graph
    let nodes: Vec<NodeIndex> = (0..k)
        .map(|i| graph.add_node(i.try_into().unwrap()))
        .collect();

    // Connect each node to every other node
    for i in 0..k {
        for j in i + 1..k {
            graph.add_edge(nodes[i], nodes[j], 0);
        }
    }

    graph
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_complete_graph_with_maximum_minimum_degree() {
        let complete_graph_hundred_vertices = generate_complete_graph(100);
        let complete_graph_twenty_vertices = generate_complete_graph(20);

        let max_min_degree_hundred =
            crate::maximum_minimum_degree_plus(&complete_graph_hundred_vertices);
        let max_min_degree_twenty =
            crate::maximum_minimum_degree_plus(&complete_graph_twenty_vertices);

        assert_eq!(max_min_degree_hundred, 99);
        assert_eq!(max_min_degree_twenty, 19);
    }

    #[test]
    fn test_generate_k_tree_with_maximum_minimum_degree() {
        let hundred_tree = generate_k_tree(100, 150).expect("k is smaller than n");
        let twenty_five_tree = generate_k_tree(25, 100).expect("k is smaller than n");

        let max_min_degree_hundred = crate::maximum_minimum_degree_plus(&hundred_tree);
        let max_min_degree_twenty_give = crate::maximum_minimum_degree_plus(&twenty_five_tree);

        assert_eq!(max_min_degree_hundred, 100);
        assert_eq!(max_min_degree_twenty_give, 25);
    }

    #[test]
    fn test_generate_partial_k_tree_with_guarantee_with_maximum_minimum_degree() {
        let mut rng = rand::thread_rng();

        for (k, n, p) in vec![
            (10, 200, 20),
            (10, 500, 20),
            (10, 1000, 20),
            (10, 200, 30),
            (10, 500, 30),
            (10, 1000, 30),
            (10, 200, 40),
            (10, 500, 40),
            (10, 1000, 40),
        ] {
            let tree = generate_partial_k_tree_with_guaranteed_treewidth(k, n, p, &mut rng)
                .expect("k is smaller than n");

            let guaranteed_lower_bound = crate::maximum_minimum_degree_plus(&tree);

            assert_eq!(guaranteed_lower_bound, k);
        }
    }

    #[test]
    fn test_generate_partial_k_tree_with_guarantee_with_high_k() {
        let mut rng = rand::thread_rng();
        let hundred_tree = generate_partial_k_tree_with_guaranteed_treewidth(20, 100, 15, &mut rng)
            .expect("k is smaller than n");
        let twenty_five_tree =
            generate_partial_k_tree_with_guaranteed_treewidth(30, 100, 10, &mut rng)
                .expect("k is smaller than n");

        let max_min_degree_hundred = crate::maximum_minimum_degree_plus(&hundred_tree);
        let max_min_degree_twenty_give = crate::maximum_minimum_degree_plus(&twenty_five_tree);

        assert_eq!(max_min_degree_hundred, 20);
        assert_eq!(max_min_degree_twenty_give, 30);
    }
}
