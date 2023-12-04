use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Scratchcard {
    id: u64,
    winning_numbers: HashSet<u64>,
    numbers_we_have: HashSet<u64>,
}

impl Scratchcard {
    fn build_from_input(line: String) -> Self {
        let mut res = Self {
            id: 0,
            winning_numbers: HashSet::new(),
            numbers_we_have: HashSet::new(),
        };
        //The parsing is not very nice, but will do the trick
        res.id = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty()) // the nice alignment of 1-digit numbers breaks the code from day 2
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        res.winning_numbers = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("|")
            .nth(0)
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty()) // the nice alignment of 1-digit numbers breaks the code from day 2
            .map(|x| {
                x.parse::<u64>()
                    .expect("You tried to parse something thats not a number")
            })
            .collect();
        res.numbers_we_have = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("|")
            .nth(1) // Select second part
            .unwrap()
            .split(" ")
            .filter(|s| !s.is_empty()) // the nice alignment of 1-digit numbers breaks the code from day 2
            .map(|x| {
                x.parse::<u64>()
                    .expect("You tried to parse something thats not a number")
            })
            .collect();
        return res;
    }

    fn get_points(&self) -> u64 {
        let winning_cnt = self
            .numbers_we_have
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .collect::<Vec<&u64>>()
            .len() as u32;
        if winning_cnt == 0 {
            return 0;
        } else {
            return 2_u64.pow(winning_cnt - 1);
        }
    }
}

fn main() {
    println!("AOC 2023, Day 04 – Scratchcards\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let scratchcards = &mut BufReader::new(file)
        .lines()
        .map(|x| Scratchcard::build_from_input(x.unwrap().to_string()))
        .collect::<Vec<Scratchcard>>();

    if task == "1" {
        println!(
            "The number of points is {}",
            scratchcards
                .iter()
                .map(Scratchcard::get_points)
                .sum::<u64>()
        );
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
