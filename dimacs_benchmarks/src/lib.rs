#[derive(Debug)]
pub enum HeuristicTypes {
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

pub enum EdgeWeightTypes<S> {
    ReturnI32(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> i32),
    ReturnI32Tuple(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> (i32, i32)),
}

use std::{collections::HashSet, hash::BuildHasher};

use petgraph::graph::NodeIndex;
use HeuristicTypes::*;
pub const HEURISTICS_BEING_TESTED: [HeuristicTypes; 8] = [
    MstTreeNi,
    FillWhileNi,
    MstTreeLd,
    FillWhileLd,
    MstTreeNiTLd,
    FillWhileNiTLd,
    MstTreeLdTNi,
    FillWhileLdTNi,
];

pub fn heuristic_to_edge_weight_heuristic<S: BuildHasher + Default>(
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

pub fn heuristic_to_computation_type(
    heuristic: &HeuristicTypes,
) -> treewidth_heuristic::TreewidthComputationMethod {
    use treewidth_heuristic::TreewidthComputationMethod::*;
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
