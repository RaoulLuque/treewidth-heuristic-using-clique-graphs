use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::BuildHasher,
};

use crate::*;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, Graph, Undirected};

#[derive(Clone, Copy, Debug)]
pub enum TreewidthComputationMethod {
    MSTAndFill,
    MSTAndUseTreeStructure,
    FillWhilstMST,
}

/// Computes an upper bound for the treewidth using the clique graph operator.
///
/// Can either use the tree structure in the spanning tree to fill up the bags of vertices
/// on paths between vertices in the clique graph or calculate paths for each individual pair
/// of vertices with intersecting bags.
///
/// Can also check the tree decomposition for correctness after computation which will up to double
/// the running time. If so, will panic if the tree decomposition if incorrect returning the vertices
/// and path that is faulty.
pub fn compute_treewidth_upper_bound<
    N: Clone,
    E: Clone,
    O: Clone + Ord + Default + Debug,
    S: Default + BuildHasher + Clone,
>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    treewidth_computation_method: TreewidthComputationMethod,
    check_tree_decomposition_bool: bool,
) -> (
    Graph<HashSet<NodeIndex, S>, O, Undirected>,
    Graph<HashSet<NodeIndex, S>, O, Undirected>,
    Option<Graph<HashSet<NodeIndex, S>, O, Undirected>>,
    Option<HashMap<NodeIndex, (NodeIndex, usize), S>>,
    Option<HashMap<NodeIndex, HashSet<NodeIndex, S>, S>>,
    usize,
) {
    // Find cliques in initial graph
    let cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _, S>(graph)
        .sorted()
        .collect();

    let (
        clique_graph_tree_after_filling_up,
        clique_graph_map,
        predecessor_map,
        clique_graph_tree_before_filling,
        clique_graph,
    ) = match treewidth_computation_method {
        TreewidthComputationMethod::MSTAndFill => {
            let clique_graph: Graph<_, _, _> =
                construct_clique_graph(cliques, edge_weight_heuristic);

            let mut clique_graph_tree: Graph<
                std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                O,
                petgraph::prelude::Undirected,
            > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
                &clique_graph,
            ));
            let clique_graph_tree_before_filling = clique_graph_tree.clone();

            fill_bags_along_paths(&mut clique_graph_tree);

            (
                clique_graph_tree,
                None,
                None,
                Some(clique_graph_tree_before_filling),
                clique_graph,
            )
        }
        TreewidthComputationMethod::MSTAndUseTreeStructure => {
            let (clique_graph, clique_graph_map) =
                construct_clique_graph_with_bags(cliques, edge_weight_heuristic);
            // DEBUG
            // println!("Initial clique graph: {:?}", clique_graph);

            let mut clique_graph_tree: Graph<
                std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                O,
                petgraph::prelude::Undirected,
            > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
                &clique_graph,
            ));
            let clique_graph_tree_before_filling = clique_graph_tree.clone();

            // DEBUG
            let clique_graph_tree_copy: Graph<
                std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                O,
                petgraph::prelude::Undirected,
            > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
                &clique_graph,
            ));
            assert!(petgraph::algo::is_isomorphic_matching(
                &clique_graph_tree,
                &clique_graph_tree_copy,
                |a, b| a.eq(b),
                |a, b| a.eq(b)
            ));

            let predecessor_map =
                fill_bags_along_paths_using_structure(&mut clique_graph_tree, &clique_graph_map);
            // DEBUG
            // println!(
            //     "Clique graph tree after filling up: {:?} \n \n",
            //     clique_graph_tree
            // );

            (
                clique_graph_tree,
                Some(clique_graph_map),
                Some(predecessor_map),
                Some(clique_graph_tree_before_filling),
                clique_graph,
            )
        }
        TreewidthComputationMethod::FillWhilstMST => {
            let (clique_graph, clique_graph_map) =
                construct_clique_graph_with_bags(cliques, edge_weight_heuristic);

            let clique_graph_tree: Graph<
                std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                O,
                petgraph::prelude::Undirected,
            > = fill_bags_while_generating_mst::<N, E, O, S>(
                &clique_graph,
                edge_weight_heuristic,
                clique_graph_map,
            );

            (clique_graph_tree, None, None, None, clique_graph)
        }
    };

    if check_tree_decomposition_bool {
        assert!(check_tree_decomposition(
            &graph,
            &clique_graph_tree_after_filling_up,
            &predecessor_map,
            &clique_graph_map
        ));
    }
    let treewidth = find_width_of_tree_decomposition(&clique_graph_tree_after_filling_up);

    (
        clique_graph,
        clique_graph_tree_after_filling_up,
        clique_graph_tree_before_filling,
        predecessor_map,
        clique_graph_map,
        treewidth,
    )
}

/// Computes an upper bound for the treewidth returning the maximum [compute_treewidth_upper_bound] on the
/// components
pub fn compute_treewidth_upper_bound_not_connected<
    N: Clone,
    E: Clone,
    S: Default + BuildHasher + Clone,
    O: Clone + Ord + Default + Debug,
>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    treewidth_computation_method: TreewidthComputationMethod,
    check_tree_decomposition_bool: bool,
) -> usize {
    let components = find_connected_components::<Vec<_>, _, _, S>(graph);
    let mut computed_treewidth: usize = 0;

    for component in components {
        let mut subgraph = graph.clone();
        subgraph.retain_nodes(|_, v| component.contains(&v));

        computed_treewidth = computed_treewidth.max(
            compute_treewidth_upper_bound(
                &subgraph,
                edge_weight_heuristic,
                treewidth_computation_method,
                check_tree_decomposition_bool,
            )
            .5,
        );
    }

    computed_treewidth
}

#[cfg(test)]
mod tests {
    use std::hash::RandomState;

    use super::*;
    use crate::tests::*;

    #[test]
    fn test_treewidth_heuristic_check_tree_decomposition() {
        for i in 0..3 {
            let test_graph = setup_test_graph(i);
            let _ = compute_treewidth_upper_bound_not_connected::<_, _, RandomState, _>(
                &test_graph.graph,
                neutral_heuristic,
                TreewidthComputationMethod::MSTAndUseTreeStructure,
                true,
            );

            let _ = compute_treewidth_upper_bound_not_connected::<_, _, RandomState, _>(
                &test_graph.graph,
                neutral_heuristic,
                TreewidthComputationMethod::MSTAndFill,
                true,
            );
        }
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_neutral_weight_heuristic() {
        for i in 0..3 {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                    _,
                    _,
                    std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                    _,
                >(
                    &test_graph.graph,
                    neutral_heuristic,
                    computation_method,
                    false,
                );
                assert_eq!(computed_treewidth, test_graph.treewidth);
            }
        }
    }

    #[test]
    #[should_panic]
    fn test_treewidth_heuristic_and_check_result_negative_intersection_weight_heuristic() {
        for i in 0..3 {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                    _,
                    _,
                    std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                    _,
                >(
                    &test_graph.graph,
                    negative_intersection_heuristic,
                    computation_method,
                    false,
                );
                assert_eq!(
                    computed_treewidth,
                    test_graph.treewidth,
                    "computation method: {:?}. Test graph {:?}",
                    computation_method,
                    i + 1
                );
            }
        }
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_least_difference_weight_heuristic() {
        for i in 0..3 {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                    _,
                    _,
                    std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                    _,
                >(
                    &test_graph.graph,
                    least_difference_heuristic,
                    computation_method,
                    false,
                );
                assert_eq!(computed_treewidth, test_graph.treewidth);
            }
        }
    }
}
