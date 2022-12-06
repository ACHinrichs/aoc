use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2022, Day 06 – Tuning-Trouble\n");
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

    for l in lines {
        //Just so that I can test all examples at once
        if task == "1" {
            for i in 3..l.len() {
                let cur = &mut l.chars().collect::<Vec<char>>()[i - 3..i + 1];
                if !(1..cur.len()).any(|i| cur[i..].contains(&cur[i - 1])) {
                    //Check for douplicate
                    println!(
                        "The first index to end an 4-letter non duplicate block is {}",
                        i + 1
                    );
                    break;
                }
            }
        } else if task == "2" {
            for i in 13..l.len() {
                let cur = &mut l.chars().collect::<Vec<char>>()[i - 13..i + 1];
                if !(1..cur.len()).any(|i| cur[i..].contains(&cur[i - 1])) {
                    //Check for douplicate
                    println!(
                        "The first index to end a 14-letter non duplicate block is {}",
                        i + 1
                    );
                    break;
                }
            }
        } else {
            panic!("Task unknown, please specify as first argument")
        }
    }
}
