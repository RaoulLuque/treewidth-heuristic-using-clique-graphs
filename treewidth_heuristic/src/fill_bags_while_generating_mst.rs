use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use petgraph::{graph::NodeIndex, Graph, Undirected};

pub fn fill_bags_while_generating_mst<N, E, O: Ord, S: Default + BuildHasher + Clone>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    clique_graph_map: HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
) -> Graph<HashSet<NodeIndex, S>, O, Undirected> {
    let mut result_graph: Graph<HashSet<NodeIndex, S>, O, Undirected> = Graph::new_undirected();
    // Maps the vertex indices from the clique graph to the corresponding vertex indices in the result graph
    let mut node_index_map: HashMap<NodeIndex, NodeIndex, S> = Default::default();
    let mut vertex_iter = clique_graph.node_indices();

    let first_vertex_clique = vertex_iter.next().expect("Graph shouldn't be empty");

    // Keeps track of the remaining vertices from the clique graph that still need to be added to
    // the result_graph
    let mut clique_graph_remaining_vertices: HashSet<NodeIndex, S> = vertex_iter.collect();

    // Keeps track of the vertices that could be added to the current sub-tree-graph
    // First Tuple entry is node_index from the result graph that has an outgoing edge
    // Second tuple entry is node_index from the clique graph that is the interesting vertex
    let mut currently_interesting_vertices: HashSet<(NodeIndex, NodeIndex), S> = Default::default();

    let first_vertex_res = result_graph.add_node(
        clique_graph
            .node_weight(first_vertex_clique)
            .expect("Vertices in clique graph should have bags as weights")
            .clone(),
    );

    // Add vertices that are reachable from first vertex
    for neighbor in clique_graph.neighbors(first_vertex_clique) {
        currently_interesting_vertices.insert((first_vertex_res, neighbor));
    }
    node_index_map.insert(first_vertex_clique, first_vertex_res);

    while !clique_graph_remaining_vertices.is_empty() {
        let (cheapest_vertex_res, cheapest_vertex_clique) = find_cheapest_vertex(
            &clique_graph,
            &result_graph,
            edge_weight_heuristic,
            &currently_interesting_vertices,
        );
        clique_graph_remaining_vertices.remove(&cheapest_vertex_clique);

        // Update result graph
        let new_vertex_res = result_graph.add_node(
            clique_graph
                .node_weight(cheapest_vertex_clique)
                .expect("Vertices in clique graph should have bags as weights")
                .clone(),
        );

        node_index_map.insert(cheapest_vertex_clique, new_vertex_res);
        result_graph.add_edge(
            cheapest_vertex_res,
            new_vertex_res,
            edge_weight_heuristic(
                result_graph
                    .node_weight(cheapest_vertex_res)
                    .expect("Vertices should have bags as weight"),
                result_graph
                    .node_weight(new_vertex_res)
                    .expect("Vertices should have bags as weight"),
            ),
        );

        // Update currently interesting vertices
        for neighbor in clique_graph.neighbors(cheapest_vertex_clique) {
            if clique_graph_remaining_vertices.contains(&neighbor) {
                currently_interesting_vertices.insert((new_vertex_res, neighbor));
            }
        }

        currently_interesting_vertices
            .retain(|(_, vertex_clique)| !vertex_clique.eq(&cheapest_vertex_clique));

        // Fill bags from result graph
        for vertex_from_starting_graph in result_graph
            .node_weight(new_vertex_res)
            .expect("Vertex should have weight since it was just added")
            .clone()
            .difference(
                &result_graph
                    .node_weight(cheapest_vertex_res)
                    .expect("Vertex should have bag as weight")
                    .clone(),
            )
        {
            if let Some(vertices_in_clique_graph) =
                clique_graph_map.get(&vertex_from_starting_graph)
            {
                for vertex_in_clique_graph in vertices_in_clique_graph {
                    if let Some(vertex_res_graph) = node_index_map.get(vertex_in_clique_graph) {
                        if vertex_res_graph != &new_vertex_res {
                            fill_bags(
                                new_vertex_res,
                                *vertex_res_graph,
                                &mut result_graph,
                                *vertex_from_starting_graph,
                            );
                        }
                    }
                }
            }
        }
    }

    result_graph
}

/// Finds a path in the given graph between start_vertex and end_vertex
///
/// Panics: Panics if there is no path between start and end_vertex, especially in the case that
/// one of the vertices is not contained in the graph
fn fill_bags<O, S: BuildHasher>(
    start_vertex: NodeIndex,
    end_vertex: NodeIndex,
    graph: &mut Graph<HashSet<NodeIndex, S>, O, Undirected>,
    vertex_to_be_insert_from_starting_graph: NodeIndex,
) {
    let mut path: Vec<_> = petgraph::algo::simple_paths::all_simple_paths::<Vec<NodeIndex>, _>(
        &*graph,
        start_vertex,
        end_vertex,
        0,
        None,
    )
    .next()
    .expect("There should be a path in the tree");

    // Last element is the given end node
    path.pop();

    for node_index in path {
        if node_index != start_vertex {
            graph
                .node_weight_mut(node_index)
                .expect("Bag for the vertex should exist")
                .insert(vertex_to_be_insert_from_starting_graph);
        }
    }
}

