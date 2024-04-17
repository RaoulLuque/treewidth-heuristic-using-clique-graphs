use std::collections::HashSet;

use petgraph::graph::NodeIndex;
use petgraph::visit::IntoNodeReferences;
use petgraph::Graph;

pub fn construct_clique_graph<InnerCollection, OuterIterator>(
    cliques: OuterIterator,
) -> Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected>
where
    OuterIterator: IntoIterator<Item = InnerCollection>,
    InnerCollection: IntoIterator<Item = NodeIndex>,
{
    let mut result_graph: Graph<HashSet<NodeIndex>, i32, petgraph::prelude::Undirected> =
        Graph::new_undirected();
    for clique in cliques {
        let vertex_index = result_graph.add_node(HashSet::from_iter(clique.into_iter()));
        let mut edges_to_be_added = Vec::new();
        for (other_vertex_index, other_vertex_weight) in result_graph.node_references() {
            if other_vertex_index == vertex_index {
                continue;
            } else {
                if let Some(_) = result_graph
                    .node_weight(vertex_index)
                    .expect("Node weight - in this case the nodes in the clique - should exist")
                    .intersection(other_vertex_weight)
                    .next()
                {
                    // Add edge, if cliques (that are the nodes of result graph) have nodes in common
                    edges_to_be_added.push(other_vertex_index);
                }
            }
        }
        for other_vertex_index in edges_to_be_added.iter() {
            result_graph.add_edge(vertex_index, *other_vertex_index, 0);
        }
    }

    result_graph
}
