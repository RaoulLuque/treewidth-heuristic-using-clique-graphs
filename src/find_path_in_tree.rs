use petgraph::visit::IntoNeighborsDirected;
use std::collections::{HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};

/// Returns an Option with a vector starting with start and continuing with a path to end, ending with end.
/// Is implemented using a stack and depth first search.
///
/// Returns None if no path exists (should never happen in a tree).
pub fn find_path_in_tree<G, T, S: Default + BuildHasher>(
    graph: G,
    start: G::NodeId,
    end: G::NodeId,
) -> Option<Vec<G::NodeId>>
where
    T: FromIterator<G::NodeId>,
    G: IntoNeighborsDirected,
    G::NodeId: Eq + Hash + Debug,
{
    let mut path_so_far = Vec::new();
    path_so_far.push(start);

    let mut stack = VecDeque::new();
    stack.push_back((path_so_far.clone(), start));
    let mut visited: HashSet<_, S> = Default::default();

    while let Some((mut path_so_far, current_vertex)) = stack.pop_front() {
        for next_vertex in graph.neighbors(current_vertex) {
            if !visited.contains(&next_vertex) {
                if next_vertex == end {
                    path_so_far.push(next_vertex);
                    return Some(path_so_far);
                } else {
                    path_so_far.push(next_vertex);
                    visited.insert(next_vertex);
                    stack.push_back((path_so_far.clone(), next_vertex));
                }
            }
        }
    }
    None
}