/// Finds the cheapest edge to a vertex not yet in the result graph considering the bags in the result graph
///
/// Returns a tuple with a node index from the result graph in the first and node index from the clique graph
/// in the second entry. The cheapest edge being the edge between these two nodes only they are different
/// in different representations (result and clique graph respectively)
fn find_cheapest_vertex<O: Ord, S>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    result_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    currently_interesting_vertices: &HashSet<(NodeIndex, NodeIndex), S>,
) -> (NodeIndex, NodeIndex) {
    *currently_interesting_vertices
        .iter()
        .min_by_key(|(vertex_res_graph, interesting_vertex_clique_graph)| edge_weight_heuristic(result_graph.node_weight(*vertex_res_graph).expect(&format!("Vertex {:?} should have weight", vertex_res_graph)), clique_graph.node_weight(*interesting_vertex_clique_graph).expect("Vertices should have weight"))).expect("There should be interesting vertices since there are vertices left and the graph is connected")
}

pub fn fill_bags_while_generating_mst_using_tree<N, E, O: Ord, S: Default + BuildHasher + Clone>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    clique_graph_map: HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
) -> Graph<HashSet<NodeIndex, S>, O, Undirected> {
    let mut result_graph: Graph<HashSet<NodeIndex, S>, O, Undirected> = Graph::new_undirected();
    // Maps the vertex indices from the clique graph to the corresponding vertex indices in the result graph
    let mut node_index_map: HashMap<NodeIndex, NodeIndex, S> = Default::default();
    let mut vertex_iter = clique_graph.node_indices();

    let first_vertex_clique = vertex_iter.next().expect("Graph shouldn't be empty");

    // Maps each vertex to its predecessor and the depth of the predecessor (distance from root) in
    // the result_graph in order to easily find paths in the tree.
    // Root is the first_vertex_clique with depth 0
    let mut tree_predecessor_map: HashMap<NodeIndex, (NodeIndex, usize), S> = Default::default();

    // Keeps track of the remaining vertices from the clique graph that still need to be added to
    // the result_graph
    let mut clique_graph_remaining_vertices: HashSet<NodeIndex, S> = vertex_iter.collect();

    // Keeps track of the vertices that could be added to the current sub-tree-graph
    // First Tuple entry is node_index from the result graph that has an outgoing edge
    // Second tuple entry is node_index from the clique graph that is the interesting vertex
    let mut currently_interesting_vertices: HashSet<(NodeIndex, NodeIndex), S> = Default::default();

    let first_vertex_res = result_graph.add_node(
        clique_graph
            .node_weight(first_vertex_clique)
            .expect("Vertices in clique graph should have bags as weights")
            .clone(),
    );

    // Add vertices that are reachable from first vertex
    for neighbor in clique_graph.neighbors(first_vertex_clique) {
        currently_interesting_vertices.insert((first_vertex_res, neighbor));
    }
    node_index_map.insert(first_vertex_clique, first_vertex_res);

    while !clique_graph_remaining_vertices.is_empty() {
        let (cheapest_vertex_res, cheapest_vertex_clique) = find_cheapest_vertex(
            &clique_graph,
            &result_graph,
            edge_weight_heuristic,
            &currently_interesting_vertices,
        );
        clique_graph_remaining_vertices.remove(&cheapest_vertex_clique);

        // Update result graph
        let new_vertex_res = result_graph.add_node(
            clique_graph
                .node_weight(cheapest_vertex_clique)
                .expect("Vertices in clique graph should have bags as weights")
                .clone(),
        );

        node_index_map.insert(cheapest_vertex_clique, new_vertex_res);
        result_graph.add_edge(
            cheapest_vertex_res,
            new_vertex_res,
            edge_weight_heuristic(
                result_graph
                    .node_weight(cheapest_vertex_res)
                    .expect("Vertices should have bags as weight"),
                result_graph
                    .node_weight(new_vertex_res)
                    .expect("Vertices should have bags as weight"),
            ),
        );

        // Update predecessor map
        if let Some((_, depth)) = tree_predecessor_map.get(&cheapest_vertex_res) {
            tree_predecessor_map.insert(new_vertex_res, (cheapest_vertex_res, depth + 1));
        } else {
            // cheapest vertex res is root
            tree_predecessor_map.insert(new_vertex_res, (cheapest_vertex_res, 0));
        }

        // Update currently interesting vertices
        for neighbor in clique_graph.neighbors(cheapest_vertex_clique) {
            if clique_graph_remaining_vertices.contains(&neighbor) {
                currently_interesting_vertices.insert((new_vertex_res, neighbor));
            }
        }

        currently_interesting_vertices
            .retain(|(_, vertex_clique)| !vertex_clique.eq(&cheapest_vertex_clique));

        // Fill bags from result graph
        for vertex_from_starting_graph in result_graph
            .node_weight(new_vertex_res)
            .expect("Vertex should have weight since it was just added")
            .clone()
        {
            if let Some(vertices_in_clique_graph) =
                clique_graph_map.get(&vertex_from_starting_graph)
            {
                for vertex_in_clique_graph in vertices_in_clique_graph {
                    if let Some(vertex_res_graph) = node_index_map.get(vertex_in_clique_graph) {
                        if vertex_res_graph != &new_vertex_res {
                            let mut vertices_that_need_path_filled: HashSet<NodeIndex, S> =
                                Default::default();
                            vertices_that_need_path_filled.insert(new_vertex_res);
                            vertices_that_need_path_filled.insert(*vertex_res_graph);
                            crate::fill_bags_along_paths::fill_bags_until_common_predecessor(
                                &mut result_graph,
                                &tree_predecessor_map,
                                &vertex_from_starting_graph,
                                &vertices_that_need_path_filled,
                            )
                        }
                    }
                }
            }
        }
    }

    result_graph
}
