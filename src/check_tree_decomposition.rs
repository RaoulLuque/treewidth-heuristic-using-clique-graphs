use itertools::Itertools;
use petgraph::{
    prelude::*,
    visit::{IntoNodeReferences, NodeRef},
};
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

/// Given a tree decomposition checks if it is a valid tree decomposition. Returns true if the decomposition
/// is valid, returns false otherwise.
///
/// If predecessor map and clique graph map are passed, gives additional in the case that it is a faulty tree decomposition.
pub fn check_tree_decomposition<N, E, O, S: BuildHasher + Default>(
    starting_graph: &Graph<N, E, Undirected>,
    tree_decomposition_graph: &Graph<
        std::collections::HashSet<petgraph::prelude::NodeIndex, S>,
        O,
        petgraph::prelude::Undirected,
    >,
    predecessor_map: &Option<HashMap<NodeIndex, (NodeIndex, usize), S>>,
    clique_graph_map: &Option<HashMap<NodeIndex, HashSet<NodeIndex, S>, S>>,
) -> bool {
    // Check if (1) from tree decomposition is satisfied (all vertices from starting graph appear in a bag in
    // tree decomposition graph)
    for vertex in starting_graph.node_indices() {
        if let None = tree_decomposition_graph
            .node_weights()
            .find(|s| s.contains(&vertex))
        {
            println!("Tree decomposition doesn't contain vertex: {:?}", vertex);
            return false;
        }
    }
    // Check if (2) from tree decomposition is satisfied (for all edges in starting graph there is bag containing
    // both its vertices)
    for edge_reference in starting_graph.edge_references() {
        let (vertex_one, vertex_two) = (edge_reference.source(), edge_reference.target());
        let mut edge_as_set: HashSet<_, S> = Default::default();
        edge_as_set.insert(vertex_one);
        edge_as_set.insert(vertex_two);
        let mut edge_is_contained = false;

        for vertex_weight in tree_decomposition_graph.node_weights() {
            if vertex_weight.is_superset(&edge_as_set) {
                edge_is_contained = true;
            }
        }

        if !edge_is_contained {
            println!("Tree decomposition doesn't contain edge: {:?}", edge_as_set);
            return false;
        }
    }
    // check if (3) from tree decomposition definition is satisfied (for one vertex in starting graph, all bags
    // contain this vertex induce a subtree)
    for mut vec in tree_decomposition_graph.node_references().combinations(2) {
        let first_tuple = vec.pop().expect("Vec should contain two items");
        let second_tuple = vec.pop().expect("Vec should contain two items");
        let (first_id, first_weight, second_id, second_weight) = (
            first_tuple.id(),
            first_tuple.weight(),
            second_tuple.id(),
            second_tuple.weight(),
        );

        let intersection_set: HashSet<_, S> =
            first_weight.intersection(second_weight).cloned().collect();

        assert_eq!(
            petgraph::algo::simple_paths::all_simple_paths::<Vec<NodeIndex>, _>(
                tree_decomposition_graph,
                first_id,
                second_id,
                0,
                None,
            )
            .collect_vec()
            .len(),
            1,
            "There should only be one path from each vertex to another vertex in a tree"
        );

        let path: Vec<_> = petgraph::algo::simple_paths::all_simple_paths::<Vec<NodeIndex>, _>(
            tree_decomposition_graph,
            first_id,
            second_id,
            0,
            None,
        )
        .next()
        .expect("There should be a path in the tree");
        if !intersection_set.is_empty() {
            for node_index in path.clone() {
                if node_index != first_id {
                    if !tree_decomposition_graph
                        .node_weight(node_index)
                        .expect("Bag for the vertex should exist")
                        .is_superset(&intersection_set)
                    {
                        let vertices_missing_along_path: HashSet<_, S> = intersection_set
                            .difference(tree_decomposition_graph.node_weight(node_index).unwrap())
                            .collect();

                        // DEBUG
                        println!("Between the vertex: {:?} \n 
                    and vertex: {:?} \n 
                    the bags intersect with: {:?} \n 
                    however vertex {:?} along their path doesn't contain the following vertices: {:?} \n \n

                    The full path is: {:?}",
                    first_tuple, second_tuple, intersection_set, node_index, vertices_missing_along_path, path);

                        if let (Some(predecessor_map), Some(clique_graph_map)) =
                            (predecessor_map, clique_graph_map)
                        {
                            // DEBUG
                            for node_index in vertices_missing_along_path {
                                println!("The intersecting vertex {:?} is contained in the following vertices in the clique graph: {:?}", node_index, clique_graph_map.get(&node_index).unwrap())
                            }

                            // DEBUG
                            for node_index in path {
                                println!(
                                    "{:?} with level: {} and predecessor {:?} 
                            and bag {:?}",
                                    node_index,
                                    match predecessor_map.get(&node_index) {
                                        Some(predecessor) => predecessor.1 + 1,
                                        None => 0,
                                    },
                                    match predecessor_map.get(&node_index) {
                                        Some(predecessor) => Some(predecessor.0),
                                        None => None,
                                    },
                                    tree_decomposition_graph.node_weight(node_index).unwrap()
                                );
                            }
                        }
                        return false;
                    }
                }
            }
        }
    }
    true
}
