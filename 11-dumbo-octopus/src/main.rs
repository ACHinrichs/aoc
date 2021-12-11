extern crate termion;
use termion::{color, style};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("example.txt").expect("file not found");
    let mut grid = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();
    let mut flash_count = 0;

    let num_steps = 100;

    let mut currently_flashing = 0;
    let mut singularity_step = 0;

    for step in 1..=num_steps {
        println!("\nStep {}\n", step);
        currently_flashing = make_step(&mut grid);
        flash_count += currently_flashing;
        if currently_flashing == (grid.len() * grid[0].len()) as u64 {
            singularity_step = step;
        }
    }
    let mut step = num_steps;
    while singularity_step == 0 {
        println!("\nStep {}\n", step);
        currently_flashing = make_step(&mut grid);
        if currently_flashing == (grid.len() * grid[0].len()) as u64 {
            singularity_step = step;
        } else {
            step += 1;
        }
    }
    println!("Flash count after 100 Steps (Part 1) is {}", flash_count);
    println!("All octopusses flash in step {}", singularity_step);
}

fn make_step(grid: &mut Vec<Vec<i64>>) -> u64 {
    let mut flash_count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            assert!(grid[i][j] <= 9);
            assert!(grid[i][j] >= 0);
            grid[i][j] += 1;
        }
    }

    //Fixpointiteration to find all flashing, flashings are not reset to 0 yet
    let mut flashing = true;
    while flashing {
        flashing = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] > 9 {
                    flash_neighbours(grid, i, j);
                    grid[i][j] = -9;
                    flashing = true;
                }
            }
        }
    }
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == -9 {
                flash_count += 1;
                grid[i][j] = 0;
                print!(
                    "{}{}{}",
                    color::Fg(color::Magenta),
                    grid[i][j],
                    color::Fg(color::Reset)
                );
            } else {
                print!("{}", grid[i][j]);
            }
        }
        print!("\n");
    }
    flash_count
}

fn count_active_neighbours(grid: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    let mut res = 0;
    for k in [-1, 0, 1] {
        for l in [-1, 0, 1] {
            //Don't count itself
            if k == 0 && l == 0 {
                continue;
            }
            if (0 <= (i as i64) + k)
                && ((i as i64) + k < grid.len() as i64)
                && (0 <= (j as i64) + l)
                && ((j as i64) + l < grid[i].len() as i64)
                && (grid[(i as i64 + k) as usize][(j as i64 + l) as usize] > 9)
            {
                //println!("{}+{} {}+{}",i,k,j,l);
                res += 1;
            }
        }
    }
    res
}

fn flash_neighbours(grid: &mut Vec<Vec<i64>>, i: usize, j: usize) {
    for k in [-1, 0, 1] {
        for l in [-1, 0, 1] {
            //Don't count itself
            if k == 0 && l == 0 {
                continue;
            }
            if (0 <= (i as i64) + k)
                && ((i as i64) + k < grid.len() as i64)
                && (0 <= (j as i64) + l)
                && ((j as i64) + l < grid[i].len() as i64)
                && (grid[(i as i64 + k) as usize][(j as i64 + l) as usize] >= 0)
            {
                //println!("{}+{} {}+{}",i,k,j,l);
                grid[(i as i64 + k) as usize][(j as i64 + l) as usize] += 1;
            }
        }
    }
}
