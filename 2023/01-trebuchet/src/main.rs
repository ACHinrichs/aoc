use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

//upper bound: 53386
// lowe bound: 53306

fn main() {
    println!("AOC 2023, Day 01 – Trebuchet\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let mut lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .collect::<Vec<String>>();
    let new_lines = &mut Vec::new();
    if task == "1" {
        // do nothing here, 2 needs only preprocessing
    } else if task == "2" {
        let replace_strings = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]; // zero is only there to provide an offset, if the solution does not work I should check if some asshole added zero to the inout where it should not be parsed!
        for l in lines.iter_mut() {
            let mut new_line = "".to_string();
            let re = Regex::new(r"[0-9]|one|two|three|four|five|six|seven|eight|nine").unwrap(); //if all else fails, just slap it with a regex       println!("{} -> {}", l, new_line);

            let mut offset = 0;
            let mut find_optn = re.find(l);
            while find_optn.is_some() {
                let find = find_optn.unwrap();
                offset += find.start() + 1;
                if replace_strings.contains(&find.as_str()) {
                    // do parsing
                    for (i, r) in replace_strings.iter().enumerate() {
                        if *r == find.as_str() {
                            new_line += &i.to_string();
                        }
                    }
                } else {
                    // just add it ot the new line
                    new_line += find.as_str();
                }
                find_optn = re.find(&l[offset..]);
            }
            println!("{} \t -> \t {} ", l, new_line);
            new_lines.push(new_line);
        }
        lines = new_lines;
    } else {
        panic!("Task unknown, please specify as first argument")
    }
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
}
