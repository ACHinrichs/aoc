use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt").expect("file not found");
	let reader = BufReader::new(file);

	let lines = reader.lines().map(|x| x.unwrap());

	let mut forward = 0;
	let mut aim = 0;
	let mut depth = 0;

	for l in lines {
		let split: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
		let value = split[1].parse().unwrap_or(0);
		match split[0].as_str() {
			"forward" => {
				forward += value;
				depth += value * aim;
			}
			"down" => aim += value,
			"up" => aim -= value,
			_ => println!("ERROR {} is not known", split[0]),
		}
	}

	println!("Result is {}", forward * depth)
}
