mod check_tree_decomposition;
mod clique_graph_edge_weight_heuristics;
mod compute_treewidth_upper_bound;
mod construct_clique_graph;
mod fill_bags_along_paths;
mod fill_bags_while_generating_mst;
mod find_connected_components;
mod find_maximum_cliques;
mod find_path_in_tree;
mod find_width_of_tree_decomposition;
mod generate_partial_k_tree;
mod maximum_minimum_degree_heuristic;

// Imports for using the library
pub(crate) use check_tree_decomposition::check_tree_decomposition;
pub use clique_graph_edge_weight_heuristics::*;
pub use compute_treewidth_upper_bound::{
    compute_treewidth_upper_bound, compute_treewidth_upper_bound_not_connected,
    TreewidthComputationMethod,
};
pub(crate) use construct_clique_graph::{construct_clique_graph, construct_clique_graph_with_bags};
pub(crate) use fill_bags_along_paths::{
    fill_bags_along_paths, fill_bags_along_paths_using_structure,
};
pub(crate) use fill_bags_while_generating_mst::{
    fill_bags_while_generating_mst, fill_bags_while_generating_mst_using_tree,
};
pub(crate) use find_connected_components::find_connected_components;
pub(crate) use find_maximum_cliques::{find_maximum_cliques, find_maximum_cliques_bounded};
pub(crate) use find_width_of_tree_decomposition::find_width_of_tree_decomposition;
pub use generate_partial_k_tree::{
    generate_partial_k_tree, generate_partial_k_tree_with_guaranteed_treewidth,
};
pub(crate) use maximum_minimum_degree_heuristic::maximum_minimum_degree;

// Debug version
#[cfg(debug_assertions)]
macro_rules! hashset {
    () => {{
        let tmp: std::collections::HashSet<_, std::hash::BuildHasherDefault<rustc_hash::FxHasher>> =
            Default::default();
        tmp
    }};
}

// Non-debug version
#[cfg(not(debug_assertions))]
macro_rules! hashset {
    () => {
        std::collections::HashSet::new()
    };
}
pub(crate) use hashset;

#[cfg(test)]
pub(crate) mod tests {
    use petgraph::{graph::NodeIndex, Graph};

    use super::*;

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

    pub const COMPUTATION_METHODS: [TreewidthComputationMethod; 3] = [
        TreewidthComputationMethod::FillWhilstMST,
        TreewidthComputationMethod::MSTAndFill,
        TreewidthComputationMethod::MSTAndUseTreeStructure,
    ];

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

    #[test]
    fn hash_test() {
        let mut test = true;
        for _ in 0..100 {
            let a = (0..100).zip(100..200);
            let mut set_one = hashset![];

            let mut set_two = hashset![];

            for entry in a {
                set_one.insert(entry);
                set_two.insert(entry);
            }

            if !set_one.into_iter().eq(set_two) {
                test = false;
            }
        }

        debug_assert!(test);
    }

    fn test_graph_on_all_heuristics<N: Clone, E: Clone>(
        graph: Graph<N, E, petgraph::prelude::Undirected>,
        expected_treewidth: usize,
        msg: &str,
    ) {
        for computation_method in COMPUTATION_METHODS {
            let treewidth = compute_treewidth_upper_bound_not_connected(
                &graph,
                negative_intersection_heuristic::<std::hash::RandomState>,
                computation_method,
                true,
                None,
            );
            assert_eq!(treewidth, expected_treewidth, "{}", msg);

            let treewidth = compute_treewidth_upper_bound_not_connected(
                &graph,
                least_difference_heuristic::<std::hash::RandomState>,
                computation_method,
                true,
                None,
            );
            assert_eq!(
                treewidth, expected_treewidth,
                "{} computation method: {:?}",
                msg, computation_method
            );
        }
    }

    #[test]
    fn test_heuristic_on_k_tree() {
        use crate::generate_partial_k_tree::generate_k_tree;
        use rand::Rng;

        for _ in 0..25 {
            let mut rng = rand::thread_rng();

            let k: usize = (rng.gen::<f32>() * 50.0) as usize;
            // n should be strictly greater than k otherwise k_tree has not guaranteed treewidth k
            let n: usize = (rng.gen::<f32>() * 100.0) as usize + k + 1;

            let k_tree: Graph<i32, i32, petgraph::prelude::Undirected> =
                generate_k_tree(k, n).expect("k should be smaller or eq to n");

            test_graph_on_all_heuristics(k_tree, k, &format!("k_tree with n: {} and k: {}", n, k));
        }
    }
}
