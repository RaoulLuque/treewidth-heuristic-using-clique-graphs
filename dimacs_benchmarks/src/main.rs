use dimacs_petgraph_parser::read_graph;
use std::fs::{self, File};
use std::io::Write;

use petgraph::Graph;
use std::time::SystemTime;
use treewidth_heuristic::{
    compute_treewidth_upper_bound_not_connected, TreewidthComputationMethod,
};

// Debug version
#[cfg(debug_assertions)]
type Hasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;

// Non-debug version
#[cfg(not(debug_assertions))]
type Hasher = std::hash::RandomState;

fn main() {
    env_logger::init();

    let computation_type: TreewidthComputationMethod = TreewidthComputationMethod::FillWhilstMST;
    let heuristic = treewidth_heuristic::negative_intersection_heuristic;

    // Opening and writing to log file
    let mut benchmark_log_file =
        File::create("dimacs_benchmarks/benchmark_results/dimacs_results.txt")
            .expect("Dimacs log file should be creatable");

    // Testing dimacs_graph
    let dimacs_graphs_paths = fs::read_dir("dimacs_graphs/color/").unwrap();

    for graph_path_res in dimacs_graphs_paths {
        if let Ok(graph_path) = graph_path_res {
            if let Some(extension) = graph_path.path().extension() {
                if extension == "col" {
                    let graph_file_name = graph_path.file_name();
                    let graph_file = File::open(graph_path.path())
                        .expect("Graph file should exist and be readable");

                    let (graph, _, _, _): (
                        Graph<i32, i32, petgraph::prelude::Undirected>,
                        _,
                        _,
                        _,
                    ) = read_graph(graph_file).expect("Graph should be in correct format");

                    // DEBUG
                    // {
                    //     use treewidth_heuristic::*;

                    //     // Find cliques in initial graph
                    //     let cliques_one: Vec<Vec<_>> = itertools::Itertools::sorted(
                    //         find_maximum_cliques::<Vec<_>, _, Hasher>(&graph),
                    //     )
                    //     .collect();
                    //     let cliques_two: Vec<Vec<_>> = itertools::Itertools::sorted(
                    //         find_maximum_cliques::<Vec<_>, _, Hasher>(&graph),
                    //     )
                    //     .collect();

                    //     assert!(
                    //         cliques_one.eq(&cliques_two),
                    //         "Cliques one: {:?} and \n cliques two: {:?}",
                    //         cliques_one,
                    //         cliques_two
                    //     );

                    //     let clique_graph_one: Graph<_, _, _> =
                    //         construct_clique_graph(cliques_one, heuristic);
                    //     let clique_graph_two: Graph<_, _, _> =
                    //         construct_clique_graph(cliques_two, heuristic);
                    //     assert!(petgraph::algo::is_isomorphic_matching(
                    //         &clique_graph_one,
                    //         &clique_graph_two,
                    //         |a, b| a.eq(b),
                    //         |a, b| a.eq(b)
                    //     ));

                    //     let mut clique_graph_tree_one: Graph<
                    //         std::collections::HashSet<petgraph::prelude::NodeIndex, Hasher>,
                    //         i32,
                    //         petgraph::prelude::Undirected,
                    //     > = petgraph::data::FromElements::from_elements(
                    //         petgraph::algo::min_spanning_tree(&clique_graph_one),
                    //     );
                    //     let mut clique_graph_tree_two: Graph<
                    //         std::collections::HashSet<petgraph::prelude::NodeIndex, Hasher>,
                    //         i32,
                    //         petgraph::prelude::Undirected,
                    //     > = petgraph::data::FromElements::from_elements(
                    //         petgraph::algo::min_spanning_tree(&clique_graph_two),
                    //     );

                    //     let clique_graph_tree_before_filling_one = clique_graph_tree_one.clone();
                    //     let clique_graph_tree_before_filling_two = clique_graph_tree_two.clone();

                    //     fill_bags_along_paths(&mut clique_graph_tree_one);
                    //     fill_bags_along_paths(&mut clique_graph_tree_two);
                    // }

                    println!("Starting calculation on graph: {:?}", graph_file_name);
                    // Time the calculation
                    let start = SystemTime::now();
                    let computed_treewidth = compute_treewidth_upper_bound_not_connected::<
                        _,
                        _,
                        Hasher,
                    >(
                        &graph, heuristic, computation_type, false
                    );

                    benchmark_log_file
                        .write_all(
                            format!(
                                "{:?}: {} took {:.3} milliseconds\n",
                                graph_file_name,
                                computed_treewidth,
                                start
                                    .elapsed()
                                    .expect("Time should be trackable")
                                    .as_millis()
                            )
                            .as_bytes(),
                        )
                        .expect("Writing to Dimacs log file should be possible");
                }
            }
        }
    }
}
