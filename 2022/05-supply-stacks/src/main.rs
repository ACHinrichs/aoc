use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2022, Day 05 – Supply-Stacks\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<String>>();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for l in lines {
        if l.starts_with("stack") {
            //Build new stack
            let mut cur_stack = l.split(" ").collect::<Vec<&str>>()[1]
                .chars()
                .collect::<Vec<char>>();
            stacks.push(cur_stack);
        } else if l.starts_with("move") {
            // command like: "move 5 from 3 to 6"
            let split = l.split(" ").collect::<Vec<&str>>();
            let count = split[1].parse::<usize>().unwrap();
            let from: usize = split[3].parse::<usize>().unwrap() - 1;
            let to: usize = split[5].parse::<usize>().unwrap() - 1;
            println!("Moving {}", count);
            if task == "1" {
                // print crates on top:
                for i in 0..count {
                    let tmp_crate = stacks[from].pop().unwrap();
                    stacks[to].push(tmp_crate);
                }
            } else if task == "2" {
                let mut tmp_stack: Vec<char> = Vec::new();
                for i in 0..count {
                    let tmp_crate = stacks[from].pop().unwrap();
                    tmp_stack.push(tmp_crate);
                }
                for i in 0..count {
                    stacks[to].push(tmp_stack.pop().unwrap());
                }
            } else {
                panic!("Task unknown, please specify as first argument")
            }
        } else {
            panic!("Unknown line!")
        }
    }
    println!("The following crates are on top of the stacks: ");
    for s in stacks {
        println!("{}", s.last().unwrap())
    }
}
