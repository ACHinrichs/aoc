use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
fn main() {
    println!("AOC 2022, Day 03 – Rucksack Reorganization\n");
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
        let compartments: Vec<Vec<String>> = lines
            .into_iter()
            .map(|line| {
                vec![
                    line.get(0..line.len() / 2).unwrap().to_owned(),
                    line.get(line.len() / 2..).unwrap().to_owned(),
                ]
            })
            .collect();
        let mut compartments_sortet: Vec<Vec<VecDeque<char>>> = compartments //WHat the fuck
            .into_iter()
            .map(|c_vec| {
                c_vec
                    .into_iter()
                    .map(|c| {
                        let mut cs = c.chars().collect::<Vec<char>>();
                        cs.sort();
                        VecDeque::from(cs)
                    })
                    .collect()
            })
            .collect();
        let mut contained_in_both: Vec<char> = Vec::new();
        let compartments_len = compartments_sortet.len();
        'a: for mut rucksack in compartments_sortet {
            while rucksack[0].len() > 0 && rucksack[1].len() > 0 {
                if rucksack[0][0] == rucksack[1][0] {
                    //if contained_in_both.len() <= 0
                    //    || *contained_in_both.get(contained_in_both.len() - 1).unwrap()
                    //        != rucksack[0][0]
                    //{
                    contained_in_both.push(rucksack[0][0]);
                    //}
                    continue 'a;
                    rucksack[0].pop_front();
                    rucksack[1].pop_front();
                } else if rucksack[0][0] > rucksack[1][0] {
                    rucksack[1].pop_front();
                } else if rucksack[0][0] < rucksack[1][0] {
                    rucksack[0].pop_front();
                }
            }
            if rucksack[0].len() == 0 || rucksack[1].len() == 0 {
                println!("That shoudl not have happened!");
            }
        }
        assert!(contained_in_both.len() == compartments_len);
        let prios: Vec<u64> = contained_in_both
            .into_iter()
            .map(|c| u64::from(c))
            .map(|o| {
                if o >= u64::from('a') {
                    o - u64::from('a') + 1
                } else {
                    o - u64::from('A') + 1 + 26
                }
            }) // Yes, i was too lazy to check the values
            .collect();
        println!("The Sum is {}", prios.into_iter().sum::<u64>());
    } else if task == "2" {
        let mut badges: Vec<char> = Vec::new();
        let mut lines_sorted: Vec<VecDeque<char>> = lines
            .into_iter()
            .map(|c| {
                let mut cs = c.chars().collect::<Vec<char>>();
                cs.sort();
                VecDeque::from(cs)
            })
            .collect();
        for i in 0..lines_sorted.len() / 3 {
            //Find badge of group
            loop {
                let starts = vec![
                    lines_sorted[i * 3][0],
                    lines_sorted[i * 3 + 1][0],
                    lines_sorted[i * 3 + 2][0],
                ];
                let mut cur_max = starts.iter().max().unwrap();
                let mut count = 0;
                for j in 0..3 {
                    if lines_sorted[i * 3 + j][0] != *cur_max {
                        lines_sorted[i * 3 + j].pop_front();
                    } else {
                        count += 1;
                    }
                }
                if count >= 3 {
                    badges.push(*cur_max);
                    break;
                }
            }
        }

        println!(
            "The Sum is {}",
            badges
                .into_iter()
                .map(|c| u64::from(c))
                .map(|o| {
                    if o >= u64::from('a') {
                        o - u64::from('a') + 1
                    } else {
                        o - u64::from('A') + 1 + 26
                    }
                }) // Yes, i was too lazy to check the values
                .sum::<u64>()
        );
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
