use crate::*;
use petgraph::{Graph, Undirected};

/// Computes an upper bound for the treewidth using the clique graph operator
pub fn compute_treewidth_upper_bound<N: Clone, E: Clone>(graph: &Graph<N, E, Undirected>) -> usize {
    let cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(graph).collect();
    let clique_graph: Graph<_, _, _> = construct_clique_graph(cliques);
    let mut clique_graph_tree: Graph<
        std::collections::HashSet<petgraph::prelude::NodeIndex>,
        i32,
        petgraph::prelude::Undirected,
    > = petgraph::data::FromElements::from_elements(petgraph::algo::min_spanning_tree(
        &clique_graph,
    ));

    fill_bags_along_paths(&mut clique_graph_tree);
    find_width_of_tree_decomposition(&clique_graph_tree)
}

/// Computes an upper bound for the treewidth returning the maximum [compute_treewidth_upper_bound] on the
/// components
pub fn compute_treewidth_upper_bound_not_connected<N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
) -> usize {
    let components = find_connected_components::<Vec<_>, _, _>(graph);
    let mut computed_treewidth: usize = 0;

    for component in components {
        let mut subgraph = graph.clone();
        subgraph.retain_nodes(|_, v| component.contains(&v));

        computed_treewidth = computed_treewidth.max(compute_treewidth_upper_bound(&subgraph));
    }

    computed_treewidth
}
