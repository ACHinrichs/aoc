use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::Instant;

struct Item {
    is_list: bool,
    value: u32,
    contents: Vec<Item>,
}

impl Item {
    // Returns: constructed item and the new index where the string starts (e.g. the first char that was not consumed during parsing)
    fn parse(s: &str) -> (Self, usize) {
        let re_digit = Regex::new(r"^\d+").unwrap();
        let mat_optn = re_digit.find(s);
        if mat_optn.is_some() {
            let mat = mat_optn.unwrap();
            let mut res = Item {
                is_list: false,
                value: 0,
                contents: Vec::new(),
            };
            res.value = s[mat.start()..mat.end()].parse::<u32>().unwrap();
            /*println!(
                "Parsing string '{}', found '{}' = {}, remaining is '{}'",
                s,
                &s[mat.start()..mat.end()],
                res.value,
                &s[mat.end()..]
            );*/
            return (res, mat.end());
        } else if s.starts_with("[") {
            let mut index: usize = 1; // yes, that is correct, thereby we ignore the starting [
            let mut res = Item {
                is_list: true,
                value: 0,
                contents: Vec::new(),
            };
            loop {
                if index >= s.len() {
                    println!("Aborted because we ran out of string");
                    //dbg!(s, index, s.len());
                    break;
                }
                if &s[index..index + 1] == "," {
                    //println!("Remove ,");
                    index += 1;
                    continue;
                }
                if &s[index..index + 1] == "]" {
                    //println!("Found ]");
                    //dbg!(s, index, &s[index..]);
                    break;
                }
                let (parsed, new_index) = Item::parse(&s[index..]);
                res.contents.push(parsed);
                //dbg!(new_index, index, &s[index..], &s[index..index]);
                index += new_index;
            }
            return (res, index + 1);
        } else if s.starts_with(",") {
            return Self::parse(&s[1..]);
        } else if s.starts_with("]") {
            //This may be an error, i'm not sure
            println!("found ], should this happen?");
            return Self::parse(&s[1..]);
        }
        return (
            Item {
                is_list: true,
                value: 0,
                contents: Vec::new(),
            },
            0,
        );
    }

    fn print(&self) {
        if self.is_list {
            print!("[");
            for c in &self.contents {
                c.print();
                print!(",");
            }
            print!("],");
        } else {
            print!("{}", self.value);
        }
    }
}

fn main() {
    println!("AOC 2022, Day 13 – Distress Signal\n");
    // Shave of the initial text, cause I'm a cheater :P
    let timing_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<String>>();

    let timing_after_fileread = Instant::now();

    let parsed = lines
        .into_iter()
        .map(|l| {
            let (s, i) = Item::parse(&l);
            s
        })
        .collect::<Vec<Item>>();
    for p in parsed {
        p.print();
        print!("\n");
    }
    if task == "1" {
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
    println!(
        "Execution time: {} µs\nExecution time without file-reading: {} µs",
        timing_start.elapsed().as_micros(),
        timing_after_fileread.elapsed().as_micros(),
    );
}
