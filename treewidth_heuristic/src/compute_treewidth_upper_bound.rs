use std::collections::HashSet;

use crate::*;
use log::info;
use petgraph::{graph::NodeIndex, Graph, Undirected};

/// Computes an upper bound for the treewidth using the clique graph operator
pub fn compute_treewidth_upper_bound<N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
) -> usize {
    info!("Finding maximum cliques");
    let cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(graph).collect();
    info!("Computing clique graph");
    let (clique_graph, clique_graph_map) = construct_clique_graph_with_bags(cliques);
    // let clique_graph: Graph<_, _, _> = construct_clique_graph(cliques, edge_weight_heuristic);
    info!("Computing min spanning tree");
    let mut clique_graph_tree: Graph<
        std::collections::HashSet<petgraph::prelude::NodeIndex>,
        i32,
        petgraph::prelude::Undirected,
    > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
        &clique_graph,
    ));
    info!("Filling bags to get tree decomposition");
    
    let predecessor_map =
        fill_bags_along_paths_abusing_structure(&mut clique_graph_tree, &clique_graph_map);
    // fill_bags_along_paths(&mut clique_graph_tree);

    assert!(check_tree_decomposition(
        &clique_graph_tree,
        &predecessor_map,
        &clique_graph_map
    ));
    find_width_of_tree_decomposition(&clique_graph_tree)
}

/// Computes an upper bound for the treewidth returning the maximum [compute_treewidth_upper_bound] on the
/// components
pub fn compute_treewidth_upper_bound_not_connected<N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex>, &HashSet<NodeIndex>) -> i32,
) -> usize {
    let components = find_connected_components::<Vec<_>, _, _>(graph);
    let mut computed_treewidth: usize = 0;

    for component in components {
        let mut subgraph = graph.clone();
        subgraph.retain_nodes(|_, v| component.contains(&v));

        computed_treewidth = computed_treewidth.max(compute_treewidth_upper_bound(
            &subgraph,
            edge_weight_heuristic,
        ));
    }

    computed_treewidth
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    #[test]
    fn test_treewidth_heuristic_on_test_graph_two() {
        let test_graph = setup_test_graph_two();
        let computed_treewidth =
            compute_treewidth_upper_bound_not_connected(&test_graph.graph, neutral_heuristic);
        // TO DO: Write heuristic that "fixes" the computed treewidth in this  case
        // assert_eq!(computed_treewidth, test_graph.treewidth + 1);
    }

    #[test]
    fn test_treewidth_heuristic_on_test_graph_three() {
        let test_graph = setup_test_graph_three();
        let computed_treewidth =
            compute_treewidth_upper_bound_not_connected(&test_graph.graph, neutral_heuristic);

        assert_eq!(computed_treewidth, test_graph.treewidth);
    }

    #[test]
    fn test_treewidth_heuristic_on_test_graph_one() {
        let test_graph = setup_test_graph_one();
        let computed_treewidth =
            compute_treewidth_upper_bound_not_connected(&test_graph.graph, neutral_heuristic);

        assert_eq!(computed_treewidth, test_graph.treewidth);
    }
}
