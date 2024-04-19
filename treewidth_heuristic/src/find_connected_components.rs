use petgraph::graph::NodeIndex;
use petgraph::visit::{EdgeCount, IntoNeighbors, IntoNodeIdentifiers};
use petgraph::{Graph, Undirected};
use std::iter::from_fn;
use std::{collections::HashSet, hash::Hash};

/// Returns the connected components of a graph
///
/// Uses breadth first search starting at vertices to find components
///
/// Adapted from [networkx connected_components](https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.components.connected_components.html)
pub fn find_connected_components<TargetColl, N: Clone, E: Clone>(
    graph: &Graph<N, E, Undirected>,
) -> impl Iterator<Item = TargetColl> + '_
where
    TargetColl: FromIterator<NodeIndex>,
{
    let mut seen_vertices: HashSet<NodeIndex> = HashSet::new();

    from_fn(move || {
        for vertex in graph.node_identifiers() {
            if !seen_vertices.contains(&vertex) {
                let component = breadth_first_search(&graph, vertex);
                seen_vertices.extend(component.iter().cloned());
                return Some(component.into_iter().collect::<TargetColl>());
            }
        }
        None
    })
}

/// Breadth first search implemented iteratively using a stack
fn breadth_first_search<G>(graph: &G, source: G::NodeId) -> HashSet<G::NodeId>
where
    G: EdgeCount,
    G: IntoNeighbors,
    G: IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
{
    let edge_count = graph.edge_count();

    let mut seen = HashSet::new();
    seen.insert(source);
    let mut next_level = Vec::new();
    next_level.push(source);
    let mut this_level;
    let mut seen_new_vertices = true;

    while seen_new_vertices {
        seen_new_vertices = false;
        this_level = next_level;
        next_level = Vec::new();

        for vertex in this_level {
            for neighbor in graph.neighbors(vertex) {
                if !seen.contains(&neighbor) {
                    seen.insert(neighbor.clone());
                    next_level.push(neighbor);
                    seen_new_vertices = true;
                }
            }
            if seen.len() == edge_count {
                return seen;
            }
        }
    }

    return seen;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_find_maximum_cliques_test_graph_one() {
        let test_graph = crate::tests::setup_test_graph_one();

        let mut components: Vec<Vec<_>> =
            find_connected_components::<Vec<_>, _, _>(&test_graph.graph).collect();

        for i in 0..components.len() {
            components[i].sort();
        }
        components.sort();

        assert_eq!(components, test_graph.expected_connected_components);
    }

    #[test]
    pub fn test_find_maximum_cliques_test_graph_two() {
        let test_graph = crate::tests::setup_test_graph_two();

        let mut components: Vec<Vec<_>> =
            find_connected_components::<Vec<_>, _, _>(&test_graph.graph).collect();

        for i in 0..components.len() {
            components[i].sort();
        }
        components.sort();

        assert_eq!(components, test_graph.expected_connected_components);
    }

    #[test]
    pub fn test_find_maximum_cliques_test_graph_three() {
        let test_graph = crate::tests::setup_test_graph_three();

        let mut components: Vec<Vec<_>> =
            find_connected_components::<Vec<_>, _, _>(&test_graph.graph).collect();

        for i in 0..components.len() {
            components[i].sort();
        }
        components.sort();

        assert_eq!(components, test_graph.expected_connected_components);
    }
}
