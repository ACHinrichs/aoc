use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2022, Day 07 – No Space Left on Device\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let mut lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<String>>();

    if task == "1" {
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
