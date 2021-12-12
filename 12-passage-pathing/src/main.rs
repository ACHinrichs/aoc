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

    for edge in graph.edge_indices(){
	let (i_start, i_end) = graph.edge_endpoints(edge).unwrap();
	let mut label_start_old = graph[i_start];
	let mut label_start: String = format!("{}", label_start_old);
	let mut label_end_old = graph[i_end];
	let mut label_end: String = format!("{}", label_end_old);
	if !is_upper(label_start_old) && label_start_old != "start" && label_start_old != "end"{
	    label_start += "_2";
	}
	if !is_upper(&label_end_old) && label_end_old != "start"&& label_end_old != "end"{
	    label_end += "_1";
	}
	println!("{} {}", label_start, label_end);
	let mut capacity = i64::MAX;
	if label_start == "start" || label_end == "end"{
	    capacity = 1;
	}
	graph_for_flow.add_edge(node_index(&graph_for_flow, label_start),
				node_index(&graph_for_flow, label_end), (capacity,0));
    }

    fs::write("graph_for_flow.dot",format!("{:?}", Dot::with_config(&graph_for_flow, &[])));

    let source = node_index(&graph_for_flow, "start".to_string());
    let target = node_index(&graph_for_flow, "end".to_string());
    add_reverse_edges(&mut graph_for_flow);
    fs::write("graph_reverse.dot",format!("{:?}", Dot::with_config(&graph_for_flow, &[])));
    let max_flow = edmonds_karp(&mut graph_for_flow, source, target);

    println!("Max flow (result für P1) is {}", max_flow);
}

fn node_index<N: std::cmp::PartialEq, T>(graph: &Graph<N,T>, s: N) -> NodeIndex{
    (*graph).node_indices().find_map(|i| if graph[i] == s {Some(i)} else {None}).unwrap()
}

fn is_upper(s: &String) -> bool{
    s.to_uppercase() == *s
}

fn add_reverse_edges(graph: &mut Graph<String,(i64,i64)>){
    let mut to_add = Vec::new();
    for e in graph.edge_references(){
	let e_s = e.source();
	let e_t = e.target();
	let e_wf = e.weight();
	if e_wf.0 < i64::MAX{
	    to_add.push((e_t, e_s, (e_wf.0, e_wf.0)));
	}
    }
    graph.extend_with_edges(to_add.iter());
}
    
fn edmonds_karp(graph: &mut Graph<String,(i64,i64)>, s:NodeIndex, t:NodeIndex) -> i64{
    let mut flow = 0;
    
    loop{
	let mut q = Vec::<NodeIndex>::new();
	q.push(s);

	let mut pred = Vec::<Option< (NodeIndex, NodeIndex) >>::new();
	for i in graph.node_indices(){
	    pred.push(None)
	}
	    
	while !q.is_empty(){
	    let cur = q.pop().unwrap();
	    for e in graph.edges(cur){
		let e_t = graph.edge_endpoints(e.id()).unwrap().1;
		// edgewheight.0 is capacity, edgewheight.1 is flow
		if pred[e_t.index()].is_none() &&
		    e_t != s &&
		    graph[e.id()].0 > graph[e.id()].1
		{
		    pred[e_t.index()] = Some(graph.edge_endpoints(e.id()).unwrap());
		    q.insert(0, e_t);
		    //println!(".");
		}
	    }
	}
	if ! pred[t.index()].is_none(){
	    let mut df = i64::MAX;
	    {
		let mut e = pred[t.index()];
		while e.is_some(){
		    let e_ref = graph
			.edges_connecting(e.unwrap().0, e.unwrap().1)
			.nth(0).unwrap();
		    let e_cap = e_ref.weight().0;
		    let e_flow = e_ref.weight().1;
		    let df = std::cmp::min(df, e_cap - e_flow);
		    e = pred[e_ref.source().index()];
		    print!(" {}", df);
		}
		println!("");
	    }
	    {
		let mut e = pred[t.index()];
		while e.is_some(){
		    // Hin-Kante updaten
		    let e_reference = graph
			.edges_connecting(e.unwrap().0, e.unwrap().1)
			.nth(0).unwrap();
		    let e_cap = e_reference.weight().0;
		    let e_flow = e_reference.weight().1;
		    if e_cap != i64::MAX {
			graph.update_edge(e.unwrap().0,
					  e.unwrap().1,
					  (e_cap,
					   e_flow + df));
		    }
		    let e_rev_maybe = graph.edges_connecting(e.unwrap().1,
							     e.unwrap().0).last();
		    if e_rev_maybe.is_some(){
			let e_rev = e_rev_maybe.unwrap();
			graph.update_edge(e_rev.source(),
					  e_rev.target(),
					  (graph[e_rev.id()].0,
					   graph[e_rev.id()].1 - df));
		    }
                    //e.flow  := e.flow + df
                    //e.rev.flow := e.rev.flow - df
		    e = pred[e.unwrap().0.index()];
		}
	    }
	    println!("{:?}",pred);
	    flow = flow + df;
	}
	if pred[t.index()].is_none(){
	    return flow;
	}
    }
}
