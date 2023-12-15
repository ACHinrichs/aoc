use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct Lens {
    label: Vec<char>,
    focal_length: u64,
}

impl Lens {
    fn from_chars(s: &Vec<char>) -> Self {
        let mut label_vec = Vec::new();
        let mut focal_length = 42;
        for i in 0..s.len() {
            if s[i] == '-' || s[i] == '=' {
                break;
            }
            label_vec.push(s[i])
        }
        if s[s.len() - 2] == '=' {
            focal_length = s[s.len() - 1].to_digit(10).unwrap() as u64;
        }
        return Lens {
            label: label_vec,
            focal_length: focal_length,
        };
    }
}

fn hash_algo(s: &Vec<char>) -> usize {
    return s.iter().fold(0, |a, c| ((a as u64 + *c as u64) * 17) % 256) as usize;
}

fn main() {
    println!("AOC 2023, Day 15 – Lens Library\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let steps: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .nth(0)
        .unwrap()
        .expect("I don't know what to expect")
        .split(",")
        .map(|x| x.chars().collect())
        .collect();
    if task == "1" {
        println!(
            "The result is {:?}",
            steps.iter().map(hash_algo).map(|x| x as u64).sum::<u64>()
        );
    } else if task == "2" {
        let mut boxes: Vec<Vec<Lens>> = Vec::new();
        // add 256 boxes (e.g. vecs)
        for _i in 0..256 {
            boxes.push(Vec::new());
        }
        for s in steps {
            let new_lens = Lens::from_chars(&s);
            let box_ind = hash_algo(&new_lens.label);
            if s.contains(&'-') {
                boxes[box_ind] = boxes[box_ind]
                    .clone()
                    .into_iter()
                    .filter(|l| l.label != new_lens.label)
                    .collect()
            } else if s.contains(&'=') {
                let mut changed = false;
                for i in 0..boxes[box_ind].len() {
                    if boxes[box_ind][i].label == new_lens.label {
                        boxes[box_ind][i] = new_lens.clone();
                        changed = true;
                    }
                }
                if !changed {
                    boxes[box_ind].push(new_lens);
                }
            } else {
                panic!("At the disco");
            }
        }

        let mut res: u64 = 0;
        for (b_id, b) in boxes.iter().enumerate() {
            for (l_id, l) in b.iter().enumerate() {
                let focal_power = (b_id as u64 + 1) * (l_id as u64 + 1) * l.focal_length as u64;
                println!("{:?}: {}", l.label, focal_power);
                res += focal_power;
            }
        }
        println!("The sum of focal powers is {}", res);
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
