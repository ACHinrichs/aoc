use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_char(c: char) -> i64 {
    return match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => i64::MAX,
    };
}

fn main() {
    println!("AOC 2022, Day 02 – Rock, Paper, Scisors\n");
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
    let mut score = 0;
    if task == "1" {
        for l in lines {
            let oponent = parse_char(l.chars().nth(0).unwrap());
            let own = parse_char(l.chars().nth(2).unwrap());

            score += own;
            if oponent == own {
                score += 3;
            } else if (oponent == 1 && own == 2)
                || (oponent == 2 && own == 3)
                || (oponent == 3 && own == 1)
            {
                score += 6;
            }
        }
    } else if task == "2" {
        for l in lines {
            let opponent = parse_char(l.chars().nth(0).unwrap());
            let own = match l.chars().nth(2).unwrap() {
                'X' => ((opponent - 1) + 2) % 3 + 1,
                'Y' => opponent,
                'Z' => ((opponent - 1) + 1) % 3 + 1,
                _ => i64::MAX,
            };

            score += own;
            if opponent == own {
                score += 3;
            } else if (opponent == 1 && own == 2)
                || (opponent == 2 && own == 3)
                || (opponent == 3 && own == 1)
            {
                score += 6;
            }
        }
    } else {
        panic!("Task unknown, please specify as first argument")
    }
    println!("Your score is {}", score);
}
