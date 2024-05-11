use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::time::SystemTime;
use treewidth_heuristic::compute_treewidth_upper_bound_not_connected;

// Use imports for benchmarking from dimacs_benchmarks crate
use dimacs_benchmarks::*;

// Debug version
#[cfg(debug_assertions)]
type Hasher = std::hash::BuildHasherDefault<rustc_hash::FxHasher>;

// Non-debug version
#[cfg(not(debug_assertions))]
type Hasher = std::hash::RandomState;

/// First coordinate is the n, second k, third p
pub const PARTIAL_K_TREE_CONFIGURATIONS: [(usize, usize, usize); 18] = [
    (100, 10, 30),
    (100, 20, 30),
    (100, 10, 40),
    (100, 20, 40),
    (100, 10, 50),
    (100, 20, 50),
    (200, 10, 30),
    (200, 20, 30),
    (200, 10, 40),
    (200, 20, 40),
    (200, 10, 50),
    (200, 20, 50),
    (500, 10, 30),
    (500, 20, 30),
    (500, 10, 40),
    (500, 20, 40),
    (500, 10, 50),
    (500, 20, 50),
];

fn main() {
    // Opening log file
    let mut benchmark_log_file =
        File::create("k_tree_benchmarks/benchmark_results/k_tree_results.txt")
            .expect("Dimacs log file should be creatable");

    let number_of_repetitions_per_heuristic = 10;

    for (n, k, p) in PARTIAL_K_TREE_CONFIGURATIONS {
        let number_of_trees = 100;

        println!("Starting calculation on graph: {:?}", (n, k, p));
        let mut calculation_vec = Vec::new();

        for i in 0..number_of_trees {
            let graph: Graph<i32, i32, petgraph::prelude::Undirected> =
                treewidth_heuristic::generate_partial_k_tree_with_guaranteed_treewidth(
                    k,
                    n,
                    p,
                    &mut rand::thread_rng(),
                )
                .expect("n should be greater than k");

            for heuristic_index in 0..HEURISTICS_BEING_TESTED.len() {
                // Time the calculation
                let start = SystemTime::now();
                let mut treewidth: usize = usize::MAX;

                let heuristic = &HEURISTICS_BEING_TESTED[heuristic_index];
                let edge_weight_heuristic = heuristic_to_edge_weight_heuristic(heuristic);
                let computation_type = heuristic_to_computation_type(heuristic);

                for _ in 0..number_of_repetitions_per_heuristic {
                    let computed_treewidth = match edge_weight_heuristic {
                        EdgeWeightTypes::ReturnI32(a) => {
                            compute_treewidth_upper_bound_not_connected::<_, _, Hasher, _>(
                                &graph,
                                a,
                                computation_type,
                                false,
                            )
                        }
                        EdgeWeightTypes::ReturnI32Tuple(a) => {
                            compute_treewidth_upper_bound_not_connected::<_, _, Hasher, _>(
                                &graph,
                                a,
                                computation_type,
                                false,
                            )
                        }
                    };

                    if computed_treewidth < treewidth {
                        treewidth = computed_treewidth;
                    }
                }

                if i == 0 {
                    calculation_vec.push((
                        treewidth,
                        start
                            .elapsed()
                            .expect("Time should be trackable")
                            .as_millis()
                            / number_of_repetitions_per_heuristic,
                    ))
                } else {
                    let (treewidth_sum, time_sum) = calculation_vec
                        .get(heuristic_index)
                        .expect("Values for calculation should exist");
                    calculation_vec[heuristic_index] = (
                        treewidth_sum + treewidth,
                        time_sum
                            + start
                                .elapsed()
                                .expect("Time should be trackable")
                                .as_millis()
                                / number_of_repetitions_per_heuristic,
                    );
                }
            }
        }
        let calculation_vec: Vec<(f32, f32)> = calculation_vec
            .iter()
            .map(|(treewidth_sum, time_sum)| {
                (
                    *treewidth_sum as f32 / number_of_trees as f32,
                    *time_sum as f32 / number_of_trees as f32,
                )
            })
            .collect();

        let mut log = format!("");

        log.push_str(&format!("n: {} | k: {} | p: {} \n", n, k, p));

        log.push_str(&format!(
            "| {0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10} | {5: <10} | {6: <10} | {7: <10} | \n",
            "MTrNi", "FiWhNi", "MTrLd", "FillWhLd", "MTrNiTLd", "FiWhNiTLd", "MTrLdTNi", "FiWhLdTNi",
        ));

        log.push_str("|");
        for i in 0..HEURISTICS_BEING_TESTED.len() {
            let current_value_tuple = calculation_vec.get(i).expect("Calculation should exist");
            log.push_str(&format!(
                "{: <4} {: <7}|",
                format!("{:.1}", current_value_tuple.0),
                format!("{:.1}", current_value_tuple.1)
            ));
        }

        log.push_str("\n \n");

        benchmark_log_file
            .write_all(log.as_bytes())
            .expect("Writing to Dimacs log file should be possible");
    }
}

// Converting dot files to pdf in bulk:
// FullPath -type f -name "*.dot" | xargs dot -Tpdf -O
fn create_dot_files<O: Debug, S>(
    graph: &Graph<i32, i32, petgraph::prelude::Undirected>,
    clique_graph: &Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected>,
    clique_graph_tree_after_filling_up: &Graph<
        HashSet<NodeIndex, S>,
        O,
        petgraph::prelude::Undirected,
    >,
    clique_graph_tree_before_filling_up: &Option<
        Graph<HashSet<NodeIndex, S>, O, petgraph::prelude::Undirected>,
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
