use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use log::error;
use petgraph::{
    prelude::*,
    visit::{IntoNodeReferences, NodeRef},
};

/// Given a tree decomposition checks if it is a valid tree decomposition. Returns true if the decomposition
/// is valid, returns false otherwise.
pub fn check_tree_decomposition(
    tree_decomposition_graph: &Graph<
        std::collections::HashSet<petgraph::prelude::NodeIndex>,
        i32,
        petgraph::prelude::Undirected,
    >,
    predecessor_map: &HashMap<NodeIndex, (NodeIndex, usize)>,
    clique_graph_map: &HashMap<NodeIndex, HashSet<NodeIndex>>,
) -> bool {
    for mut vec in tree_decomposition_graph.node_references().combinations(2) {
        let first_tuple = vec.pop().expect("Vec should contain two items");
        let second_tuple = vec.pop().expect("Vec should contain two items");
        let (first_id, first_weight, second_id, second_weight) = (
            first_tuple.id(),
            first_tuple.weight(),
            second_tuple.id(),
            second_tuple.weight(),
        );

        let intersection_set: HashSet<_> =
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

        for node_index in path.clone() {
            if node_index != first_id {
                if !tree_decomposition_graph
                    .node_weight(node_index)
                    .expect("Bag for the vertex should exist")
                    .is_superset(&intersection_set)
                {
                    let vertices_missing_along_path: HashSet<_> = intersection_set
                        .difference(tree_decomposition_graph.node_weight(node_index).unwrap())
                        .collect();

                    // DEBUG
                    error!("Between the vertex: {:?} \n 
                    and vertex: {:?} \n 
                    the bags intersect with: {:?} \n 
                    however vertex {:?} along their path doesn't contain the following vertices: {:?} \n \n

                    The full path is: {:?}",
                    first_tuple, second_tuple, intersection_set, node_index, vertices_missing_along_path, path);

                    for node_index in vertices_missing_along_path {
                        error!("The intersecting vertex {:?} is contained in the following vertices in the clique graph: {:?}", node_index, clique_graph_map.get(&node_index).unwrap())
                    }

                    for node_index in path {
                        error!(
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
                    return false;
                }
            }
        }
    }
    true
}
