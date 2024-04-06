mod algorithms;
use algorithms::find_maximum_cliques::find_maximum_cliques;
use petgraph::Graph;

fn main() {
    let mut graph: Graph<u32, u32, petgraph::prelude::Undirected> =
            petgraph::Graph::new_undirected();

    let nodes = [
        graph.add_node(0),
        graph.add_node(0),
        graph.add_node(0),
        graph.add_node(0),
        graph.add_node(0),
        graph.add_node(0),
        graph.add_node(0),
    ];

    graph.add_edge(nodes[0], nodes[1], 0);
    graph.add_edge(nodes[0], nodes[2], 0);
    graph.add_edge(nodes[0], nodes[5], 0);
    graph.add_edge(nodes[1], nodes[2], 0);
    graph.add_edge(nodes[1], nodes[3], 0);
    graph.add_edge(nodes[1], nodes[5], 0);
    graph.add_edge(nodes[2], nodes[5], 0);
    graph.add_edge(nodes[3], nodes[4], 0);
    graph.add_edge(nodes[3], nodes[5], 0);
    graph.add_edge(nodes[3], nodes[6], 0);
    graph.add_edge(nodes[4], nodes[6], 0);

    let mut cliques: Vec<Vec<_>> = find_maximum_cliques::<Vec<_>, _>(&graph).collect();

        
}
