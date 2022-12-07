use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_directory(
    lines: &mut dyn std::iter::Iterator<Item = String>,
    directories: &mut Vec<u64>,
) -> u64 {
    let mut size = 0;
    loop {
        let cur_line_maybe = lines.next();
        if cur_line_maybe.is_none() {
            break;
        }
        let cur_line = cur_line_maybe.unwrap();
        if cur_line.starts_with("$") {
            // Command
            if cur_line == "$ cd .." {
                break;
            } else if cur_line.starts_with("$ cd") {
                size += parse_directory(lines, directories);
            } else {
                // Should be ls, do nothing
                assert!(cur_line == "$ ls");
            }
        } else if cur_line.starts_with("dir") {
            // do nothing
        } else {
            //dbg!(cur_line.split(" ").collect::<Vec<&str>>());
            size += cur_line.split(" ").collect::<Vec<&str>>()[0]
                .parse::<u64>()
                .unwrap();
        }
    }
    directories.push(size);
    return size;
}

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

    let mut dirs = Vec::new();
    let total_size = parse_directory(&mut lines.into_iter(), &mut dirs);
    if task == "1" {
        println!(
            "The Sum of sizes of all directories <= 100000 is:\n{}",
            dirs.into_iter().filter(|d| *d <= 100000).sum::<u64>()
        );
    } else if task == "2" {
        let max_size = 70000000;
        let target_free = 30000000;
        let unused = max_size - total_size;
        let to_free = target_free - unused;
        println!(
            "The smallest directory to delete has a size of:\n{}",
            dirs.into_iter()
                .filter(|d| *d >= to_free)
                .reduce(|a, b| std::cmp::min(a, b))
                .unwrap()
        );
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
