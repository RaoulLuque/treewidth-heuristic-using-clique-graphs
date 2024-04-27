mod check_tree_decomposition;
mod clique_graph_edge_weight_heuristics;
mod compute_treewidth_upper_bound;
mod construct_clique_graph;
mod fill_bags_along_paths;
mod find_connected_components;
mod find_maximum_cliques;
mod find_path_in_tree;
mod find_width_of_tree_decomposition;
mod generate_partial_k_tree;
mod maximum_minimum_degree_heuristic;

// Imports for using the library
pub use check_tree_decomposition::check_tree_decomposition;
pub use clique_graph_edge_weight_heuristics::*;
pub use compute_treewidth_upper_bound::{
    compute_treewidth_upper_bound, compute_treewidth_upper_bound_not_connected,
};
pub use construct_clique_graph::{construct_clique_graph, construct_clique_graph_with_bags};
pub use fill_bags_along_paths::{fill_bags_along_paths, fill_bags_along_paths_abusing_structure};
pub use find_connected_components::find_connected_components;
pub use find_maximum_cliques::{find_maximum_cliques, find_maximum_cliques_bounded};
pub use find_path_in_tree::find_path_in_tree;
pub use find_width_of_tree_decomposition::find_width_of_tree_decomposition;
pub use generate_partial_k_tree::{
    generate_partial_k_tree, generate_partial_k_tree_with_guaranteed_treewidth,
};
pub use maximum_minimum_degree_heuristic::maximum_minimum_degree;

#[cfg(test)]
pub(crate) mod tests {
    use petgraph::{graph::NodeIndex, Graph};

    /// Struct for TestGraphs with necessary info for testing different functionalities
    ///
    /// Graph is the Graph to be tested
    ///
    /// Treewidth is the correct treewidth of the graph
    ///
    /// Expected max clique is a sorted vector with vectors with NodeIndexes of the expected max cliques
    ///
    /// max_min_degree is the expected result when calculating the maximum minimum degree across all subgraphs
    pub struct TestGraph {
        pub graph: Graph<i32, i32, petgraph::prelude::Undirected>,
        pub treewidth: usize,
        pub expected_max_cliques: Vec<Vec<NodeIndex>>,
        pub max_min_degree: usize,
        pub expected_connected_components: Vec<Vec<NodeIndex>>,
    }

    /// Sets up test graph:
    ///
    /// Test graph 0 has:
    /// 11 vertices, 13 edges, Treewidth 3 and maximum minimum degree 3
    ///
    /// Test graph 1 has:
    /// 6 vertices, 10 edges, Treewidth 3 and maximum minimum degree 3
    ///
    /// Test graph 2 (and higher) has:
    /// 5 vertices, 9 edges, Treewidth 3 and maximum minimum degree 3
    pub fn setup_test_graph(test_graph_number: usize) -> TestGraph {
        match test_graph_number {
            0 => {
                let mut graph: Graph<i32, i32, petgraph::prelude::Undirected> =
                    petgraph::Graph::new_undirected();

                let nodes = [
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
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
                graph.add_edge(nodes[7], nodes[8], 0);
                graph.add_edge(nodes[9], nodes[10], 0);

                let expected_max_cliques: Vec<Vec<_>> = vec![
                    vec![2, 6, 1, 3],
                    vec![2, 6, 4],
                    vec![5, 4, 7],
                    vec![8, 9],
                    vec![10, 11],
                ];
                let mut expected_max_cliques: Vec<Vec<_>> = expected_max_cliques
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_max_cliques.len() {
                    expected_max_cliques[i].sort();
                }
                expected_max_cliques.sort();

                let expected_connected_components =
                    vec![vec![1, 2, 3, 4, 5, 6, 7], vec![8, 9], vec![10, 11]];
                let mut expected_connected_components: Vec<Vec<_>> = expected_connected_components
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_connected_components.len() {
                    expected_connected_components[i].sort();
                }
                expected_connected_components.sort();

                TestGraph {
                    graph,
                    treewidth: 3,
                    expected_max_cliques,
                    max_min_degree: 3,
                    expected_connected_components,
                }
            }
            1 => {
                let mut graph: Graph<i32, i32, petgraph::prelude::Undirected> =
                    petgraph::Graph::new_undirected();

                let nodes = [
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                ];

                graph.add_edge(nodes[0], nodes[1], 0);
                graph.add_edge(nodes[0], nodes[3], 0);
                graph.add_edge(nodes[0], nodes[4], 0);
                graph.add_edge(nodes[0], nodes[5], 0);
                graph.add_edge(nodes[1], nodes[2], 0);
                graph.add_edge(nodes[2], nodes[3], 0);
                graph.add_edge(nodes[2], nodes[5], 0);
                graph.add_edge(nodes[3], nodes[4], 0);
                graph.add_edge(nodes[3], nodes[5], 0);
                graph.add_edge(nodes[4], nodes[5], 0);

                let expected_max_cliques: Vec<Vec<_>> =
                    vec![vec![1, 2], vec![1, 4, 5, 6], vec![2, 3], vec![3, 4, 6]];
                let mut expected_max_cliques: Vec<Vec<_>> = expected_max_cliques
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_max_cliques.len() {
                    expected_max_cliques[i].sort();
                }
                expected_max_cliques.sort();

                let expected_connected_components = vec![vec![1, 2, 3, 4, 5, 6]];
                let mut expected_connected_components: Vec<Vec<_>> = expected_connected_components
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_connected_components.len() {
                    expected_connected_components[i].sort();
                }
                expected_connected_components.sort();

                TestGraph {
                    graph,
                    treewidth: 3,
                    expected_max_cliques,
                    max_min_degree: 3,
                    expected_connected_components,
                }
            }
            _ => {
                let mut graph: Graph<i32, i32, petgraph::prelude::Undirected> =
                    petgraph::Graph::new_undirected();

                let nodes = [
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                    graph.add_node(0),
                ];

                graph.add_edge(nodes[0], nodes[1], 0);
                graph.add_edge(nodes[0], nodes[2], 0);
                graph.add_edge(nodes[0], nodes[3], 0);
                graph.add_edge(nodes[1], nodes[2], 0);
                graph.add_edge(nodes[1], nodes[3], 0);
                graph.add_edge(nodes[1], nodes[4], 0);
                graph.add_edge(nodes[2], nodes[3], 0);
                graph.add_edge(nodes[2], nodes[4], 0);
                graph.add_edge(nodes[3], nodes[4], 0);

                let expected_max_cliques: Vec<Vec<_>> = vec![vec![1, 2, 3, 4], vec![2, 3, 4, 5]];
                let mut expected_max_cliques: Vec<Vec<_>> = expected_max_cliques
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_max_cliques.len() {
                    expected_max_cliques[i].sort();
                }
                expected_max_cliques.sort();

                let expected_connected_components = vec![vec![1, 2, 3, 4, 5]];
                let mut expected_connected_components: Vec<Vec<_>> = expected_connected_components
                    .into_iter()
                    .map(|v| {
                        v.into_iter()
                            .map(|e| petgraph::graph::node_index(e - 1))
                            .collect::<Vec<_>>()
                    })
                    .collect();
                for i in 0..expected_connected_components.len() {
                    expected_connected_components[i].sort();
                }
                expected_connected_components.sort();

                TestGraph {
                    graph,
                    treewidth: 3,
                    expected_max_cliques,
                    max_min_degree: 3,
                    expected_connected_components,
                }
            }
        }
    }
}
