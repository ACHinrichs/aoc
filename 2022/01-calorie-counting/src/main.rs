use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2022, Day 01 – Calorie Counting\n");
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
        // Find out how many calories the top 1 ekf is carrying
        let mut max_counted = 0;
        let mut cur_counted = 0;
        for l in lines {
            if l == "" {
                cur_counted = 0;
            } else {
                cur_counted += l.parse::<i64>().unwrap();
                if cur_counted > max_counted {
                    max_counted = cur_counted;
                }
            }
        }
        println!("The Elf carrying the most Calorie caries {}", max_counted);
    } else if task == "2" {
        let mut elf_list = Vec::new();
        elf_list.push(0);
        for l in lines {
            if l == "" {
                elf_list.push(0);
            } else {
                let len = elf_list.len();
                elf_list[len - 1] += l.parse::<i64>().unwrap();
            }
        }
        elf_list.sort();

        println!(
            "The top 3 elves are carrying a sum of {} Calories",
            elf_list[elf_list.len() - 1]
                + elf_list[elf_list.len() - 2]
                + elf_list[elf_list.len() - 3]
        );
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
