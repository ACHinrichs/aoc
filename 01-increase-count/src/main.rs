use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let mut last = -1;
    let mut inc_count = -1; //initialize in a way that the first line does not count
    
    for line in reader.lines(){
	let current = line.unwrap().parse().unwrap_or(-1);
	if current > last {
	    inc_count = inc_count + 1;
	}
	last = current;
    }
    println!("{}", inc_count);
}
