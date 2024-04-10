use petgraph::visit::IntoNeighborsDirected;
use std::collections::VecDeque;
use std::hash::Hash;

/// Returns an Option with a vector starting with start and continuing with a path to end, ending with end.
/// Is implemented using a stack and depth first search.
///
/// Returns None if no path exists (should never happen in a tree).
pub fn find_path_in_tree<G, T>(graph: G, start: G::NodeId, end: G::NodeId) -> Option<Vec<G::NodeId>>
where
    T: FromIterator<G::NodeId>,
    G: IntoNeighborsDirected,
    G::NodeId: Eq + Hash,
{
    let mut path_so_far = Vec::new();
    path_so_far.push(start);

    let mut stack = VecDeque::new();
    stack.push_back((path_so_far.clone(), start));

    while let Some((mut path_so_far, current_vertex)) = stack.pop_front() {
        for next_vertex in graph.neighbors(current_vertex) {
            if path_so_far.len() <= 1
                || *path_so_far
                    .get(path_so_far.len() - 2)
                    .expect("Path so far should be long enough")
                    != next_vertex
            {
                if next_vertex == end {
                    path_so_far.push(next_vertex);
                    return Some(path_so_far);
                } else {
                    path_so_far.push(next_vertex);
                    stack.push_back((path_so_far.clone(), next_vertex));
                }
            }
        }
    }

    None
}
