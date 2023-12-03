use colored::Colorize;
use regex::Regex;
use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_adjacently_matched(
    lines: &Vec<String>,
    re_candidate: Regex,
    re_adjacent: Regex,
) -> Vec<u64> {
    let mut res = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let mut last_end = 0;
        for cand_match in re_candidate.find_iter(l) {
            // check if there is a symbol touching the number
            print!("{}", l[last_end..cand_match.start()].to_string());
            if ((i > 0)
                && re_adjacent.is_match(
                    &lines[i - 1][(cmp::max(0, (cand_match.start() as i64) - 1) as usize)
                        ..cmp::min(cand_match.end() + 1, lines[i - 1].len())]
                        .to_string(),
                ))
                || ((i + 1 < lines.len())
                    && re_adjacent.is_match(
                        &lines[i + 1][(cmp::max(0, (cand_match.start() as i64) - 1) as usize)
                            ..cmp::min(cand_match.end() + 1, lines[i + 1].len())]
                            .to_string(),
                    ))
                || ((cand_match.start() > 0)
                    && re_adjacent.is_match(&l[(cand_match.start() - 1)..(cand_match.start())]))
                || ((cand_match.end() + 1 < l.len())
                    && re_adjacent.is_match(&l[cand_match.end()..(cand_match.end() + 1)]))
            {
                //if we are here, the candidate is a part-number
                res.push(
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
    return res;
}

fn get_adjacent_candidates<'a>(
    lines: &'a Vec<String>,
    gear_pos_x: usize, // where in the line is the *
    gear_pos_y: usize, // in which line is the *
    re: &'a Regex,
) -> Vec<regex::Match<'a>> {
    let mut res = Vec::new();
    for l in
        &lines[cmp::max(0, gear_pos_y as i32 - 1) as usize..cmp::min(gear_pos_y + 2, lines.len())]
    {
        for cand in re.find_iter(l) {
            //println!("{:?}", cand);
            // the gear_pos_x has to be somewhere between start-1 and end + 1
            /*println!(
                "{} <= {} <= {}",
                cand.start() as i64 - 1,
                gear_pos_x,
                cand.end()
            );*/
            if cand.start() as i64 - 1 <= gear_pos_x as i64 && cand.end() >= gear_pos_x {
                //println!("pushed");
                res.push(cand);
            }
        }
    }
    return res;
}

fn get_gear_ratios(lines: &Vec<String>, re_num: Regex, re_symbol: Regex) -> Vec<u64> {
    let mut res = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let mut last_end = 0;
        for cand_match in re_symbol.find_iter(l) {
            print!("{}", l[last_end..cand_match.start()].to_string());
            last_end = cand_match.end();
            let adj_nums = get_adjacent_candidates(lines, cand_match.start(), i, &re_num);
            if adj_nums.len() < 2 {
                print!("{}", cand_match.as_str().yellow());
            } else if adj_nums.len() == 2 {
                print!("{}", cand_match.as_str().green());
                res.push(
                    adj_nums
                        .into_iter()
                        .map(|a| a.as_str().parse::<u64>().unwrap())
                        .reduce(|a, b| a * b)
                        .unwrap(),
                );
            } else {
                print!("{}", cand_match.as_str().red());
            }
        }
        print!("{}", l[last_end..].to_string());
        print!("\n");
    }
    return res;
}

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

    let re = Regex::new(r"[0-9]+").unwrap(); //if all else fails, just slap it with a regex       println!("{} -> {}", l, new_line);

    if task == "1" {
        let re_adjacent = Regex::new(r"[^\.\d]").unwrap();
        let part_nums = get_adjacently_matched(lines, re, re_adjacent);
        println!(
            "The sum of part numbers is {}",
            part_nums.iter().sum::<u64>()
        );
    } else if task == "2" {
        let re_star = Regex::new(r"\*").unwrap();
        let gear_ratios = get_gear_ratios(lines, re, re_star);
        println!(
            "The sum of gear ratios is {}",
            gear_ratios.iter().sum::<u64>()
        );
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
