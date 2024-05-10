use dimacs_petgraph_parser::read_graph;
use std::fs::{self, File};
use std::io::Write;

use petgraph::Graph;
use std::time::SystemTime;
use treewidth_heuristic::{
    compute_treewidth_upper_bound_not_connected, TreewidthComputationMethod,
};

fn main() {
    env_logger::init();

    let computation_type: TreewidthComputationMethod = TreewidthComputationMethod::FillWhilstMST;
    let heuristic = treewidth_heuristic::least_difference_heuristic;

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

                    let graph: Graph<i32, i32, petgraph::prelude::Undirected> =
                        read_graph(graph_file).expect("Graph should be in correct format");

                    println!("Starting calculation on graph: {:?}", graph_file_name);
                    // Time the calculation
                    let start = SystemTime::now();
                    let computed_treewidth = compute_treewidth_upper_bound_not_connected(
                        &graph,
                        heuristic,
                        computation_type,
                        false,
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
