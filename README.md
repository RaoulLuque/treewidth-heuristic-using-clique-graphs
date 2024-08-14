# Treewidth Heuristic Using Clique Graphs

A library implementing a heuristic for computing an upper bound for the treewidth of arbitrary graphs using the clique graph operator for my bachelor thesis.

## Idea of the heuristic
The heuristic computes a tree decomposition of a given graph by using the clique graph operator. First the maximal cliques of the given graph are computed which in general can take exponential time (and space) in the size of the graph since a graph can have up to an exponential number of maximal cliques in the number of vertices, see Moon and Moser graph. With these maximal cliques, the intersection graph is computed, that is, the clique graph of the original graph. For the heuristic, we identify the vertices in the clique graph with the maximal cliques they correspond to. Moreover we identify these maximal cliques with bags in a tree decomposition. Since a clique graph is in general not a tree, we compute a spanning tree of the clique graph. Afterwards we fill up all the necessary vertices in the bags by checking each pair of vertices/bags in the computed spanning tree and if they have a non-empty intersection and add this intersection to all bags along the path of the two vertices in the spanning tree. This way a tree decomposition is obtained.

In pseudocode this can be described as follows
```
def clique_graph_treewidth_heuristic(graph):
    clique_graph = construct_clique_graph(graph)        # step 1
    clique_graph_tree = min_spanning_tree(clique_graph) # step 2

    fill_bags_along_paths(clique_graph_tree)            # step 3

    treewidth = compute_treewidth(clique_graph_tree)    # step 4
    return treewidth
```
where the last step just computes the largest bag size and subtracts 1 from it according to the definition of the width of a tree decomposition.

## Usage

This crate mainly provides `compute_treewidth_upper_bound` and `compute_treewidth_upper_bound_not_connected` as functions.
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

## Benchmarks
Benchmarks are found in [this](https://github.com/RaoulLuque/treewidth-heuristic-clique-graph-benchmarks) repository.
