use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("ERR: file not found!");
    let lines = &BufReader::new(file).lines().map(|x| x.expect("ERR: while reading the File!")).map(String::from).collect::<Vec<String>>();

    let len = &lines[0].len();

    let mut bits_vec: Vec<Vec<u32>> = Vec::new();
    for l in lines{
	let bits = l.chars().map(|x| x as u32 -48).collect();// '0' is 48 in ascii
	bits_vec.push(bits);
    }
    let mut gamma = 0;
    let mut epsilon = 0;
	
    for i in 0..*len{
	let ones = count_ones(&bits_vec.iter().map(|x| x[i]).collect());
	let zeros = bits_vec.len() as u32 - ones;
	gamma = gamma * 2;
	epsilon = epsilon * 2;
	if ones > zeros{
	    gamma += 1;
	    print!("1");
	}else{
	    epsilon += 1;
	    print!("0");
	}
    }
	
    print!("\n");

    println!("Results:\n");
    println!("\ngamma: {}", gamma);
    println!("\nepsilon: {}", epsilon);

    println!("gamma * epsilon = {}", gamma * epsilon)
}

fn count_ones(bits: &Vec<u32>) -> u32{
    let mut res = 0;
    for i in bits{
	if *i == 1 {
	    res += 1;
	}
    }
    res
}
