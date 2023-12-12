use memoize::memoize;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::thread;
/* Function to bruteforce the nubmer of possible solutions for each line.
Works in the following way:
- if the first group from the groups vec has size 0, remove it, got to next recrusion
- if the first group has size larger than 0, find the first group in the string, verify size is identical
-- if the a ? is found before the end of the group: recurse on both possible values.
-- if no ? is found, and groupsize matches: remove group both from line and groups, recurse
-- otherwise abort recursion
 */
#[memoize]
fn bruteforce_line_combinations(line: Vec<char>, groups: Vec<usize>) -> u64 {
    //println!("{:?} {:?}", line, groups);
    if groups.len() == 0 {
        if line.contains(&'#') {
            // we have no group left, but there are still broken springs
            // -> invalid solution, return 0
            return 0;
        } else {
            // we have used up all groups, and the remaining line only has '.' and '?'
            // -> valid solution, return 1
            //println!("abort with 1");
            return 1;
        }
    } else if line.len() == 0 {
        // We have groups, but no more text
        // -> invalid solution, return 0
        return 0;
    } else if line.len() <= (groups.iter().sum::<usize>() + groups.len()) {
        // Heuristic, maybe this helps
        // -> invalid solution, return 0
        return 0;
    } else if groups[0] == 0 {
        return bruteforce_line_combinations(line, groups[1..].to_vec());
    } else {
        // find first group
        let mut start = 0;
        let mut i = 0;
        while i < line.len() {
            if line[i] == '?' {
                //Recurse
                let mut with_dot = line[..i].to_vec();
                with_dot.push('.');
                with_dot.extend(line[i + 1..].to_vec());
                let mut with_GARTENZAUN = line[..i].to_vec();
                with_GARTENZAUN.push('#');
                with_GARTENZAUN.extend(line[i + 1..].to_vec());
                return bruteforce_line_combinations(with_dot, groups.clone())
                    + bruteforce_line_combinations(with_GARTENZAUN, groups.clone());
            } else if line[i] == '.' {
                if i == start {
                    // We have not reached the first group yet
                    i += 1;
                    start = i;
                    continue;
                } else {
                    // we are after the first group
                    if i - start == groups[0] {
                        // size of the group matches
                        return bruteforce_line_combinations(
                            line[i..].to_vec(),
                            groups[1..].to_vec(),
                        );
                    } else {
                        // groupsize does not match
                        return 0;
                    }
                }
            }
            if line[i] == '#' {
                i += 1;
            }
        }
        if start == i {
            // we have iterated over the end without finding a (possible) group,
            // this is an invalid solution because we still have groups we have to find
            return 0;
        }
        // we do not have to regard the case that the last group ends on the last char,
        // because we added one . as padding

        // iterated, did not find a group, return max_int, because this should never happen i hope
        println!("It happened");
        return u64::MAX;
    }
}

fn split_line(l: String) -> (Vec<char>, Vec<usize>) {
    let mut template: Vec<char> = l.split(" ").nth(0).unwrap().chars().collect();
    template.push('.'); // padding so that we do not have to regard the edge-case that a group ends with the end f the line
    let groups = l
        .split(" ")
        .nth(1)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    return (template, groups);
}

fn main() {
    println!("AOC 2023, Day 08 – Haunted Wasteland\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let mut lines: Vec<(Vec<char>, Vec<usize>)> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .map(split_line)
        .collect();
    if task == "1" {
        // no preprocessing
    } else if task == "2" {
        // everything times 5, i have a bad feeling about this
        let mut new_lines: Vec<(Vec<char>, Vec<usize>)> = Vec::new();
        for (mut template, groups) in lines {
            // remove padding:
            template.pop();
            let mut new_template = template.clone();
            let mut new_groups = groups.clone();
            for i in 1..5 {
                new_template.push('?');
                new_template.extend(template.clone());
                new_groups.extend(groups.clone());
            }
            new_template.push('.'); // add padding again
            new_lines.push((new_template, new_groups));
        }
        lines = new_lines;
    } else {
        panic!("Task unknown, please specify as first argument")
    }
    let mut sum = 0;
    let mut i = 0;
    let len = lines.len();
    for (template, groups) in lines {
        println!("{:?} {:?}", template.clone(), groups.clone());
        let count = bruteforce_line_combinations(template.clone(), groups.clone());
        println!("{}/{}: {}", i, len, count);
        sum += count;
        i += 1;
    }
    println!("\n the sum is {}", sum);
}
