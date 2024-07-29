use csv::WriterBuilder;
use petgraph::{graph::NodeIndex, Graph, Undirected};
use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

/// The function computes a [tree decomposition][https://en.wikipedia.org/wiki/Tree_decomposition]
/// with the vertices having bags (HashSets) as labels
/// given a clique graph. For this a minimum spanning tree of the clique graph is constructed using
/// prim's algorithm and the edge labels in the clique graph as edge weights. Whenever a new vertex
/// is added to the spanning tree, the bags of the current spanning tree are filled up/updated
/// according to the [tree decomposition criteria][https://en.wikipedia.org/wiki/Tree_decomposition#Definition].
///
/// **Panics**
/// The log_bag_size parameter enables logging of the increase in size of the biggest bag of the spanning
/// tree over time while the spanning tree is constructed (i.e. for each new vertex added to the spanning
/// tree, logs the current size of the biggest bag). If log_bag_size == true the file
/// k-tree-benchmarks/benchmark_results/k_tree_maximum_bag_size_over_time.csv (where k-tree-benchmarks
/// is a subdirectory of the runtime directory) has to exist otherwise this function will panic.
pub fn fill_bags_while_generating_mst<N, E, O: Ord, S: Default + BuildHasher + Clone>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    edge_weight_heuristic: fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> O,
    clique_graph_map: HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    log_bag_size: bool,
) -> Graph<HashSet<NodeIndex, S>, O, Undirected> {
    // For logging the size of the maximum bags. Stays empty if log_bag_size == False
    let mut vector_for_logging = Vec::new();

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

    // Log current maximum bag size
    if log_bag_size {
        vector_for_logging.push(
            crate::find_width_of_tree_decomposition::find_width_of_tree_decomposition(
                &result_graph,
            ),
        );
    }

    while !clique_graph_remaining_vertices.is_empty() {
        // The cheapest_old_vertex_res is one of the vertices from the already constructed tree that the new vertex
        // is being attached to
        // The cheapest_new_vertex_clique is the new vertex that is being added to the tree. The NodeIndex corresponds
        // to the vertex in the clique graph and not the result graph and thus still needs to be translated.
        let (cheapest_old_vertex_res, cheapest_new_vertex_clique) = find_cheapest_vertex(
            &clique_graph,
            &result_graph,
            edge_weight_heuristic,
            &currently_interesting_vertices,
        );
        clique_graph_remaining_vertices.remove(&cheapest_new_vertex_clique);

        // Update result graph
        let cheapest_new_vertex_res = result_graph.add_node(
            clique_graph
                .node_weight(cheapest_new_vertex_clique)
                .expect("Vertices in clique graph should have bags as weights")
                .clone(),
        );

        node_index_map.insert(cheapest_new_vertex_clique, cheapest_new_vertex_res);
        result_graph.add_edge(
            cheapest_old_vertex_res,
            cheapest_new_vertex_res,
            edge_weight_heuristic(
                result_graph
                    .node_weight(cheapest_old_vertex_res)
                    .expect("Vertices should have bags as weight"),
                result_graph
                    .node_weight(cheapest_new_vertex_res)
                    .expect("Vertices should have bags as weight"),
            ),
        );

        // Update currently interesting vertices
        for neighbor in clique_graph.neighbors(cheapest_new_vertex_clique) {
            if clique_graph_remaining_vertices.contains(&neighbor) {
                currently_interesting_vertices.insert((cheapest_new_vertex_res, neighbor));
            }
        }

        currently_interesting_vertices
            .retain(|(_, vertex_clique)| !vertex_clique.eq(&cheapest_new_vertex_clique));

        fill_bags_from_result_graph(
            &mut result_graph,
            cheapest_new_vertex_res,
            cheapest_old_vertex_res,
            &clique_graph_map,
            &node_index_map,
        );

        // Log current maximum bag size
        vector_for_logging.push(
            crate::find_width_of_tree_decomposition::find_width_of_tree_decomposition(
                &result_graph,
            ),
        );
    }

    // Log bag size if log_bag_size == true
    if log_bag_size {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("k-tree-benchmarks/benchmark_results/k_tree_maximum_bag_size_over_time.csv")
            .unwrap();

        let mut writer = WriterBuilder::new().flexible(false).from_writer(file);
        let vector_for_logging = vector_for_logging.into_iter().map(|v| v.to_string());
        writer
            .write_record(vector_for_logging)
            .expect("Writing to logs for maximum bag size for fill while should be possible");
        writer
            .flush()
            .expect("Flushing logs for maximum bag size for fill while should be possible");
    }

    result_graph
}

