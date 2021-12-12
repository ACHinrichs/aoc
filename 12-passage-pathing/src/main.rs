use petgraph::graph::{Graph, NodeIndex, EdgeReference};
use petgraph::data::FromElements;
use petgraph::visit::{Bfs, EdgeRef};
use petgraph::dot::{Dot, Config};

use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;

fn main() {
    let file = File::open("example_10.txt").expect("file not found");
    let lines: Vec<Vec<String>> = BufReader::new(file)
        .lines().map(|x| x.unwrap()
		     .split("-")
		     .map(|y| y.to_string())
		     .collect::<Vec<String>>()).collect();

    let mut graph = Graph::<&String, i64>::new();
    for split in lines.iter(){
	if !graph.node_indices().any(|x| graph[x] == &split[0] ){
	    graph.add_node(&split[0]);
	}
	if !graph.node_indices().any(|x| graph[x] == &split[1]) {
	    graph.add_node(&split[1]);
	}
	let i_a = graph.node_indices().find_map(|i| if graph[i] == &split[0] {Some(i)} else {None}).unwrap();
	let i_b = node_index(&graph, &split[1]);
	graph.add_edge(i_a, i_b, i64::MAX);
	graph.add_edge(i_b, i_a, i64::MAX);
    }
 
    fs::write("graph.dot",format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel])));    
    let mut graph_for_flow = Graph::<String, (i64, i64)>::new();
    // Create subgraphs for Node-restricitons
    for node in graph.node_indices(){
	
	if is_upper(graph[node])
	    || graph[node] == "start"
	    || graph[node] == "end"{
	    // It is a unrestricted node, so we can just copy it
	    graph_for_flow.add_node(format!("{}", graph[node])); // This is probably very dirty
	} else {
	    // Node with restriction 1
	    let i_start = graph_for_flow.add_node(format!("{}_1", graph[node]));
	    let i_end = graph_for_flow.add_node(format!("{}_2", graph[node]));
	    graph_for_flow.add_edge(i_start, i_end, (1,0));
	}
    }

}

fn node_index<N: std::cmp::PartialEq, T>(graph: &Graph<N,T>, s: N) -> NodeIndex{
    (*graph).node_indices().find_map(|i| if graph[i] == s {Some(i)} else {None}).unwrap()
}

fn is_upper(s: &String) -> bool{
    s.to_uppercase() == *s
}
