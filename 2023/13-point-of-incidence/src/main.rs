use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// Returns the index in the outer vec along which the vertical line of reflection lies
fn find_vertical(p: Vec<Vec<char>>, num_fixes: usize) -> Option<usize> {
    if p.len() == 0 {
        return None;
    }
    for i in 0..p.len() - 1 {
        // i is the row before the guessed line of reflection
        // that means if i is 4, the reflection is between indices 4 and 5
        let mut consistent = 0;
        let min_to_border = cmp::min(i + 1, p.len() - i - 1);
        //println!("{}", min_to_border);
        'outer: for j in 0..min_to_border {
            //println!("{} {}", i - j, i + j + 1);
            for k in 0..p[j].len() {
                if p[i - j][k] != p[i + j + 1][k] {
                    consistent += 1;
                    if consistent > num_fixes {
                        break 'outer;
                    }
                }
            }
        }
        // yes, we need to fix _exactly_ that many errors
        if consistent == num_fixes {
            return Some(i);
        }
    }
    return None;
}

// Returns the index inside the inner vec, after which the line of horizontal reflection lies
fn find_horizontal(p: Vec<Vec<char>>, num_fixes: usize) -> Option<usize> {
    if p.len() == 0 {
        return None;
    }
    for i in 0..p[0].len() - 1 {
        // i is the row before the guessed line of reflection
        // that means if i is 4, the reflection is between indices 4 and 5
        let mut consistent = 0;
        let min_to_border = cmp::min(i + 1, p[0].len() - i - 1);
        //println!("{}", min_to_border);
        'outer: for j in 0..min_to_border {
            for k in 0..p.len() {
                //println!("{} {}", i - j, i + j + 1);
                if p[k][i - j] != p[k][i + j + 1] {
                    consistent += 1;
                    if consistent > num_fixes {
                        break 'outer;
                    }
                }
            }
        }
        // yes, we need to fix _exactly_ that many errors
        if consistent == num_fixes {
            return Some(i);
        }
    }
    return None;
}

fn main() {
    println!("AOC 2023, Day 12 – Hot Springs\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    // parse lines to patterns (i.e. split the lines vec at empty lines)
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    let mut p: Vec<Vec<char>> = Vec::new();
    for l in lines {
        if l.len() == 0 {
            patterns.push(p);
            p = Vec::new();
        } else {
            p.push(l);
        }
    }
    patterns.push(p);

    let num_fixes;
    if task == "1" {
        num_fixes = 0;
    } else if task == "2" {
        num_fixes = 1;
    } else {
        panic!("Task unknown, please specify as first argument")
    }

    println!("Fixing exactly {} smudges per pattern", num_fixes);
    let mut sum: u64 = 0;

    for p in patterns {
        let res_h = find_horizontal(p.clone(), num_fixes);
        //println!("{:?}", res_h);
        let res_v = find_vertical(p.clone(), num_fixes);
        //println!("{:?}", res_v);
        assert!(!(res_h.is_some() && res_v.is_some()));
        if res_h.is_some() {
            sum += res_h.unwrap() as u64 + 1;
        }
        if res_v.is_some() {
            sum += (res_v.unwrap() as u64 + 1) * 100;
        };
    }
    println!("the sum is {}", sum)
}