fn fill_bags_from_result_graph<S: BuildHasher + Clone, O>(
    result_graph: &mut Graph<HashSet<NodeIndex, S>, O, Undirected>,
    new_vertex_res: NodeIndex,
    cheapest_old_vertex_res: NodeIndex,
    clique_graph_map: &HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    node_index_map: &HashMap<NodeIndex, NodeIndex, S>,
) {
    for vertex_from_starting_graph in result_graph
        .node_weight(new_vertex_res)
        .expect("Vertex should have weight since it was just added")
        .clone()
        .difference(
            &result_graph
                .node_weight(cheapest_old_vertex_res)
                .expect("Vertex should have bag as weight")
                .clone(),
        )
    {
        if let Some(vertices_in_clique_graph) = clique_graph_map.get(&vertex_from_starting_graph) {
            for vertex_in_clique_graph in vertices_in_clique_graph {
                if let Some(vertex_res_graph) = node_index_map.get(vertex_in_clique_graph) {
                    if vertex_res_graph != &new_vertex_res {
                        fill_bags(
                            new_vertex_res,
                            *vertex_res_graph,
                            result_graph,
                            *vertex_from_starting_graph,
                        );
                    }
                }
            }
        }
    }
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

/// Computes a tree decomposition similar to [fill_bags_while_generating_mst] except that whenever
/// a vertex is added to the current spanning tree and the bags of the current spanning tree are
/// filled up/updated, edges to other vertices in the entire clique graph are updated (in order to
/// preserve the property that two vertices/bags in the clique graph are adjacent iff they have a
/// non-empty intersection).
pub fn fill_bags_while_generating_mst_update_edges<
    N,
    E,
    O: Ord,
    S: Default + BuildHasher + Clone,
>(
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
        // The cheapest_old_vertex_res is one of the vertices from the already constructed tree that the new vertex
        // is being attached to
        // The cheapest_new_vertex_clique is the new vertex that is being added to the tree. The NodeIndex corresponds
        // to the vertex in the clique graph and not the result graph and thus still needs to be translated.
        let (cheapest_old_vertex_res, cheapest_new_vertex_clique) = find_cheapest_vertex(
            &clique_graph,
            &result_graph,
            edge_weight_heuristic,
            &currently_interesting_vertices,
        );
        clique_graph_remaining_vertices.remove(&cheapest_new_vertex_clique);

        // Update result graph
        let cheapest_new_vertex_res = result_graph.add_node(
            clique_graph
                .node_weight(cheapest_new_vertex_clique)
                .expect("Vertices in clique graph should have bags as weights")
                .clone(),
        );

        node_index_map.insert(cheapest_new_vertex_clique, cheapest_new_vertex_res);
        result_graph.add_edge(
            cheapest_old_vertex_res,
            cheapest_new_vertex_res,
            edge_weight_heuristic(
                result_graph
                    .node_weight(cheapest_old_vertex_res)
                    .expect("Vertices should have bags as weight"),
                result_graph
                    .node_weight(cheapest_new_vertex_res)
                    .expect("Vertices should have bags as weight"),
            ),
        );

        // Update currently interesting vertices
        for neighbor in clique_graph.neighbors(cheapest_new_vertex_clique) {
            if clique_graph_remaining_vertices.contains(&neighbor) {
                currently_interesting_vertices.insert((cheapest_new_vertex_res, neighbor));
            }
        }

        currently_interesting_vertices
            .retain(|(_, vertex_clique)| !vertex_clique.eq(&cheapest_new_vertex_clique));

        fill_bags_from_result_graph_updating_edges(
            &mut result_graph,
            cheapest_new_vertex_res,
            cheapest_old_vertex_res,
            &clique_graph_map,
            &node_index_map,
            &mut currently_interesting_vertices,
        );
    }

    result_graph
}

fn fill_bags_from_result_graph_updating_edges<S: BuildHasher + Clone, O>(
    result_graph: &mut Graph<HashSet<NodeIndex, S>, O, Undirected>,
    new_vertex_res: NodeIndex,
    cheapest_old_vertex_res: NodeIndex,
    clique_graph_map: &HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    node_index_map: &HashMap<NodeIndex, NodeIndex, S>,
    currently_interesting_vertices: &mut HashSet<(NodeIndex, NodeIndex), S>,
) {
    for vertex_from_starting_graph in result_graph
        .node_weight(new_vertex_res)
        .expect("Vertex should have weight since it was just added")
        .clone()
        .difference(
            &result_graph
                .node_weight(cheapest_old_vertex_res)
                .expect("Vertex should have bag as weight")
                .clone(),
        )
    {
        if let Some(vertices_in_clique_graph) = clique_graph_map.get(&vertex_from_starting_graph) {
            for vertex_in_clique_graph in vertices_in_clique_graph {
                if let Some(vertex_res_graph) = node_index_map.get(vertex_in_clique_graph) {
                    if vertex_res_graph != &new_vertex_res {
                        fill_bags_updating_edges(
                            new_vertex_res,
                            *vertex_res_graph,
                            result_graph,
                            *vertex_from_starting_graph,
                            clique_graph_map,
                            node_index_map,
                            currently_interesting_vertices,
                        );
                    }
                }
            }
        }
    }
}

