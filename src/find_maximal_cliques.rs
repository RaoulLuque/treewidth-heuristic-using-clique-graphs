use itertools::Itertools;
use petgraph::visit::{GraphBase, IntoNeighborsDirected, IntoNodeIdentifiers, NodeCount};
use std::hash::BuildHasher;
use std::iter::from_fn;
use std::{collections::HashSet, hash::Hash};

/// Returns an iterator that produces all [maximal cliques][https://en.wikipedia.org/wiki/Clique_(graph_theory)#Definitions]
/// in the given graph in arbitrary order.
///
/// This algorithm is adapted from [networkX find_cliques algorithm][https://networkx.org/documentation/stable/reference/algorithms/generated/networkx.algorithms.clique.find_cliques.html].
pub fn find_maximal_cliques<TargetColl, G, S: Default + BuildHasher + Clone>(
    graph: G,
) -> impl Iterator<Item = TargetColl>
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

    let mut atcc: HashSet<G::NodeId, S> = graph.node_identifiers().collect();

    let u = *atcc
        .iter()
        .max_by_key(|v| {
            let mut tmp = graph.neighbors(**v).collect::<Vec<_>>();
            tmp.retain(|w| atcc.contains(w));
            tmp.len()
        })
        .expect("Graph shouldn't be empty");

    let mut promising_candidates: Vec<G::NodeId> = atcc.iter().cloned().collect();
    let neighbors_u: HashSet<G::NodeId, S> = graph.neighbors(u).collect();
    promising_candidates.retain(|v| !neighbors_u.contains(v));

    let mut candidates: HashSet<G::NodeId, S> = graph.node_identifiers().collect();

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

                    let adjacent_to_q: HashSet<G::NodeId, S> = graph.neighbors(q).collect();
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
                            let neighbors_u: HashSet<G::NodeId, S> = graph.neighbors(u).collect();
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

/// Returns an iterator that produces (once each) all cliques that are [maximal][https://en.wikipedia.org/wiki/Clique_(graph_theory)#Definitions]
/// (and of size less than k) or of size k (and not necessarily maximal) in arbitrary order.
/// If k is negative, k is set by the function as k = k + omega(G) where omega(G) is the clique number of G
/// (the size of a maximum clique in G). Therefore, for k = -1, k = omega(G) - 1 is used instead.
///
/// Uses the [find_maximum_cliques] method.
pub fn find_maximal_cliques_bounded<TargetColl, G, S: Default + Clone + BuildHasher>(
    graph: G,
    k: i32,
) -> impl Iterator<Item = TargetColl>
where
    G: NodeCount,
    G: IntoNeighborsDirected,
    G: IntoNodeIdentifiers,
    G::NodeId: Eq + Hash + Ord,
    TargetColl: FromIterator<G::NodeId>,
    <G as GraphBase>::NodeId: 'static,
{
    let maximal_cliques = find_maximal_cliques::<HashSet<_, S>, G, S>(graph);
    let k = if k < 0 {
        maximal_cliques
            .max_by_key(|c| c.len())
            .expect("The graph should not be empty")
            .len()
            + k as usize
    } else {
        k as usize
    };

    let mut maximal_cliques = find_maximal_cliques::<HashSet<_, S>, G, S>(graph);
    let mut combinations = HashSet::<_, S>::default().into_iter().combinations(k);
    let mut seen_combinations = HashSet::<_, S>::default();
    from_fn(move || loop {
        if let Some(mut clique_combination) = combinations.next() {
            clique_combination.sort();
            if seen_combinations.insert(clique_combination.clone()) {
                // Only insert combination if it hasn't been seen yet (remove duplicate combinations)
                return Some(clique_combination.into_iter().collect::<TargetColl>());
            }
        } else if let Some(clique) = maximal_cliques.next() {
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
    use std::hash::RandomState;

    use super::*;

    #[test]
    pub fn test_find_maximum_cliques() {
        for i in 0..3 {
            let test_graph = crate::tests::setup_test_graph(i);

            let mut cliques: Vec<Vec<_>> =
                find_maximal_cliques::<Vec<_>, _, RandomState>(&test_graph.graph).collect();

            for i in 0..cliques.len() {
                cliques[i].sort();
            }
            cliques.sort();

            assert_eq!(
                cliques, test_graph.expected_max_cliques,
                "Test graph: {}",
                i
            );
        }
    }

    #[test]
    pub fn test_find_maximum_cliques_bounded() {
        let test_graph = crate::tests::setup_test_graph(0);

        let mut cliques: Vec<Vec<_>> =
            find_maximal_cliques_bounded::<Vec<_>, _, RandomState>(&test_graph.graph, 3).collect();

        for i in 0..cliques.len() {
            cliques[i].sort();
        }
        cliques.sort();

        let expected_bounded_max_cliques: Vec<Vec<_>> = vec![
            vec![2, 6, 1],
            vec![2, 6, 3],
            vec![2, 1, 3],
            vec![6, 1, 3],
            vec![2, 6, 4],
            vec![5, 4, 7],
            vec![8, 9],
            vec![10, 11],
        ];
        let mut expected_bounded_max_cliques: Vec<Vec<_>> = expected_bounded_max_cliques
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|e| petgraph::graph::node_index(e - 1))
                    .collect::<Vec<_>>()
            })
            .collect();
        for i in 0..expected_bounded_max_cliques.len() {
            expected_bounded_max_cliques[i].sort();
        }
        expected_bounded_max_cliques.sort();

        assert_eq!(cliques, expected_bounded_max_cliques);

        let test_graph = crate::tests::setup_test_graph(2);

        let mut cliques: Vec<Vec<_>> =
            find_maximal_cliques_bounded::<Vec<_>, _, RandomState>(&test_graph.graph, 3).collect();

        for i in 0..cliques.len() {
            cliques[i].sort();
        }
        cliques.sort();

        let expected_bounded_max_cliques: Vec<Vec<_>> = vec![
            vec![1, 2, 3],
            vec![1, 2, 4],
            vec![1, 3, 4],
            vec![2, 3, 4],
            vec![2, 3, 5],
            vec![2, 4, 5],
            vec![3, 4, 5],
        ];

        let mut expected_bounded_max_cliques: Vec<Vec<_>> = expected_bounded_max_cliques
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|e| petgraph::graph::node_index(e - 1))
                    .collect::<Vec<_>>()
            })
            .collect();
        for i in 0..expected_bounded_max_cliques.len() {
            expected_bounded_max_cliques[i].sort();
        }
        expected_bounded_max_cliques.sort();

        assert_eq!(cliques, expected_bounded_max_cliques);
    }
}
