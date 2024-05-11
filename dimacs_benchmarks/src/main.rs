use dimacs_petgraph_parser::read_graph;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;
use std::fs::{self, File};
use std::hash::BuildHasher;
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

#[derive(Debug)]
#[allow(dead_code)]
enum HeuristicTypes {
    // MstTree = Minimum spanning tree then fill using tree structure
    // FillWhile = Fill while building minimum spanning tree
    // Ni = Negative Intersection
    MstTreeNi,
    FillWhileNi,
    // Ld = Least difference
    MstTreeLd,
    FillWhileLd,
    // T = Then
    MstTreeNiTLd,
    FillWhileNiTLd,
    MstTreeLdTNi,
    FillWhileLdTNi,
}

enum EdgeWeightTypes<S> {
    ReturnI32(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> i32),
    ReturnI32Tuple(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> (i32, i32)),
}

use HeuristicTypes::*;
const HEURISTICS_BEING_TEST: [HeuristicTypes; 8] = [
    MstTreeNi,
    FillWhileNi,
    MstTreeLd,
    FillWhileLd,
    MstTreeNiTLd,
    FillWhileNiTLd,
    MstTreeLdTNi,
    FillWhileLdTNi,
];

fn main() {
    env_logger::init();

    let number_of_repetitions_per_heuristic = 10;

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

    benchmark_log_file
        .write_all(
            format!(
                "| {0: <20} | {1: <12} | {2: <12} | {3: <12} | {4: <12} | {5: <12} | {6: <12} | {7: <12} | {8: <12} | {9: <12} | \n",
                "Graph name", "Upper bound", "MSTTreeNi", "FillWhileNi", "MSTTreeLd", "FillWhileLd", "MstTreeNiTLd", "FillWhileNiTLd",
                "MstTreeLdTNi", "FillWhileLdTNi",
            )
            .as_bytes(),
        )
        .expect("Writing to Dimacs log file should be possible");

    for graph_path in dimacs_graph_paths_vec {
        let graph_file_name = graph_path.file_name();
        let graph_file =
            File::open(graph_path.path()).expect("Graph file should exist and be readable");

        let (graph, _, _, upper_bound): (Graph<i32, i32, petgraph::prelude::Undirected>, _, _, _) =
            read_graph(graph_file).expect("Graph should be in correct format");

        println!("Starting calculation on graph: {:?}", graph_file_name);
        let mut calculation_vec = Vec::new();
        for heuristic in HEURISTICS_BEING_TEST {
            // Time the calculation
            let start = SystemTime::now();
            let mut treewidth: usize = usize::MAX;

            let edge_weight_heuristic = heuristic_to_edge_weight_heuristic(&heuristic);
            let computation_type = heuristic_to_computation_type(&heuristic);

            for i in 0..number_of_repetitions_per_heuristic {
                println!("Iteration: {} for heuristic: {:?}", i, heuristic);
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

            calculation_vec.push((
                treewidth,
                start
                    .elapsed()
                    .expect("Time should be trackable")
                    .as_millis()
                    / number_of_repetitions_per_heuristic,
            ))
        }

        let mut log = format!("");

        log.push_str(&format!(
            "| {0: <20} | {1: <12} |",
            graph_file_name
                .into_string()
                .expect("Graph file name should be utf8 string"),
            match upper_bound {
                Some(i) => i.to_string(),
                None => "None".to_string(),
            }
        ));

        for i in 0..HEURISTICS_BEING_TEST.len() {
            let current_value_tuple = calculation_vec.get(i).expect("Calculation should exist");
            log.push_str(&format!(
                "{0: <4} {1: <8} |",
                current_value_tuple.0, current_value_tuple.1
            ));
        }

        log.push_str("\n");

        benchmark_log_file
            .write_all(log.as_bytes())
            .expect("Writing to Dimacs log file should be possible");
    }
}

fn heuristic_to_edge_weight_heuristic<S: BuildHasher + Default>(
    heuristic: &HeuristicTypes,
) -> EdgeWeightTypes<S> {
    use treewidth_heuristic::*;
    use HeuristicTypes::*;
    match heuristic {
        MstTreeNi => EdgeWeightTypes::ReturnI32(negative_intersection_heuristic),
        FillWhileNi => EdgeWeightTypes::ReturnI32(negative_intersection_heuristic),
        MstTreeLd => EdgeWeightTypes::ReturnI32(least_difference_heuristic),
        FillWhileLd => EdgeWeightTypes::ReturnI32(least_difference_heuristic),
        MstTreeLdTNi => {
            EdgeWeightTypes::ReturnI32Tuple(least_difference_then_negative_intersection_heuristic)
        }
        FillWhileLdTNi => {
            EdgeWeightTypes::ReturnI32Tuple(least_difference_then_negative_intersection_heuristic)
        }
        MstTreeNiTLd => {
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference_heuristic)
        }
        FillWhileNiTLd => {
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference_heuristic)
        }
    }
}

fn heuristic_to_computation_type(heuristic: &HeuristicTypes) -> TreewidthComputationMethod {
    use TreewidthComputationMethod::*;
    match heuristic {
        MstTreeNi => MSTAndUseTreeStructure,
        FillWhileNi => FillWhilstMST,
        MstTreeLd => MSTAndUseTreeStructure,
        FillWhileLd => FillWhilstMST,
        MstTreeLdTNi => MSTAndUseTreeStructure,
        FillWhileLdTNi => FillWhilstMST,
        MstTreeNiTLd => MSTAndUseTreeStructure,
        FillWhileNiTLd => FillWhilstMST,
    }
}
