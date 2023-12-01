use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2023, Day 01 – Trebuchet\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .collect::<Vec<String>>();
    if task == "1" {
        let mut count = 0;
        for l in lines {
            let mut first = 0;
            let mut last = 0;
            for c in l.chars() {
                if ('0' <= c) && (c <= '9') {
                    if first == 0 {
                        first = (c as i32 - 0x30) * 10;
                    }
                    last = c as i32 - 0x30;
                }
            }
            count += first + last;
        }
        println!("The total sum of calibration values is {}", count)
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
