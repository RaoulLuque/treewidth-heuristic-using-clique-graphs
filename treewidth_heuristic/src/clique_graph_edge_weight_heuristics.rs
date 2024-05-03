use std::collections::HashSet;

use petgraph::graph::NodeIndex;

pub fn neutral_heuristic(_: &HashSet<NodeIndex>, _: &HashSet<NodeIndex>) -> i32 {
    0
}

// Classic
pub fn negative_intersection_heuristic(
    first_vertex: &HashSet<NodeIndex>,
    second_vertex: &HashSet<NodeIndex>,
) -> i32 {
    -(first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_>>()
        .len() as i32)
}

pub fn positive_intersection_heuristic(
    first_vertex: &HashSet<NodeIndex>,
    second_vertex: &HashSet<NodeIndex>,
) -> i32 {
    first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_>>()
        .len() as i32
}

pub fn disjoint_union_heuristic(
    first_vertex: &HashSet<NodeIndex>,
    second_vertex: &HashSet<NodeIndex>,
) -> i32 {
    (first_vertex.len() + second_vertex.len()) as i32
}

pub fn union_heuristic(
    first_vertex: &HashSet<NodeIndex>,
    second_vertex: &HashSet<NodeIndex>,
) -> i32 {
    first_vertex
        .union(second_vertex)
        .collect::<HashSet<_>>()
        .len() as i32
}

// Classic alt?
pub fn least_difference_heuristic(
    first_vertex: &HashSet<NodeIndex>,
    second_vertex: &HashSet<NodeIndex>,
) -> i32 {
    first_vertex
        .symmetric_difference(second_vertex)
        .collect::<HashSet<_>>()
        .len() as i32
}
