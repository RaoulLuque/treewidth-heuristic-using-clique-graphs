use treewidth_heuristic::TreewidthComputationMethod::*;

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
    FillWhileTreeNiTLd,
    MstTreeLdTNi,
    FillWhileLdTNi,
}

impl std::fmt::Display for HeuristicTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let display_string = match self {
            MstTreeNi => "MTrNi",
            FillWhileNi => "FiWhNi",
            MstTreeLd => "MTrLd",
            FillWhileLd => "FiWhLd",
            MstTreeNiTLd => "MTrNiTLd",
            FillWhileNiTLd => "FiWhNiTLd",
            MstTreeLdTNi => "MTrLdTNi",
            FillWhileLdTNi => "FiWhLdTNi",
            FillWhileTreeNiTLd => "FWTNiTLd",
        };
        write!(f, "{}", display_string)
    }
}

pub enum EdgeWeightTypes<S> {
    ReturnI32(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> i32),
    ReturnI32Tuple(fn(&HashSet<NodeIndex, S>, &HashSet<NodeIndex, S>) -> (i32, i32)),
}

use std::{collections::HashSet, hash::BuildHasher};

use petgraph::graph::NodeIndex;
use HeuristicTypes::*;
// pub const HEURISTICS_BEING_TESTED: [HeuristicTypes; 8] = [
//     MstTreeNi,
//     FillWhileNi,
//     MstTreeLd,
//     FillWhileLd,
//     MstTreeNiTLd,
//     FillWhileNiTLd,
//     MstTreeLdTNi,
//     FillWhileLdTNi,
// ];

pub const HEURISTICS_BEING_TESTED: [HeuristicTypes; 2] = [MstTreeNi, MstTreeNiTLd]; //FillWhileNiTLd, FillWhileTreeNiTLd];

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
        FillWhileTreeNiTLd => {
            EdgeWeightTypes::ReturnI32Tuple(negative_intersection_then_least_difference_heuristic)
        }
    }
}

pub fn heuristic_to_computation_type(
    heuristic: &HeuristicTypes,
) -> treewidth_heuristic::TreewidthComputationMethod {
    match heuristic {
        MstTreeNi => MSTAndUseTreeStructure,
        FillWhileNi => FillWhilstMST,
        MstTreeLd => MSTAndUseTreeStructure,
        FillWhileLd => FillWhilstMST,
        MstTreeLdTNi => MSTAndUseTreeStructure,
        FillWhileLdTNi => FillWhilstMST,
        MstTreeNiTLd => MSTAndUseTreeStructure,
        FillWhileNiTLd => FillWhilstMST,
        FillWhileTreeNiTLd => FillWhilstMSTTree,
    }
}

pub fn heuristic_to_clique_bound(heuristic: &HeuristicTypes) -> Option<usize> {
    match heuristic {
        MstTreeNi => None,
        FillWhileNi => None,
        MstTreeLd => None,
        FillWhileLd => None,
        MstTreeLdTNi => None,
        FillWhileLdTNi => None,
        MstTreeNiTLd => None,
        FillWhileNiTLd => None,
        FillWhileTreeNiTLd => None,
    }
}
