use dimacs_petgraph_parser::read_graph;
use std::fs::{self, File};
use std::io::Write;

use petgraph::Graph;
use treewidth_heuristic::compute_treewidth_upper_bound_not_connected;

fn main() {
    // Opening and writing to log file
    let mut dimacs_log_file = File::create("benchmarks/benchmark_results/dimacs_results.txt")
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
                    let computed_treewidth = compute_treewidth_upper_bound_not_connected(&graph);

                    dimacs_log_file
                        .write_all(
                            format!("{:?}: {} \n", graph_file_name, computed_treewidth).as_bytes(),
                        )
                        .expect("Writing to Dimacs log file should be possible");
                }
            }
        }
    }

    // let cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(&graph).collect();
    // let mut clique_graph = construct_clique_graph(cliques);
    // fill_bags_along_paths(&mut clique_graph);
    // let computed_treewidth = find_width_of_tree_decomposition(&clique_graph);

    // println!("The computed treewidth is: {}", computed_treewidth);

    // let start_graph_dot_file = Dot::with_config(&graph, &[Config::EdgeNoLabel]);
    // let result_graph_dot_file = Dot::with_config(&clique_graph, &[Config::EdgeNoLabel]);

    // fs::create_dir_all("target/visualizations")
    //     .expect("Could not create directory for visualizations");
    // fs::write(
    //     "target/visualizations/starting_graph.dot",
    //     start_graph_dot_file.to_string(),
    // )
    // .expect("Unable to write dotfile for first graph to files");

    // let mut w = fs::File::create("target/visualizations/result_graph.dot")
    //     .expect("Result graph file could not be created");
    // write!(&mut w, "{:?}", result_graph_dot_file)
    //     .expect("Unable to write dotfile for result graph to files");
}
