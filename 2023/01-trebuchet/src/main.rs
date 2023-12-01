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
            let mut changes = true;
            let mut precedence: i64 = 1;
            let mut new_line = l.to_string();
            while changes {
                changes = false;
                let mut first_pos = (new_line.len() + 1); // GOES TITS-UP IF YOU USE USIZE::MAX
                let mut first_precedence = first_pos as i64; // yes, we need this, because first_pos is not initilized correctly for our first comparison with precedence -1, it is correct that this is initially always positive
                let mut replace_by = 0;
                for (i, replace_string) in replace_strings.iter().enumerate() {
                    let substr = new_line.find(replace_string);
                    if substr.is_some() {
                        //println!("Found {}", replace_string);
                        //println!("{} {}", first_pos, replace_by);
                        //println!(
                        //    "{} {}",
                        //    (substr.unwrap() as i64) * precedence,
                        //    (first_pos as i64) * precedence
                        //);
                        if (substr.unwrap() as i64) * precedence < first_precedence {
                            println!("Found {}", replace_string);
                            first_precedence = (substr.unwrap() as i64) * precedence;
                            first_pos = substr.unwrap();
                            replace_by = i;
                        }
                    }
                }
                if first_pos < new_line.len() {
                    // ensure we actually found soemthing
                    println!(
                        "Replacing {} by {}",
                        &replace_strings[replace_by],
                        &replace_by.to_string()
                    );
                    new_line =
                        new_line.replacen(&replace_strings[replace_by], &replace_by.to_string(), 1);
                    changes = true;
                }
                if precedence == 1 {
                    // Quick hack, because the last digit in word form may overlap the
                    // digit before it, so we change the order of precedence by this
                    precedence = -1
                } else {
                    //changes = false;
                }
            }
            println!("{} -> {}", l, new_line);
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
