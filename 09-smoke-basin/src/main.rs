use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let ground = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("Low Level Sum (Part 1) is {}", low_level_sum(&ground));
}

fn low_level_sum(ground: &Vec<Vec<u32>>) -> u32 {
    let mut low_level_sum: u32 = 0;
    for i in 0..ground.len() {
        for j in 0..ground[i].len() {
            if (i <= 0 || ground[i][j] < ground[i - 1][j])
                && (i >= ground.len() - 1 || ground[i][j] < ground[i + 1][j])
                && (j <= 0 || ground[i][j] < ground[i][j - 1])
                && (j >= ground[i].len() - 1 || ground[i][j] < ground[i][j + 1])
            {
                low_level_sum += ground[i][j] + 1;
            }
        }
    }
    low_level_sum
}
