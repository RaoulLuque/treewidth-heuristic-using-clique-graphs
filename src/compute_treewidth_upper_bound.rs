use petgraph::{graph::NodeIndex, Graph, Undirected};
use std::{collections::HashSet, fmt::Debug, hash::BuildHasher};

use crate::*;
use construct_clique_graph::*;
use fill_bags_along_paths::*;
use find_maximal_cliques::*;
use find_width_of_tree_decomposition::find_width_of_tree_decomposition;

/// Different methods for computing the spanning tree of the clique graph that is used as the base
/// of the tree decomposition.
///
/// MSTre Constructs a minimum spanning tree of the clique graph and fills up the bags
/// afterwards
///
/// MSTreIUseTr Constructs a minimum spanning tree of the clique graph and fills up the
/// bags afterwards trying to speed up filling up by using the tree structure
///
/// FilWh Fills bags while constructing a spanning tree minimizing according to the edge
/// heuristic
///
/// FilWhILogBagSize Does the same computation as FillWhilstMST however tracks the size of the
/// biggest bag every time a new vertex is added to the current spanning tree. The file
/// k-tree-benchmarks/benchmark_results/k_tree_maximum_bag_size_over_time.csv (where k-tree-benchmarks
/// is a subdirectory of the runtime directory) otherwise this option will panic.
///
/// FWhUE Fill bags while constructing a spanning tree minimizing according to
/// the edge heuristic. Updating adjacencies in clique graph according to bag updates
///
/// FilWhIUseTr Fill bags while constructing a spanning tree minimizing according to the
/// edge heuristic trying to speed up filling up by using the tree structure
///
/// FWBag Fills bags while constructing a spanning tree of the clique graph trying to minimize the maximum bag size in each step
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SpanningTreeConstructionMethod {
    MSTre,
    MSTreIUseTr,
    FilWh,
    FilWhILogBagSize,
    FWhUE,
    FilWhIUseTr,
    FWBag,
}

/// Computes an upper bound for the treewidth using the clique graph operator.
///
/// Does this by computing the clique graph of the given graph and then constructing a spanning
/// tree on the constructed clique graph. Then the bags are filled up to satisfy the properties of
/// a tree decomposition.
///
/// See [SpanningTreeConstructionMethod] for the different options of spanning tree construction.
///
/// Also see [edge weight functions][crate::clique_graph_edge_weight_functions] for the different
/// weight options for the edges in the clique graph.
///
/// It is possible to not use the clique graph but the clique graph with a bound on the
/// size of the cliques instead. The resulting graph is the intersection graph of the set of all
/// cliques that are maximal or have a size of clique_bound. For further information on this read the
/// documentation of [find_maximal_cliques_bounded].
///
/// Can also check the tree decomposition for correctness after computation which will on average at least double
/// the running time. If so, will panic if the tree decomposition is incorrect returning the vertices
/// and path that is faulty.
pub fn compute_treewidth_upper_bound<
    N: Clone,
    E: Clone,
    O: Clone + Ord + Default + Debug,
    S: Default + BuildHasher + Clone,
>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_function: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    treewidth_computation_method: SpanningTreeConstructionMethod,
    check_tree_decomposition_bool: bool,
    clique_bound: Option<i32>,
) -> usize {
    // Find cliques in initial graph
    let cliques: Vec<Vec<_>> = if let Some(k) = clique_bound {
        find_maximal_cliques_bounded::<Vec<_>, _, S>(graph, k)
            // .sorted()
            .collect()
    } else {
        find_maximal_cliques::<Vec<_>, _, S>(graph)
            // .sorted()
            .collect()
    };

    let (clique_graph_tree_after_filling_up, clique_graph_map, predecessor_map) =
        match treewidth_computation_method {
            SpanningTreeConstructionMethod::MSTre => {
                let clique_graph: Graph<_, _, _> =
                    construct_clique_graph(cliques, edge_weight_function);

                let mut clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
                    &clique_graph,
                ));

                fill_bags_along_paths(&mut clique_graph_tree);

                (clique_graph_tree, None, None)
            }
            SpanningTreeConstructionMethod::MSTreIUseTr => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let mut clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
                    &clique_graph,
                ));

                let predecessor_map = fill_bags_along_paths_using_structure(
                    &mut clique_graph_tree,
                    &clique_graph_map,
                );

                (
                    clique_graph_tree,
                    Some(clique_graph_map),
                    Some(predecessor_map),
                )
            }
            SpanningTreeConstructionMethod::FilWh => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = fill_bags_while_generating_mst::<N, E, O, S>(
                    &clique_graph,
                    edge_weight_function,
                    clique_graph_map,
                    false,
                );

                (clique_graph_tree, None, None)
            }
            SpanningTreeConstructionMethod::FilWhILogBagSize => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = fill_bags_while_generating_mst::<N, E, O, S>(
                    &clique_graph,
                    edge_weight_function,
                    clique_graph_map,
                    true,
                );

                (clique_graph_tree, None, None)
            }
            SpanningTreeConstructionMethod::FWhUE => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = fill_bags_while_generating_mst_update_edges::<N, E, O, S>(
                    &clique_graph,
                    edge_weight_function,
                    clique_graph_map,
                );

                (clique_graph_tree, None, None)
            }
            SpanningTreeConstructionMethod::FilWhIUseTr => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = fill_bags_while_generating_mst_using_tree::<N, E, O, S>(
                    &clique_graph,
                    edge_weight_function,
                    clique_graph_map,
                );

                (clique_graph_tree, None, None)
            }
            SpanningTreeConstructionMethod::FWBag => {
                let (clique_graph, clique_graph_map) =
                    construct_clique_graph_with_bags(cliques, edge_weight_function);

                let clique_graph_tree: Graph<
                    std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
                    O,
                    petgraph::prelude::Undirected,
                > = fill_bags_while_generating_mst_least_bag_size::<N, E, O, S>(
                    &clique_graph,
                    clique_graph_map,
                );

                (clique_graph_tree, None, None)
            }
        };

    if check_tree_decomposition_bool {
        assert!(
            check_tree_decomposition(
                &graph,
                &clique_graph_tree_after_filling_up,
                &predecessor_map,
                &clique_graph_map
            ),
            "Tree decomposition is invalid. See previous print statements for reason."
        );
    }
    let treewidth = find_width_of_tree_decomposition(&clique_graph_tree_after_filling_up);

    treewidth
}

