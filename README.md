# Treewidth Heuristic Using Clique Graphs

A library implementing a heuristic for computing an upper bound for the treewidth of arbitrary graphs using the clique graph operator for my bachelor thesis.

## Usage

This crate provides `compute_treewidth_upper_bound` and `compute_treewidth_upper_bound_not_connected` as functions.
They calculate upper bounds on the treewidth of connected and not connected undirected (pet)graphs respectively.

```rust
use treewidth_heuristic::compute_treewidth_upper_bound;
use treewidth_heuristic::negative_intersection_heuristic;
use treewidth_heuristic::TreewidthComputationMethod::FillWhilstMST;

use petgraph::graph::UnGraph;

// Create an undirected graph
let graph = UnGraph::<i32, ()>::from_edges(&[
    (1, 2), (2, 3), (3, 4),
    (1, 4)]);

// Compute treewidth using the negative intersection heuristic, 
// the FillWhilstMST computation method and not checking the 
// tree decomposition for correctness after the computation.
let treewidth_upper_bound = compute_treewidth_upper_bound(
    &graph,
    negative_intersection_heuristic,
    FillWhilstMST,
    false,
);
```