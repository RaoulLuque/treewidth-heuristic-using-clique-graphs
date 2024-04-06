use itertools::{Combinations, Itertools};
use petgraph::visit::{GraphBase, IntoNeighborsDirected, IntoNodeIdentifiers, NodeCount};
use std::iter::from_fn;
use std::{collections::HashSet, hash::Hash};

/// Returns an iterator that produces all maximum cliques in the given graph in arbitrary order.
///
/// This algorithm is adapted from <https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.clique.find_cliques.html>.
pub fn find_maximum_cliques<TargetColl, G>(graph: G) -> impl Iterator<Item = TargetColl>
where
    G: NodeCount,
    G: IntoNeighborsDirected,
    G: IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
    TargetColl: FromIterator<G::NodeId>,
    <G as GraphBase>::NodeId: 'static,
{
    // stack of nodes that are in the clique that is currently being constructed
    let mut current_clique: Vec<Option<<G as GraphBase>::NodeId>> = vec![None];
    // list of children of currently exploring path nodes,
    // last elem is list of children of last visited node
    let mut stack = vec![];

    let mut atcc: HashSet<G::NodeId> = graph.node_identifiers().collect();

    let u = *atcc
        .iter()
        .max_by_key(|v| {
            let mut tmp = graph.neighbors(**v).collect::<Vec<_>>();
            tmp.retain(|w| atcc.contains(w));
            tmp.len()
        })
        .expect("Graph shouldn't be empty");

    let mut promising_candidates: Vec<G::NodeId> = atcc.iter().cloned().collect();
    let neighbors_u: HashSet<G::NodeId> = graph.neighbors(u).collect();
    promising_candidates.retain(|v| !neighbors_u.contains(v));

    let mut candidates: HashSet<G::NodeId> = graph.node_identifiers().collect();

    // current clique - Q                       : Clique that is currently being constructed
    // candidates - cand                        : Current candidates that could be added to Q (current Clique) - special for handling cliques with the given set of nodes
    // adjacent to current clique - atcc - subg : Nodes that are adjacent to all nodes so far in Q (current Clique)
    // promising_candidates                     : Current candidates that could be added to Q (current Clique)

    from_fn(move || {
        // Check if graph is empty
        if graph.node_count() == 0 {
            return None;
        }

        loop {
            if let Some(q) = promising_candidates.pop() {
                if current_clique.len() > 0 {
                    let len = current_clique.len();
                    current_clique[len - 1] = Some(q);

                    candidates.remove(&q);

                    let adjacent_to_q: HashSet<G::NodeId> = graph.neighbors(q).collect();
                    let mut atcc_q = atcc.clone();
                    atcc_q.retain(|v| adjacent_to_q.contains(v));

                    if atcc_q.is_empty() {
                        let clique: TargetColl = current_clique
                            .iter()
                            .cloned()
                            .flatten()
                            .collect::<TargetColl>();
                        return Some(clique);
                    } else {
                        let mut candidates_q = candidates.clone();
                        candidates_q.retain(|v| adjacent_to_q.contains(v));
                        if !candidates_q.is_empty() {
                            stack.push((
                                atcc.clone(),
                                candidates.clone(),
                                promising_candidates.clone(),
                            ));
                            current_clique.push(None);
                            atcc = atcc_q.clone();
                            candidates = candidates_q.clone();

                            let u = *atcc
                                .iter()
                                .max_by_key(|v| {
                                    let mut tmp = graph.neighbors(**v).collect::<Vec<_>>();
                                    tmp.retain(|w| atcc.contains(w));
                                    tmp.len()
                                })
                                .expect("Graph shouldn't be empty");
                            promising_candidates = candidates.iter().cloned().collect();
                            let neighbors_u: HashSet<G::NodeId> = graph.neighbors(u).collect();
                            promising_candidates.retain(|v| !neighbors_u.contains(v));
                        }
                    }
                }
            } else {
                current_clique.pop();
                if let Some(stack_element) = stack.pop() {
                    (atcc, candidates, promising_candidates) = stack_element;
                } else {
                    return None;
                }
            }
        }
    })
}

pub fn find_maximum_cliques_bounded<TargetColl, G>(
    graph: G,
    k: usize,
) -> impl Iterator<Item = TargetColl>
where
    G: NodeCount,
    G: IntoNeighborsDirected,
    G: IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
    TargetColl: FromIterator<G::NodeId>,
    <G as GraphBase>::NodeId: 'static,
{
    let mut maximum_cliques = find_maximum_cliques::<HashSet<_>, G>(graph);
    let mut combinations: Combinations<_> = HashSet::new().into_iter().combinations(k);
    from_fn(move || loop {
        if let Some(clique_combination) = combinations.next() {
            return Some(clique_combination.into_iter().collect::<TargetColl>());
        } else if let Some(clique) = maximum_cliques.next() {
            if clique.len() <= k {
                return Some(clique.into_iter().collect::<TargetColl>());
            } else {
                combinations = clique.into_iter().combinations(k);
            }
        } else {
            return None;
        }
    })
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;

    use super::*;

    #[test]
    pub fn test_find_maximum_cliques1() {
        let mut graph: Graph<u32, u32, petgraph::prelude::Undirected> =
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

        let mut cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(&graph).collect();

        for i in 0..cliques.len() {
            cliques[i].sort();
        }
        cliques.sort();

        let expected: Vec<Vec<_>> = vec![
            vec![2, 6, 1, 3],
            vec![2, 6, 4],
            vec![5, 4, 7],
            vec![8, 9],
            vec![10, 11],
        ];
        let mut expected: Vec<Vec<_>> = expected
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|e| petgraph::graph::node_index(e - 1))
                    .collect::<Vec<_>>()
            })
            .collect();
        for i in 0..expected.len() {
            expected[i].sort();
        }
        expected.sort();

        assert_eq!(cliques, expected);
    }

    #[test]
    fn test_find_maximum_cliques2() {
        let mut graph: Graph<u32, u32, petgraph::prelude::Undirected> =
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

        let mut cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(&graph).collect();

        for i in 0..cliques.len() {
            cliques[i].sort();
        }
        cliques.sort();

        let expected: Vec<Vec<_>> = vec![vec![1, 2], vec![1, 4, 5, 6], vec![2, 3], vec![3, 4, 6]];
        let mut expected: Vec<Vec<_>> = expected
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|e| petgraph::graph::node_index(e - 1))
                    .collect::<Vec<_>>()
            })
            .collect();
        for i in 0..expected.len() {
            expected[i].sort();
        }
        expected.sort();

        assert_eq!(cliques, expected);
    }

    #[test]
    pub fn test_find_maximum_cliques_bounded() {
        let mut graph: Graph<u32, u32, petgraph::prelude::Undirected> =
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

        let mut cliques: Vec<Vec<_>> =
            find_maximum_cliques_bounded::<Vec<_>, _>(&graph, 3).collect();

        for i in 0..cliques.len() {
            cliques[i].sort();
        }
        cliques.sort();

        let expected: Vec<Vec<_>> = vec![
            vec![2, 6, 1],
            vec![2, 6, 3],
            vec![2, 1, 3],
            vec![6, 1, 3],
            vec![2, 6, 4],
            vec![5, 4, 7],
            vec![8, 9],
            vec![10, 11],
        ];
        let mut expected: Vec<Vec<_>> = expected
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|e| petgraph::graph::node_index(e - 1))
                    .collect::<Vec<_>>()
            })
            .collect();
        for i in 0..expected.len() {
            expected[i].sort();
        }
        expected.sort();

        assert_eq!(cliques, expected);
    }
}
