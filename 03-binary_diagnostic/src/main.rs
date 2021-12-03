use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("ERR: file not found!");
    let lines = &BufReader::new(file).lines().map(|x| x.expect("ERR: while reading the File!")).map(String::from).collect::<Vec<String>>();

    let len = &lines[0].len();
    let mut ones = vec![0; *len];
    let mut zeros = vec![0; *len];

    let mut linecount = 0;
    for l in lines{
	linecount += 1;
	for i in 0..*len{
	    let c =  l.chars().nth(i).expect("ERR: String to CHars failed");
	    //println!("{:?}",c);
	    match c {
		'1' => ones[i]+= 1,
		'0' => zeros[i]+= 1,
		_ => println!("unexpected value {}", c)
	    }
	}
	print!(".");
    }
    println!("{} Lines processed", linecount);

    let mut gamma = 0;
    let mut epsilon = 0; 
    for i in 0..*len{
	gamma = gamma * 2;
	epsilon = epsilon * 2;
	if ones[i] > zeros[i]{
	    gamma += 1;
	    print!("1");
	}else{
	    epsilon += 1;
	    print!("0");
	}
    }
    print!("\n");

    println!("Results:\n");
    println!("ones: {:?}", ones);
    println!("zeros: {:?}", zeros);
    println!("\ngamma: {}", gamma);
    println!("\nepsilon: {}", epsilon);

    println!("gamma * epsilon = {}", gamma * epsilon)
}
