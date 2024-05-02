use std::fs::File;
use std::io::Write;

use petgraph::Graph;
use std::time::SystemTime;
use treewidth_heuristic::compute_treewidth_upper_bound_not_connected;

fn main() {
    let k = 20;
    let n = 300;
    let p = 0;

    // Opening and writing to log file
    let mut benchmark_log_file =
        File::create("k_tree_benchmarks/benchmark_results/k_tree_results.txt")
            .expect("Dimacs log file should be creatable");

    for i in 0..100 {
        let graph: Graph<i32, i32, petgraph::prelude::Undirected> =
            treewidth_heuristic::generate_partial_k_tree_with_guaranteed_treewidth(
                k,
                n,
                p,
                &mut rand::thread_rng(),
            )
            .expect("n is greater than k");

        println!("Starting calculation on graph number: {:?}", i);
        // Time the calculation
        let start = SystemTime::now();
        let computed_treewidth = compute_treewidth_upper_bound_not_connected(
            &graph,
            treewidth_heuristic::negative_intersection_heuristic,
            true,
            true,
        );

        benchmark_log_file
            .write_all(
                format!(
                    "Graph {:?}: {} took {:.3} milliseconds\n",
                    i,
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
