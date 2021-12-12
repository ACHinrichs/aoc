use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{EdgeReference, Graph, NodeIndex};
use petgraph::visit::{Bfs, EdgeRef};

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Cave {
    name: String,
    is_large: bool,
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines: Vec<Vec<String>> = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .split("-")
                .map(|y| y.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    let mut graph = Graph::<&String, i64>::new();
    for split in lines.iter() {
        if !graph.node_indices().any(|x| graph[x] == &split[0]) {
            graph.add_node(&split[0]);
        }
        if !graph.node_indices().any(|x| graph[x] == &split[1]) {
            graph.add_node(&split[1]);
        }
        let i_a = graph
            .node_indices()
            .find_map(|i| if graph[i] == &split[0] { Some(i) } else { None })
            .unwrap();
        let i_b = node_index(&graph, &split[1]);
        if (split[1] != "start") && (split[0] != "end") {
            graph.add_edge(i_a, i_b, 0);
        }
        if (split[0] != "start") && (split[1] != "end") {
            graph.add_edge(i_b, i_a, 0);
        }
    }
    let start = node_index(&graph, &"start".to_string());
    let end = node_index(&graph, &"end".to_string());

    fs::write(
        "graph.dot",
        format!("{}", Dot::with_config(&graph, &[Config::EdgeNoLabel])),
    );

    let paths = modified_dfs(&graph, start, end, None, &mut Vec::<NodeIndex>::new());
    println!("Number of paths is {}", paths)
}

fn node_index<N: std::cmp::PartialEq, T>(graph: &Graph<N, T>, s: N) -> NodeIndex {
    (*graph)
        .node_indices()
        .find_map(|i| if graph[i] == s { Some(i) } else { None })
        .unwrap()
}

fn is_upper(s: &String) -> bool {
    s.to_uppercase() == *s
}

fn modified_dfs(
    graph: &Graph<&String, i64>,
    s: NodeIndex,
    t: NodeIndex,
    allow_double: Option<NodeIndex>,
    visited: &mut Vec<NodeIndex>,
) -> i64 {
    if s == t {
        return 1;
    }
    if (allow_double.is_none() && visited.contains(&s))
        || (allow_double.is_some() && allow_double.unwrap() != s && visited.contains(&s))
        || (allow_double.is_some()
            && allow_double.unwrap() == s
            && visited.iter().fold(0, |b, x| {
                if x == &allow_double.unwrap() {
                    b + 1
                } else {
                    b
                }
            }) > 1)
    {
        println!("returning from {:?}", s);
        return 0;
    }
    //visited.push(s);
    println!("{:?}", visited);
    let mut res = 0;

    if !is_upper(graph[s]) {
        visited.push(s);
    }

    for n in graph.edges(s) {
        res = res + modified_dfs(graph, n.target(), t, allow_double, visited);
        if allow_double.is_none() && visited.contains(&n.target()) {
            println!("Allowing {:?}", n.target());
            res = res + modified_dfs(graph, n.target(), t, Some(n.target()), visited);
        }
    }

    if visited.contains(&s) {
        assert_eq!(visited.pop().unwrap(), s);
    }

    return res;
}
