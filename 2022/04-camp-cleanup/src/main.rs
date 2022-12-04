use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Section {
    start: u64,
    end: u64,
}

impl Section {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn new_from_str(input: &str) -> Self {
        let values: Vec<&str> = input.split("-").collect();
        Self {
            start: (values[0].parse::<u64>().unwrap()),
            end: (values[1].parse::<u64>().unwrap()),
        }
    }

    pub fn complete_overlap(&self, other: &Section) -> bool {
        return (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end);
    }
}

fn main() {
    println!("AOC 2022, Day 04 – Camp-Cleanup\n");
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
    let mut pairs = lines
        .into_iter()
        .map(|x| {
            x.split(",")
                .map(|s| Section::new_from_str(s))
                .collect::<Vec<Section>>()
        })
        .collect::<Vec<Vec<Section>>>();
    println!("{:?}", pairs);
    if task == "1" {
        pairs.retain(|x| x[0].complete_overlap(&x[1]));
        println!(
            "There are {} pairs where one section lies completly in the other",
            pairs.len()
        );
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
