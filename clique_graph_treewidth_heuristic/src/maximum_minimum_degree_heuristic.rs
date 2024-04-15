use petgraph::{visit::IntoNodeIdentifiers, Graph, Undirected};

/// Calculates the maximum minimum degree across the given Graph and all it's subgraphs.
pub fn maximum_minimum_degree<N: Clone, E: Clone>(graph: &Graph<N, E, Undirected>) -> usize {
    let mut max_min = 0;
    let mut graph_copy = graph.clone();

    while graph_copy.node_count() >= 2 {
        let min_degree_vertex = graph_copy
            .node_identifiers()
            .min_by_key(|id| graph_copy.neighbors(*id).collect::<Vec<_>>().len())
            .expect("Graph should have at least 2 nodes");

        max_min = max_min.max(
            graph_copy
                .neighbors(min_degree_vertex)
                .collect::<Vec<_>>()
                .len(),
        );
        graph_copy.remove_node(min_degree_vertex);
    }

    max_min
}
