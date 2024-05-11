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

    let computation_type: TreewidthComputationMethod =
        TreewidthComputationMethod::MSTAndUseTreeStructure;
    let heuristic = treewidth_heuristic::negative_intersection_heuristic;

    let mut benchmark_log_file =
        File::create("dimacs_benchmarks/benchmark_results/dimacs_results.txt")
            .expect("Dimacs log file should be creatable");

    // Sorting files in dimacs directory
    let dimacs_graphs_paths: fs::ReadDir = fs::read_dir("dimacs_graphs/color/").unwrap();
    let mut dimacs_graph_paths_vec = Vec::new();
    for graph_path_res in dimacs_graphs_paths {
        if let Ok(graph_path) = graph_path_res {
            if let Some(extension) = graph_path.path().extension() {
                if extension == "col" {
                    dimacs_graph_paths_vec.push(graph_path);
                }
            }
        }
    }
    dimacs_graph_paths_vec.sort_by_key(|e| e.file_name());

    for graph_path in dimacs_graph_paths_vec {
        let graph_file_name = graph_path.file_name();
        let graph_file =
            File::open(graph_path.path()).expect("Graph file should exist and be readable");

        let (graph, _, _, _): (Graph<i32, i32, petgraph::prelude::Undirected>, _, _, _) =
            read_graph(graph_file).expect("Graph should be in correct format");

        println!("Starting calculation on graph: {:?}", graph_file_name);
        // Time the calculation
        let start = SystemTime::now();
        let computed_treewidth = compute_treewidth_upper_bound_not_connected::<_, _, Hasher, _>(
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
