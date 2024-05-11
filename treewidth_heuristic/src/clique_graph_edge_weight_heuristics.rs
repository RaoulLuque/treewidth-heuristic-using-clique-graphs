use std::{collections::HashSet, hash::BuildHasher};

use petgraph::graph::NodeIndex;

pub fn neutral_heuristic<S>(_: &HashSet<NodeIndex, S>, _: &HashSet<NodeIndex, S>) -> i32 {
    0
}

pub fn negative_intersection_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    -(first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32)
}

pub fn positive_intersection_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

pub fn disjoint_union_heuristic<S: BuildHasher>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    (first_vertex.len() + second_vertex.len()) as i32
}

pub fn union_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .union(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

pub fn least_difference_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .symmetric_difference(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

pub fn negative_intersection_then_least_difference_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> (i32, i32) {
    (
        negative_intersection_heuristic(first_vertex, second_vertex),
        least_difference_heuristic(first_vertex, second_vertex),
    )
}

pub fn least_difference_then_negative_intersection_heuristic<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> (i32, i32) {
    (
        least_difference_heuristic(first_vertex, second_vertex),
        negative_intersection_heuristic(first_vertex, second_vertex),
    )
}