/// Computes an upper bound for the treewidth returning the maximum [compute_treewidth_upper_bound] on the
/// components
pub fn compute_treewidth_upper_bound_not_connected<
    N: Clone + Debug,
    E: Clone + Debug,
    O: Clone + Ord + Default + Debug,
    S: Default + BuildHasher + Clone,
>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_function: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    treewidth_computation_method: SpanningTreeConstructionMethod,
    check_tree_decomposition_bool: bool,
    clique_bound: Option<i32>,
) -> usize {
    let components = find_connected_components::<Vec<_>, _, _, S>(graph);
    let mut computed_treewidth: usize = 0;

    for component in components {
        println!("Test");
        let mut subgraph = graph.clone();
        subgraph.retain_nodes(|_, v| component.contains(&v));

        println!("Graph: {:?} \n Subgraph: {:?}", graph, subgraph);

        computed_treewidth = computed_treewidth.max(compute_treewidth_upper_bound(
            &subgraph,
            edge_weight_function,
            treewidth_computation_method,
            check_tree_decomposition_bool,
            clique_bound,
        ));
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
            let _ = compute_treewidth_upper_bound_not_connected::<_, _, _, RandomState>(
                &test_graph.graph,
                constant,
                SpanningTreeConstructionMethod::MSTreIUseTr,
                true,
                None,
            );

            let _ = compute_treewidth_upper_bound_not_connected::<_, _, _, RandomState>(
                &test_graph.graph,
                constant,
                SpanningTreeConstructionMethod::MSTre,
                true,
                None,
            );
        }
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_neutral_weight_heuristic() {
        for i in 0..3 {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth =
                    compute_treewidth_upper_bound_not_connected::<
                        _,
                        _,
                        _,
                        std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                    >(
                        &test_graph.graph, constant, computation_method, false, None
                    );
                if !(i == 1
                    && (computation_method == SpanningTreeConstructionMethod::MSTre
                        || computation_method == SpanningTreeConstructionMethod::MSTreIUseTr))
                {
                    if i == 1 && computation_method == SpanningTreeConstructionMethod::FilWh {
                        assert_eq!(computed_treewidth, 4);
                    } else {
                        assert_eq!(
                            computed_treewidth, test_graph.treewidth,
                            "Test graph number {} failed with computation method {:?}",
                            i, computation_method
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_negative_intersection_weight_heuristic() {
        for i in vec![0, 2] {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                    _,
                    _,
                    _,
                    std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                >(
                    &test_graph.graph,
                    negative_intersection,
                    computation_method,
                    true,
                    None,
                );
                if !(i == 1
                    && (computation_method == SpanningTreeConstructionMethod::MSTre
                        || computation_method == SpanningTreeConstructionMethod::MSTreIUseTr))
                {
                    assert_eq!(
                        computed_treewidth, test_graph.treewidth,
                        "computation method: {:?}. Test graph {:?}",
                        computation_method, i
                    );
                }
            }
        }
    }

    #[test]
    fn negative_intersection_weight_heuristic_does_not_fail_on_first_test_graph() {
        let i = 1;
        let computation_method = SpanningTreeConstructionMethod::MSTreIUseTr;

        let test_graph = setup_test_graph(i);
        let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
            _,
            _,
            _,
            std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
        >(
            &test_graph.graph,
            negative_intersection,
            computation_method,
            true,
            None,
        );
        assert_eq!(
            computed_treewidth, test_graph.treewidth,
            "computation method: {:?}. Test graph {:?}",
            computation_method, i
        );
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_least_difference_weight_heuristic() {
        for i in 0..3 {
            for computation_method in COMPUTATION_METHODS {
                let test_graph = setup_test_graph(i);
                let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                    _,
                    _,
                    _,
                    std::hash::BuildHasherDefault<rustc_hash::FxHasher>,
                >(
                    &test_graph.graph,
                    least_difference,
                    computation_method,
                    false,
                    None,
                );
                assert_eq!(computed_treewidth, test_graph.treewidth);
            }
        }
    }

    #[test]
    fn test_treewidth_heuristic_does_not_panic() {
        let graph =
            petgraph::graph::UnGraph::<i32, ()>::from_edges(&[(0, 1), (1, 2), (2, 3), (3, 0)]);

        let treewidth_upper_bound = compute_treewidth_upper_bound::<_, _, _, std::hash::RandomState>(
            &graph,
            negative_intersection,
            SpanningTreeConstructionMethod::FilWh,
            false,
            None,
        );

        assert_eq!(treewidth_upper_bound, 2);
    }
}
