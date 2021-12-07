use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file).lines()
	.map(|x| x.unwrap())
	.collect::<Vec<String>>();

    let mut heights = lines[0].split(",")
	.map(|x| x.parse::<i64>().unwrap_or(0))
	.collect::<Vec<i64>>();
    heights.sort();
    let max_height = heights.iter().max().unwrap_or(&0);

    let optimal_height = heights[heights.len()/2];

    let mut cost = 0;
    for h in heights.iter(){
	cost += (optimal_height - h).abs();
    }
    
    println!("Best is {} at a cost of {}", optimal_height, cost);
}
