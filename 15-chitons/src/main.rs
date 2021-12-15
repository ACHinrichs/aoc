use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("ERR: file not found!");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap());

    let input_numbers: Vec<Vec<i64>> = lines
        .map(|y| {
            y.split("")
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<i64>()
                        .expect("ERR: Encountered something that is not a Number")
                })
                .collect::<Vec<i64>>()
        })
        .collect();

    let mut res_vec: Vec<Vec<i64>> = Vec::new();
    let mut mark_vec: Vec<Vec<bool>> = Vec::new();
    // init vec
    for y in 0..input_numbers.len() {
        let mut row = Vec::new();
        let mut row_marks = Vec::new();
        for _x in 0..input_numbers[y].len() {
            row.push(i64::MAX);
            row_marks.push(false);
        }
        res_vec.push(row);
        mark_vec.push(row_marks);
    }

    // init upper left point:
    res_vec[0][0] = 0;

    let mut coord_stack: Vec<(i64, i64)> = Vec::new();
    coord_stack.push((0, 0));

    while !coord_stack.is_empty() {
        let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        coord_stack.sort_unstable_by(|d, c| {
            res_vec[d.0 as usize][d.1 as usize]
                .partial_cmp(&res_vec[c.0 as usize][c.1 as usize])
                .unwrap()
        });
        let pos = coord_stack[0];
        coord_stack.remove(0);
        mark_vec[pos.0 as usize][pos.1 as usize] = true;
        println!("{:?}", pos);
        for d in directions {
            if 0 <= pos.0 + d.0
                && pos.0 + d.0 < res_vec.len() as i64
                && 0 <= pos.1 + d.1
                && pos.1 + d.1 < res_vec[(pos.0 + d.0) as usize].len() as i64
            {
                let i: usize = (pos.0 + d.0) as usize;
                let j: usize = (pos.1 + d.1) as usize;

                if res_vec[pos.0 as usize][pos.1 as usize] + input_numbers[i][j] < res_vec[i][j] {
                    res_vec[i][j] = res_vec[pos.0 as usize][pos.1 as usize] + input_numbers[i][j];
                    if !mark_vec[i][j] {
                        coord_stack.push((i as i64, j as i64));
                    }
                }
            }
        }
    }
    print_matrix(&res_vec);
}

fn print_matrix(x: &Vec<Vec<i64>>) {
    for i in 0..x.len() {
        for j in 0..x[i].len() {
            if x[i][j] == i64::MAX {
                print!("   ∞");
            } else {
                print!(" {number:>3}", number = x[i][j]);
            }
        }
        print!("\n");
    }
}
