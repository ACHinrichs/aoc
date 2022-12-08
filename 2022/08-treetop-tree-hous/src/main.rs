use colored::*;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("AOC 2022, Day 08 – Treetop Tree Houses\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let mut lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<String>>();
    let mut trees: Vec<Vec<u32>> = lines
        .into_iter()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() + 1)
                .collect::<Vec<u32>>()
        })
        .collect();
    // Add border of 0s:
    if task == "1" {
        let len_lines_without_border = trees.len();
        trees.insert(0, vec![0; len_lines_without_border]);
        trees.push(vec![0; len_lines_without_border]);
        trees = trees
            .into_iter()
            .map(|mut l| {
                l.insert(0, 0);
                l
            })
            .map(|mut l| {
                l.push(0);
                l
            })
            .collect();
        let mut visi_map: Vec<Vec<bool>> = Vec::new();
        // move from left to right and construct bool-map
        for y in 0..trees.len() {
            let mut max_height = 0;
            visi_map.push(vec![false; trees[y].len()]);
            for x in 0..trees[y].len() {
                if trees[y][x] > max_height {
                    max_height = trees[y][x];
                    visi_map[y][x] = true;
                }
            }
        }

        // move from right to left
        for y in 0..trees.len() {
            let mut max_height = 0;
            for x in (0..trees[y].len()).rev() {
                if trees[y][x] > max_height {
                    max_height = trees[y][x];
                    visi_map[y][x] = true;
                }
            }
        }

        // move from top to bottom
        for x in 0..trees[0].len() {
            let mut max_height = 0;
            for y in 0..trees.len() {
                if trees[y][x] > max_height {
                    max_height = trees[y][x];
                    visi_map[y][x] = true;
                }
            }
        }
        // move from bottom to top
        for x in 0..trees[0].len() {
            let mut max_height = 0;
            for y in (0..trees.len()).rev() {
                if trees[y][x] > max_height {
                    max_height = trees[y][x];
                    visi_map[y][x] = true;
                }
            }
        }

        for y in 0..trees.len() {
            for x in 0..trees[y].len() {
                if trees[y][x] > 0 {
                    if visi_map[y][x] {
                        print!("{}", (trees[y][x] - 1).to_string().green());
                    } else {
                        print!("{}", (trees[y][x] - 1).to_string().hidden());
                    }
                }
            }
            println!("");
        }
        println!("green are visible");
        println!(
            "Number of visible Trees is:\n{}",
            visi_map
                .into_iter()
                .map(|l| l.into_iter().filter(|visible| *visible).count())
                .reduce(|a, b| a + b)
                .unwrap()
        );
    } else if task == "2" {
        let mut distances: Vec<Vec<u64>> = Vec::new();
        for y in 0..trees.len() {
            distances.push(Vec::new());
            for x in 0..trees[y].len() {
                let dist_len = distances.len();
                let score: u64 = count_viewingdistance(&trees, x as isize, y as isize, 0, 1) as u64
                    * count_viewingdistance(&trees, x as isize, y as isize, 1, 0) as u64
                    * count_viewingdistance(&trees, x as isize, y as isize, -1, 0) as u64
                    * count_viewingdistance(&trees, x as isize, y as isize, 0, -1) as u64;
                distances[dist_len - 1].push(score);
            }
        }

        //Visualize and find max:
        // Does obviously not work, i need coffee
        /*
        let mut max = 0;
        for y in 0..distances.len() {
            for x in 0..distances[y].len() {
                if distances[y][x] >= max {
                    print!("{}", distances[y][x].to_string().red());
                    max = distances[y][x];
                } else {
                    print!("{}", distances[y][x].to_string());
                }
            }
            println!("");
        }*/
        dbg!(&distances);
        println!(
            "The largest viewing-score is {}",
            distances
                .into_iter()
                .map(|l| l
                    .into_iter()
                    .reduce(|a, b| if a > b { a } else { b })
                    .unwrap())
                .reduce(|a, b| if a > b { a } else { b })
                .unwrap()
        )
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}

fn count_viewingdistance(
    trees: &Vec<Vec<u32>>,
    x_start: isize,
    y_start: isize,
    x_inc: isize,
    y_inc: isize,
) -> usize {
    let mut result = 0;
    let mut x = x_start;
    let mut y = y_start;
    loop {
        x += x_inc;
        y += y_inc;

        if y >= trees.len() as isize || y < 0 || x >= trees[y as usize].len() as isize || x < 0 {
            break;
        }
        result += 1;
        if trees[y as usize][x as usize] >= trees[y_start as usize][x_start as usize] {
            break;
        }
    }
    return result;
}
