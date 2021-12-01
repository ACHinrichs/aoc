use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);
    let inc_count = count_inc(reader.lines()
			      .map(|x| x.unwrap().parse().unwrap_or(-1)).collect()); //initialize in a way that the first line does not count
    
    println!("{}", inc_count);
}


fn count_inc(lines: Vec<i32>) -> i32 {
    let mut last = -1;
    let mut inc_count = -1; //initialize in a way that the first line does not count
    
    for line in lines{
	let current = line;
	if current > last {
	    inc_count = inc_count + 1;
	}
	last = current;
    }
    
    inc_count
} 
