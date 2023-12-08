use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Mapnode<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Mapnode<'a> {
    fn from_line(line: &'a str) -> Self {
        let re = Regex::new(r"[0-9A-Z][0-9A-Z][0-9A-Z]").unwrap();
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        return Self {
            name: matches[0],
            left: matches[1],
            right: matches[2],
        }; // Very "parsing", but this is aoc after all
    }
}

fn main() {
    println!("AOC 2023, Day 08 – Haunted Wasteland\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .collect();
    let mut instructions = Vec::new();
    let mut nodes = HashMap::new();
    for l in lines.iter() {
        // again, very very dirty, but works on well-formed input
        if instructions.is_empty() {
            instructions = l.chars().collect();
        } else {
            if l != "" {
                let node = Mapnode::from_line(&l);
                //println!("{}", node.name);
                nodes.insert(node.name, node);
            }
        }
    }
    let mut sorted_nodes = nodes.keys().map(|s| *s).collect::<Vec<_>>();
    sorted_nodes.sort();
    if task == "1" {
        let start = sorted_nodes[0];
        let target = sorted_nodes[sorted_nodes.len() - 1];
        let mut cur = start;
        let mut steps = 0;
        while cur != target {
            let next_step = instructions[steps % instructions.len()]; //instruction roll over
                                                                      // println!("{}", cur);
            let cur_node = nodes.get(cur).expect("Current node does not exist, wth?");
            cur = match next_step {
                'L' => &cur_node.left,
                'R' => &cur_node.right,
                _ => "Non-Exisiting Node",
            };
            steps += 1;
        }
        println!(
            "We needed {} Steps to reach {} from {}",
            steps, target, start
        );
    } else if task == "2" {
        let starts: Vec<&str> = sorted_nodes
            .into_iter()
            .filter(|n| n.chars().last().unwrap() == 'A')
            .collect();
        let mut periods: Vec<HashMap<_, _>> = Vec::new();

        println!("Navigating on {} nodes in parallel", starts.len());
        for c in starts {
            println!("Navigating from Node {}", c);
            let mut steps = 0;
            let mut cur_node = c;
            periods.push(HashMap::new());
            loop {
                // that is intentional
                //instruction roll over
                let next_step = instructions[steps % instructions.len()];

                // println!("{}", cur);
                cur_node = nodes
                    .get(match next_step {
                        'L' => {
                            nodes
                                .get(cur_node)
                                .expect("Current node does not exist, wth?")
                                .left
                        }
                        'R' => {
                            nodes
                                .get(cur_node)
                                .expect("Current node does not exist, wth?")
                                .right
                        }
                        _ => "Non-Exisiting Node",
                    })
                    .expect("Next node does not exist, wth?")
                    .name;
                steps = steps + 1;

                // Write into HashSet, if node is a target node
                if cur_node.chars().last().unwrap() == 'Z' {
                    // Abort if
                    // - steps % instruction.len() is already in periods, AND we are on the same node as we were then
                    let mut abort = false;
                    let last_ind = periods.len() - 1;
                    for (k, v) in periods[last_ind].iter() {
                        if (*v == cur_node)
                            && (k % instructions.len() == steps % instructions.len())
                        {
                            abort = true
                        }
                    }
                    if abort {
                        break;
                    } else {
                        periods[last_ind].insert(steps, cur_node);
                        println!("Found node {} after {} steps", cur_node, steps);
                    }
                }
            }
            println!("Reached periodicity after {} steps", steps);
            println!("To get your result, use wolfram alpha to find the least common multiple of the steps above")
        }
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
