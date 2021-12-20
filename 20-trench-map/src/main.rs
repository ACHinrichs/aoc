use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("example.txt").expect("file not found");
	let lines = &mut BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().to_string())
		.filter(|x| {
			if x.starts_with("//") {
				//Filter comments
				println!("Found comment in input: {}", &x[2..]);
				false
			} else {
				//Filter empty lines
				x != ""
			}
		})
		.collect::<Vec<String>>();
	println!("Hello, world!");
	//First line is the "algorithm"
	let parse_char = |x| match x {'#' => 1, '.' => 0, _ => i64::MAX};
	let gorithm = lines[0].chars().map(parse_char).collect::<Vec<i64>>();
	
}
