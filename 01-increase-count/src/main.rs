use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt").expect("file not found");
	let reader = BufReader::new(file);

	let lines = reader
		.lines()
		.map(|x| x.unwrap().parse().unwrap_or(-1))
		.collect();

	let windows = sliding_window(&lines, 3);

	for i in 0..lines.len() {
		if i < windows.len() {
			println!("{} {}", lines[i], windows[i])
		} else {
			println!("{}", lines[i])
		}
	}

	let inc_count = count_inc(&windows); //initialize in a way that the first line does not count
	println!("Result: {}", inc_count);
}

fn count_inc(lines: &Vec<i32>) -> i32 {
	let mut last = -1;
	let mut inc_count = -1; //initialize in a way that the first line does not count

	for line in lines {
		let current = line;
		if *current > last {
			inc_count = inc_count + 1;
		}
		last = *current;
	}

	inc_count
}

fn sliding_window(input: &Vec<i32>, windowsize: usize) -> Vec<i32> {
	let mut res = Vec::with_capacity(input.len() - (windowsize - 1));

	for _i in 0..(res.capacity()) {
		res.push(0)
	}

	for i in 0..windowsize {
		for j in 0..input.len() - (windowsize - 1) {
			res[j] += input[j + i]
		}
	}

	res
}
