use std::collections::HashSet;

use itertools::Itertools;
use petgraph::{graph::NodeIndex, visit::IntoNodeIdentifiers, Graph, Undirected};

/// Computes the contraction degeneracy of the given graph according to https://link.springer.com/chapter/10.1007/978-3-540-30140-0_56 (see MMD+: least-c)
pub fn maximum_minimum_degree_plus<N: Clone + Default, E: Clone + Default>(
    graph: &Graph<N, E, Undirected>,
) -> usize {
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

        let min_degree_vertex_neighbours = graph_copy
            .neighbors(min_degree_vertex)
            .collect::<HashSet<_>>();

        let least_common_neighbours_neighbour = min_degree_vertex_neighbours
            .iter()
            .min_by_key(|id| {
                if id == &&min_degree_vertex {
                    graph_copy.node_count() + 1
                } else {
                    graph_copy
                        .neighbors(**id)
                        .collect::<HashSet<_>>()
                        .intersection(&min_degree_vertex_neighbours)
                        .collect_vec()
                        .len()
                }
            })
            .expect("Graph should have at least 2 nodes");

        contract_edge(
            &mut graph_copy,
            min_degree_vertex,
            *least_common_neighbours_neighbour,
        );
    }

    max_min
}

/// Contracts the edge between vertex one and vertex two. If no edge exists, nothing happens
fn contract_edge<N: Clone + Default, E: Clone + Default>(
    graph: &mut Graph<N, E, Undirected>,
    vertex_one: NodeIndex,
    vertex_two: NodeIndex,
) -> () {
    if graph.contains_edge(vertex_one, vertex_two) {
        let new_vertex = graph.add_node(N::default());
        let mut edges_to_add: HashSet<_> = HashSet::new();

        for neighbour in graph.neighbors(vertex_one) {
            edges_to_add.insert(neighbour);
        }
        for neighbour in graph.neighbors(vertex_two) {
            edges_to_add.insert(neighbour);
        }

        for neighbour_to_add in edges_to_add {
            graph.add_edge(new_vertex, neighbour_to_add, E::default());
        }

        graph.remove_node(vertex_one);
        graph.remove_node(vertex_two);
    }
}
