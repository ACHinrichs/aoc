use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn read_file(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>){
    let file = File::open(filename).expect("ERR: file not found!");
    let mut lines = BufReader::new(file).lines().map(|x| x.unwrap());

    //Parse first line (The drawn numbers)
    let res_numbers:Vec<i32> = lines.next().expect("ERR: Numbers expected here")
	.split(',')
	.map(|s| s.parse::<i32>().expect("ERR: Encountered something that is not a Number"))
	.collect::<Vec<i32>>();
    
    //Parse boards
    let mut res_boards:Vec<Vec<Vec<i32>>> = Vec::new();
    let mut i = 0;
    for l in lines{
	if l == ""{
	    //Between the boards we generate the new ones
	    res_boards.push(Vec::new());
	    println!("New board");
	} else {
	    //Parse one line of a board
	    res_boards.last_mut().expect("ERR Board invalid").push((i..i+5).collect());
	    i+=1;
	    println!("{:?}", res_boards.last());
	}
    }
    
    (res_numbers, res_boards)
    
}

fn main() {
    read_file("example.txt");
}
