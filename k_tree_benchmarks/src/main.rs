use std::collections::HashSet;
use std::fs::{self, File};
use std::io::Write;

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::time::SystemTime;
use treewidth_heuristic::{compute_treewidth_upper_bound, TreewidthComputationMethod};

// Debug version
#[cfg(debug_assertions)]
type Hasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;

// Non-debug version
#[cfg(not(debug_assertions))]
type Hasher = std::hash::RandomState;

fn main() {
    let k = 10;
    let n = 100;
    let p = 10;
    let edge_heuristic = treewidth_heuristic::least_difference_heuristic::<Hasher>;
    let number_of_trees = 100;
    let computation_type = TreewidthComputationMethod::FillWhilstMST;

    let mut sum_of_treewidths_predecessors = 0;
    let mut sum_of_treewidths_no_predecessors = 0;

    // Opening and writing to log file
    let mut benchmark_log_file =
        File::create("k_tree_benchmarks/benchmark_results/k_tree_results.txt")
            .expect("Dimacs log file should be creatable");

    for i in 0..number_of_trees {
        let graph: Graph<i32, i32, petgraph::prelude::Undirected> =
            treewidth_heuristic::generate_partial_k_tree_with_guaranteed_treewidth(
                k,
                n,
                p,
                &mut rand::thread_rng(),
            )
            .expect("n should be greater than k");

        println!("Starting calculation on graph number: {:?}", i);
        // Time the calculation
        let start = SystemTime::now();
        let (
            clique_graph,
            clique_graph_tree_after_filling_up,
            clique_graph_tree_before_filling_up,
            _,
            _,
            computed_treewidth,
        ) = compute_treewidth_upper_bound(&graph, edge_heuristic, computation_type, true);

        // DEBUG
        let (
            clique_graph_alt,
            clique_graph_tree_after_filling_up_alt,
            clique_graph_tree_before_filling_up_alt,
            _,
            _,
            computed_treewidth_alt,
        ) = compute_treewidth_upper_bound(
            &graph,
            edge_heuristic,
            TreewidthComputationMethod::MSTAndFill,
            true,
        );

        // DEBUG
        // assert!(petgraph::algo::is_isomorphic_matching(
        //     &clique_graph,
        //     &clique_graph_alt,
        //     |a, b| a.eq(b),
        //     |a, b| a.eq(b)
        // ));

        sum_of_treewidths_no_predecessors += computed_treewidth_alt;
        sum_of_treewidths_predecessors += computed_treewidth;

        benchmark_log_file
            .write_all(
                format!(
                    "Graph {:?}: {} {} took {:.3} milliseconds\n",
                    i,
                    computed_treewidth,
                    computed_treewidth_alt,
                    start
                        .elapsed()
                        .expect("Time should be trackable")
                        .as_millis()
                )
                .as_bytes(),
            )
            .expect("Writing to Dimacs log file should be possible");

        // Debug
        // if i == 0 {
        //     println!(
        //         "Predecessor map first graph: \n {:?}",
        //         predecessor_map.unwrap()
        //     );
        //     println!(
        //         "Clique graph map first graph: \n {:?}",
        //         clique_graph_map.unwrap()
        //     );
        // }

        create_dot_files(
            &graph,
            &clique_graph,
            &clique_graph_tree_after_filling_up,
            &clique_graph_tree_before_filling_up,
            i,
            "",
        );

        // DEBUG
        // create_dot_files(
        //     &graph,
        //     &clique_graph_alt,
        //     &clique_graph_tree_after_filling_up_alt,
        //     &clique_graph_tree_before_filling_up_alt,
        //     i,
        //     "no_predecessors",
        // );
    }
    benchmark_log_file
        .write_all(
            format!(
                "Predecessors averaged: {:.2}; No Predecessors Average: {:.2}",
                sum_of_treewidths_predecessors as f32 / number_of_trees as f32,
                sum_of_treewidths_no_predecessors as f32 / number_of_trees as f32,
            )
            .as_bytes(),
        )
        .expect("Writing to Dimacs log file should be possible");
}

// Converting dot files to pdf in bulk:
// FullPath -type f -name "*.dot" | xargs dot -Tpdf -O
fn create_dot_files<S>(
    graph: &Graph<i32, i32, petgraph::prelude::Undirected>,
    clique_graph: &Graph<HashSet<NodeIndex, S>, Vec<i32>, petgraph::prelude::Undirected>,
    clique_graph_tree_after_filling_up: &Graph<
        HashSet<NodeIndex, S>,
        Vec<i32>,
        petgraph::prelude::Undirected,
    >,
    clique_graph_tree_before_filling_up: &Option<
        Graph<HashSet<NodeIndex, S>, Vec<i32>, petgraph::prelude::Undirected>,
    >,
    i: usize,
    name: &str,
) {
    fs::create_dir_all("k_tree_benchmarks/benchmark_results/visualizations")
        .expect("Could not create directory for visualizations");

    let start_graph_dot_file = Dot::with_config(graph, &[Config::EdgeNoLabel]);
    let result_graph_dot_file =
        Dot::with_config(clique_graph_tree_after_filling_up, &[Config::EdgeNoLabel]);
    let clique_graph_dot_file = Dot::with_config(&clique_graph, &[Config::EdgeNoLabel]);

    if let Some(clique_graph_tree_before_filling_up) = clique_graph_tree_before_filling_up {
        let clique_graph_tree_before_filling_up_dot_file =
            Dot::with_config(clique_graph_tree_before_filling_up, &[Config::EdgeNoLabel]);
        let clique_graph_node_indices = Dot::with_config(
            clique_graph_tree_before_filling_up,
            &[Config::EdgeNoLabel, Config::NodeIndexLabel],
        );

        let mut w = fs::File::create(format!(
            "k_tree_benchmarks/benchmark_results/visualizations/{}_result_graph_before_filling_{}.dot",
            i, name
        ))
        .expect("Result graph without filling up file could not be created");
        write!(&mut w, "{:?}", clique_graph_tree_before_filling_up_dot_file)
            .expect("Unable to write dotfile for result graph without filling up to files");

        let mut w = fs::File::create(format!(
            "k_tree_benchmarks/benchmark_results/visualizations/{}_result_graph_node_indices_{}.dot",
            i, name
        ))
        .expect("Clique graph node indices file could not be created");
        write!(&mut w, "{:?}", clique_graph_node_indices)
            .expect("Unable to write dotfile for Clique graph node indices  to files");
    }

    let mut w = fs::File::create(format!(
        "k_tree_benchmarks/benchmark_results/visualizations/{}_starting_graph_{}.dot",
        i, name
    ))
    .expect("Start graph file could not be created");
    write!(&mut w, "{:?}", start_graph_dot_file)
        .expect("Unable to write dotfile for start graph to files");

    let mut w = fs::File::create(format!(
        "k_tree_benchmarks/benchmark_results/visualizations/{}_clique_graph_{}.dot",
        i, name
    ))
    .expect("Start graph file could not be created");
    write!(&mut w, "{:?}", clique_graph_dot_file)
        .expect("Unable to write dotfile for start graph to files");

    let mut w = fs::File::create(format!(
        "k_tree_benchmarks/benchmark_results/visualizations/{}_result_graph_{}.dot",
        i, name
    ))
    .expect("Result graph file could not be created");
    write!(&mut w, "{:?}", result_graph_dot_file)
        .expect("Unable to write dotfile for result graph to files");
}