/// Adapted from [fill_bags]
fn fill_bags_updating_edges<O, S: BuildHasher>(
    start_vertex: NodeIndex,
    end_vertex: NodeIndex,
    graph: &mut Graph<HashSet<NodeIndex, S>, O, Undirected>,
    vertex_to_be_insert_from_starting_graph: NodeIndex,
    clique_graph_map: &HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    node_index_map: &HashMap<NodeIndex, NodeIndex, S>,
    currently_interesting_vertices: &mut HashSet<(NodeIndex, NodeIndex), S>,
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

            for vertex_clique_graph in clique_graph_map
                .get(&vertex_to_be_insert_from_starting_graph)
                .expect("There should be bags containing this vertex")
            {
                if !node_index_map.contains_key(vertex_clique_graph) {
                    currently_interesting_vertices.insert((node_index, *vertex_clique_graph));
                }
            }
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

/// Computes a tree decomposition similar to [fill_bags_while_generating_mst] except that instead of
/// using edge weights in prim's algorithm, the weight of an edge (u,v) (v is not yet in the
/// spanning tree) is the size of the biggest bag in the spanning tree if v was added to the
/// spanning tree and the bags were filled up/updated accordingly.
pub fn fill_bags_while_generating_mst_least_bag_size<
    N,
    E,
    O: Ord + Default + Clone,
    S: Default + BuildHasher + Clone,
>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
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
        let (cheapest_old_vertex_res, cheapest_vertex_clique) = find_vertex_that_minimizes_bag_size(
            &clique_graph,
            &result_graph,
            &currently_interesting_vertices,
            &clique_graph_map,
            &node_index_map,
        );
        clique_graph_remaining_vertices.remove(&cheapest_vertex_clique);

        // Update result graph
        let cheapest_new_vertex_res = result_graph.add_node(
            clique_graph
                .node_weight(cheapest_vertex_clique)
                .expect("Vertices in clique graph should have bags as weights")
                .clone(),
        );

        node_index_map.insert(cheapest_vertex_clique, cheapest_new_vertex_res);
        result_graph.add_edge(
            cheapest_old_vertex_res,
            cheapest_new_vertex_res,
            O::default(),
        );

        // Update currently interesting vertices
        for neighbor in clique_graph.neighbors(cheapest_vertex_clique) {
            if clique_graph_remaining_vertices.contains(&neighbor) {
                currently_interesting_vertices.insert((cheapest_new_vertex_res, neighbor));
            }
        }

        currently_interesting_vertices
            .retain(|(_, vertex_clique)| !vertex_clique.eq(&cheapest_vertex_clique));

        fill_bags_from_result_graph(
            &mut result_graph,
            cheapest_new_vertex_res,
            cheapest_old_vertex_res,
            &clique_graph_map,
            &node_index_map,
        );
    }

    result_graph
}

/// Finds the cheapest edge to a vertex not yet in the result graph trying find the vertex that minimizes
/// the size of the biggest bag in the result graph if the vertex is added.
///
/// Returns a tuple with a node index from the result graph in the first and node index from the clique graph
/// in the second entry. The cheapest edge being the edge between these two nodes only they are different
/// in different representations (result and clique graph respectively)
fn find_vertex_that_minimizes_bag_size<O: Ord + Default + Clone, S: BuildHasher + Clone>(
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    result_graph: &Graph<HashSet<NodeIndex, S>, O, Undirected>,
    currently_interesting_vertices: &HashSet<(NodeIndex, NodeIndex), S>,
    clique_graph_map: &HashMap<NodeIndex, HashSet<NodeIndex, S>, S>,
    node_index_map: &HashMap<NodeIndex, NodeIndex, S>,
) -> (NodeIndex, NodeIndex) {
    *currently_interesting_vertices
        .iter()
        .min_by_key(|(vertex_res_graph, interesting_vertex_clique_graph)| {
            // Clone result graph
            let mut result_graph: Graph<HashSet<NodeIndex, S>, O, Undirected> = result_graph.clone();

            // Update result graph
            let cheapest_new_vertex_res = result_graph.add_node(
                clique_graph
                    .node_weight(*interesting_vertex_clique_graph)
                    .expect("Vertices in clique graph should have bags as weights")
                    .clone(),
            );

            result_graph.add_edge(
                *vertex_res_graph,
                cheapest_new_vertex_res,
                O::default(),
            );

            fill_bags_from_result_graph(
                &mut result_graph,
                cheapest_new_vertex_res,
                *vertex_res_graph,
                clique_graph_map,
                node_index_map
            );

            // Find treewidth (biggest bag size) of 
            crate::find_width_of_tree_decomposition::find_width_of_tree_decomposition(&result_graph)
        }).expect("There should be interesting vertices since there are vertices left and the graph is connected")
}
