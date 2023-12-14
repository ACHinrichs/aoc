use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn perform_spin_cycle(inp: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let res = inp.clone();
    return res;
}

fn main() {
    println!("AOC 2023, Day 14 – Parabolic Reflector Dish\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let mut lines: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    // Add padding to each line (start and end)
    for l in &mut lines {
        l.insert(0, '#');
        l.push('#');
    }

    // Add a row of padding
    lines.insert(0, vec!['#'; lines[0].len()]);
    lines.push(vec!['#'; lines[0].len()]);

    if task == "1" {
        // just iterate over each char, if it is a 0 and over it is a . move it
        // no need to look at edgecases, because we added padding
        let mut changes = true;
        while changes {
            changes = false;
            for y in 0..lines.len() {
                for x in 0..lines[y].len() {
                    if lines[y][x] == 'O' && lines[y - 1][x] == '.' {
                        // move rock up
                        lines[y][x] = '.';
                        lines[y - 1][x] = 'O';
                        changes = true;
                    }
                }
            }
        }

        // calculate load
        let mut load = 0;
        for y in 0..lines.len() {
            print!("\n");
            for x in 0..lines[y].len() {
                print!("{}", lines[y][x]);
                if lines[y][x] == 'O' {
                    load += lines.len() - y - 1;
                }
            }
        }

        println!("The load is {}", load);
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
