use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("arguments: fire-hazard [input] [1|2]");
    }
    let task = &args[2];
    let file = File::open(&args[1]).expect(&format!("Inputfile {} not found", args[1]));
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|x| {
            if x.starts_with("//") {
                //Filter comments
                println!("Found comment in input: {}", &x[2..]);
                false
            } else {
                //Filter empty lines
                x != ""
            }
        })
        .collect::<Vec<String>>();

    let mut field = [[0; 1000]; 1000];

    for l in lines {
        //FInd all numbers
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        // iterate over all matches
        let coordinates = RE
            .find_iter(l)
            // try to parse the string matches as i64 (inferred from fn type signature)
            // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
            .filter_map(|digits| digits.as_str().parse::<usize>().ok())
            // collect the results in to a Vec<i64> (inferred from fn type signature)
            .collect::<Vec<usize>>();
        if task == "1" {
            if l.starts_with("toggle") {
                //toggle
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        field[x][y] = 1 - field[x][y]
                    }
                }
            } else if l.starts_with("turn on") {
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        field[x][y] = 1
                    }
                }
            } else if l.starts_with("turn off") {
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        field[x][y] = 0
                    }
                }
            }
        } else if task == "2" {
            if l.starts_with("toggle") {
                //toggle
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        //field[x][y] = 1 - field[x][y]
                        field[x][y] += 2
                    }
                }
            } else if l.starts_with("turn on") {
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        //field[x][y] = 1
                        field[x][y] += 1
                    }
                }
            } else if l.starts_with("turn off") {
                for x in coordinates[0]..=coordinates[2] {
                    for y in coordinates[1]..=coordinates[3] {
                        //field[x][y] = 0
                        if field[x][y] >= 1 {
                            field[x][y] -= 1
                        }
                    }
                }
            }
        }
    }
    // now count:
    let mut count = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            count += field[x][y];
        }
    }

    println!("Count is {}", count);
}
