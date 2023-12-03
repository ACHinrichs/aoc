use colored::Colorize;
use regex::Regex;
use std::cmp;
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
        let mut part_nums: Vec<u64> = Vec::new();
        let re = Regex::new(r"[0-9]+").unwrap(); //if all else fails, just slap it with a regex       println!("{} -> {}", l, new_line);
        for (i, l) in lines.iter().enumerate() {
            let mut last_end = 0;
            for cand_match in re.find_iter(l) {
                // check if there is a symbol touching the number

                // should match any character thats neither a . nor a digit
                let symbol_re = Regex::new(r"[^\.\d]").unwrap();

                print!("{}", l[last_end..cand_match.start()].to_string());

                if ((i > 0)
                    && symbol_re.is_match(
                        &lines[i - 1][(cmp::max(0, (cand_match.start() as i64) - 1) as usize)
                            ..cmp::min(cand_match.end() + 1, lines[i - 1].len())]
                            .to_string(),
                    ))
                    || ((i + 1 < lines.len())
                        && symbol_re.is_match(
                            &lines[i + 1][(cmp::max(0, (cand_match.start() as i64) - 1) as usize)
                                ..cmp::min(cand_match.end() + 1, lines[i + 1].len())]
                                .to_string(),
                        ))
                    || ((cand_match.start() > 0)
                        && symbol_re.is_match(&l[(cand_match.start() - 1)..(cand_match.start())]))
                    || ((cand_match.end() + 1 < l.len())
                        && symbol_re.is_match(&l[cand_match.end()..(cand_match.end() + 1)]))
                {
                    //if we are here, the candidate is a part-number
                    part_nums.push(
                        cand_match
                            .as_str()
                            .parse()
                            .expect("You matched a 'number' that is not a number, you fool!"),
                    );
                    print!("{}", cand_match.as_str().green());
                } else {
                    print!("{}", cand_match.as_str().red());
                }
                last_end = cand_match.end();
            }
            print!("{}", l[last_end..].to_string());
            print!("\n");
        }
        println!(
            "The sum of part numbers is {}",
            part_nums.iter().sum::<u64>()
        );
    } else if task == "2" {
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
