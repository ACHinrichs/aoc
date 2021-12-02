use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|x| x.unwrap());

    let mut forward = 0;
    let mut depth = 0;

    for l in lines{
	let split: Vec<String> = l.split(' ').map(|s| s.to_string()).collect();
	match split[0].as_str(){
	    "forward"=>forward+=split[1].parse().unwrap_or(0),
	    "down"=>depth+=split[1].parse().unwrap_or(0),
	    "up"=>depth-=split[1].parse().unwrap_or(0),
	    _=>println!("ERROR {} is not known", split[0]),
	}
    }

    println!("Result is {}", forward * depth)
    
}
