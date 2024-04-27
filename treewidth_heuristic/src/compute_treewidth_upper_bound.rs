use std::collections::HashSet;

use crate::*;
use petgraph::{graph::NodeIndex, Graph, Undirected};

/// Computes an upper bound for the treewidth using the clique graph operator.
///
/// Can either use the tree structure in the spanning tree to fill up the bags of vertices
/// on paths between vertices in the clique graph or calculate paths for each individual pair
/// of vertices with intersecting bags.
///
/// Can also check the tree decomposition for correctness after computation which will up to double
/// the running time. If so, will panic if the tree decomposition if incorrect returning the vertices
/// and path that is faulty.
pub fn compute_treewidth_upper_bound<N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
    use_predecessor_map_to_fill_bags: bool,
    check_tree_decomposition_bool: bool,
) -> usize {
    // Find cliques in initial graph
    let cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(graph).collect();

    let (clique_graph_tree, clique_graph_map, predecessor_map) = if use_predecessor_map_to_fill_bags
    {
        let (clique_graph, clique_graph_map) =
            construct_clique_graph_with_bags(cliques, edge_weight_heuristic);
        // DEBUG
        println!("Initial clique graph: {:?}", clique_graph);

        let mut clique_graph_tree: Graph<
            std::collections::HashSet<petgraph::prelude::NodeIndex>,
            i32,
            petgraph::prelude::Undirected,
        > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
            &clique_graph,
        ));

        let predecessor_map =
            fill_bags_along_paths_abusing_structure(&mut clique_graph_tree, &clique_graph_map);
        // DEBUG
        println!(
            "Clique graph tree after filling up: {:?} \n \n",
            clique_graph_tree
        );

        (
            clique_graph_tree,
            Some(clique_graph_map),
            Some(predecessor_map),
        )
    } else {
        let clique_graph: Graph<_, _, _> = construct_clique_graph(cliques, edge_weight_heuristic);

        let mut clique_graph_tree: Graph<
            std::collections::HashSet<petgraph::prelude::NodeIndex>,
            i32,
            petgraph::prelude::Undirected,
        > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
            &clique_graph,
        ));

        fill_bags_along_paths(&mut clique_graph_tree);

        (clique_graph_tree, None, None)
    };
    if check_tree_decomposition_bool {
        assert!(check_tree_decomposition(
            &clique_graph_tree,
            predecessor_map,
            clique_graph_map
        ));
    }

    find_width_of_tree_decomposition(&clique_graph_tree)
}

/// Computes an upper bound for the treewidth returning the maximum [compute_treewidth_upper_bound] on the
/// components
pub fn compute_treewidth_upper_bound_not_connected<N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
    use_predecessor_map_to_fill_bags: bool,
    check_tree_decomposition_bool: bool,
) -> usize {
    let components = find_connected_components::<Vec<_>, _, _>(graph);
    let mut computed_treewidth: usize = 0;

    for component in components {
        let mut subgraph = graph.clone();
        subgraph.retain_nodes(|_, v| component.contains(&v));

        computed_treewidth = computed_treewidth.max(compute_treewidth_upper_bound(
            &subgraph,
            edge_weight_heuristic,
            use_predecessor_map_to_fill_bags,
            check_tree_decomposition_bool,
        ));
    }

    computed_treewidth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn test_treewidth_heuristic_check_tree_decomposition() {
        for i in 0..3 {
            let test_graph = setup_test_graph(i);
            let _ = compute_treewidth_upper_bound_not_connected(
                &test_graph.graph,
                neutral_heuristic,
                true,
                true,
            );

            let _ = compute_treewidth_upper_bound_not_connected(
                &test_graph.graph,
                neutral_heuristic,
                false,
                true,
            );
        }
    }

    #[test]
    fn test_treewidth_heuristic_and_check_result_neutral_weight_heuristic() {
        for i in vec![0, 2] {
            let test_graph = setup_test_graph(i);
            let computed_treewidth = compute_treewidth_upper_bound_not_connected(
                &test_graph.graph,
                neutral_heuristic,
                true,
                false,
            );
            assert_eq!(computed_treewidth, test_graph.treewidth);

            let computed_treewidth = compute_treewidth_upper_bound_not_connected(
                &test_graph.graph,
                neutral_heuristic,
                false,
                false,
            );
            assert_eq!(computed_treewidth, test_graph.treewidth);
        }
    }
}
