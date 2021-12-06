use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut lanternfish = lines[0].split(",").map(|x| x.parse::<i64>().unwrap_or(0)).collect::<Vec<i64>>();
    let LIFETIME = 256;
    
    for i in 0..LIFETIME{
	let (a,b): (Vec<i64>, Vec<i64>) = lanternfish.iter().map(|x| if x == &0 {(6, 8)} else {(x-1,-1)} ).unzip();
	lanternfish = a.iter().map(|x| *x).chain(b.iter().map(|x| *x).filter(|x| x != &-1)).collect::<Vec<i64>>();
	//println!("{:?}",lanternfish)
	if i % 10 == 0 {
	    println!("{}",i);
	}
    }

    println!("Number of lanternfish after {} steps is {}", LIFETIME, lanternfish.len());
}
