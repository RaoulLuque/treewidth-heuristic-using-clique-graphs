use petgraph::graph::NodeIndex;
use rand::Rng;
use std::{collections::HashSet, hash::BuildHasher};

/// Returns 0.
pub fn neutral<S>(_: &HashSet<NodeIndex, S>, _: &HashSet<NodeIndex, S>) -> i32 {
    0
}

/// Returns a random i32 integer
pub fn random<S>(_: &HashSet<NodeIndex, S>, _: &HashSet<NodeIndex, S>) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen::<i32>()
}

/// Returns the negative of the cardinality of the intersection.
pub fn negative_intersection<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    -(first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32)
}

/// Returns the cardinality of the intersection.
pub fn positive_intersection<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .intersection(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

/// Returns the sum of the cardinalities (the sum of the disjoint union).
pub fn disjoint_union<S: BuildHasher>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    (first_vertex.len() + second_vertex.len()) as i32
}

/// Returns the cardinality of the union (sum of the cardinalities - cardinality of intersection).
pub fn union<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .union(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

/// Returns the cardinality of the symmetric difference.
pub fn least_difference<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> i32 {
    first_vertex
        .symmetric_difference(second_vertex)
        .collect::<HashSet<_, S>>()
        .len() as i32
}

/// Returns a tuple with [negative_intersection] in the first and [least_difference] in the second entry
pub fn negative_intersection_then_least_difference<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> (i32, i32) {
    (
        negative_intersection(first_vertex, second_vertex),
        least_difference(first_vertex, second_vertex),
    )
}

/// Returns a tuple with [least_difference] in the first and [negative_intersection] in the second entry.
pub fn least_difference_then_negative_intersection<S: BuildHasher + Default>(
    first_vertex: &HashSet<NodeIndex, S>,
    second_vertex: &HashSet<NodeIndex, S>,
) -> (i32, i32) {
    (
        least_difference(first_vertex, second_vertex),
        negative_intersection(first_vertex, second_vertex),
    )
}
