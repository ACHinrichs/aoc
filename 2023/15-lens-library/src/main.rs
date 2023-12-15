use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2023, Day 15 – Lens Library\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let steps: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .nth(0)
        .unwrap()
        .expect("I don't know what to expect")
        .split(",")
        .map(|x| x.chars().collect())
        .collect();
    if task == "1" {
        println!(
            "The result is {}",
            steps
                .iter()
                .map(|s| s.iter().fold(0, |a, c| ((a as u64 + *c as u64) * 17) % 256))
                .sum::<u64>()
        )
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
