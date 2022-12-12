use colored::*;
use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::time::{Duration, Instant};

fn main() {
    println!("AOC 2022, Day 12 – Hill Climb Algorithm\n");
    // Shave of the initial text, cause I'm a cheater :P
    let timing_start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
        .filter(|l| !l.starts_with("//"))
        .collect::<Vec<String>>();

    let timing_after_fileread = Instant::now();

    let mut heightmap: Vec<Vec<u32>> = Vec::new();

    //Parse trees and save start and end location
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    let mut y = 0;
    let mut x = 0;
    for l in lines {
        let mut tmp_line = Vec::new();
        for c in l.chars() {
            let mut value = c as u32;
            if c == 'S' {
                value = 'a' as u32;
                start_x = x + 1; // Add 1 because we will add a border later!
                start_y = y + 1; // Add 1 because we will add a border later!
            } else if c == 'E' {
                value = 'z' as u32;
                end_x = x + 1; // Add 1 because we will add a border later!
                end_y = y + 1; // Add 1 because we will add a border later!
            }
            value = value - 'a' as u32 + 1;
            tmp_line.push(value);
            x += 1;
        }
        heightmap.push(tmp_line);
        y += 1;
        x = 0;
    }

    // Add border of height z+2, so that it can not be reached, therefore we need not to worry about over and underflows of the array index
    let len_lines_without_border = heightmap[0].len();
    let unreachable_height = 28;
    heightmap.insert(0, vec![unreachable_height; len_lines_without_border]);
    heightmap.push(vec![unreachable_height; len_lines_without_border]);
    heightmap = heightmap
        .into_iter()
        .map(|mut l| {
            l.insert(0, unreachable_height);
            l
        })
        .map(|mut l| {
            l.push(unreachable_height);
            l
        })
        .collect();

    let mut visited = Vec::new();
    for i in 0..heightmap.len() {
        visited.push(vec![false; heightmap[i].len()]);
    }
    if task == "1" {
        let (found, length) = bfs(
            start_x,
            start_y,
            end_x,
            end_y,
            &heightmap,
            &mut visited,
            false,
        );
        if found {
            println!("Found a path of size {}!", length);
        } else {
            println!("Did not find a path");
        }
    } else if task == "2" {
        let (found, length) = bfs(
            end_x,
            end_y,
            start_x,
            start_y,
            &heightmap,
            &mut visited,
            true,
        );
        if found {
            println!("Found a path of size {}!", length);
        } else {
            println!("Did not find a path");
        }
    } else {
        panic!("Task unknown, please specify as first argument")
    }
    println!(
        "Execution time: {} µs\nExecution time without file-reading: {} µs",
        timing_start.elapsed().as_micros(),
        timing_after_fileread.elapsed().as_micros(),
    );
}

fn bfs(
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    heights: &Vec<Vec<u32>>,
    visited: &mut Vec<Vec<bool>>,
    reversed: bool,
) -> (bool, u32) {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y, 0));
    while queue.len() > 0 {
        let (cur_x, cur_y, mut cur_length) = queue.pop_front().unwrap();
        //dbg!(cur_x, cur_y, end_x, end_y);
        print_maps(cur_x, cur_y, heights, visited);
        if !reversed {
            if cur_x == end_x && cur_y == end_y {
                return (true, cur_length);
            }
        } else {
            if heights[cur_y][cur_x] == heights[end_x][end_y] {
                // yes, i know that that is bad code, cry me a river
                return (true, cur_length);
            }
        }
        if visited[cur_y][cur_x] {
            continue;
        }
        visited[cur_y][cur_x] = true;
        let cur_height = heights[cur_y][cur_x];
        for (d_x, d_y) in directions {
            let new_x = (cur_x as isize + d_x) as usize;
            let new_y = (cur_y as isize + d_y) as usize;
            if !visited[new_y][new_x] {
                if !reversed && heights[new_y][new_x] <= cur_height + 1 {
                    queue.push_back((new_x, new_y, cur_length + 1));
                } else if reversed
                    && heights[new_y][new_x] >= cur_height - 1
                    && heights[new_y][new_x] <= 26
                {
                    queue.push_back((new_x, new_y, cur_length + 1));
                }
            }
        }
    }
    return (false, 0);
}

#[cfg(debug_assertions)]
fn print_maps(cur_x: usize, cur_y: usize, heights: &Vec<Vec<u32>>, visited: &Vec<Vec<bool>>) {
    let start_offset = 'a' as u32 - 1;
    for y in 0..heights.len() {
        for x in 0..heights[y].len() {
            let cur_symbol = char::from_u32(heights[y][x] + start_offset)
                .unwrap()
                .to_string();
            if y == cur_y && x == cur_x {
                print!("{}", cur_symbol.red());
            } else if visited[y][x] {
                print!("{}", cur_symbol.blue());
            } else {
                print!("{}", cur_symbol);
            }
        }
        println!("");
    }
}

#[cfg(not(debug_assertions))]
fn print_maps(cur_x: usize, cur_y: usize, heights: &Vec<Vec<u32>>, visited: &Vec<Vec<bool>>) {
    return;
}

fn get_unvisited(
    heights: &Vec<Vec<u32>>,
    visited: &Vec<Vec<bool>>,
    a: u32,
) -> Option<(usize, usize)> {
    for y in 0..visited.len() {
        for x in 0..visited.len() {
            if !visited[y][x] && heights[y][x] == a {
                return Some((x, y));
            }
        }
    }
    return None;
}
