use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp;

fn main() {
    let file = File::open("input.txt").expect("ERR: file not found!");
    let lines = BufReader::new(file)
	.lines().map(|x| x.unwrap());
		     
    let input_numbers:Vec<Vec<i64>> = lines
	.map(|y| y.split("")
	     .filter(|s| !s.is_empty())
	     .map(|s| s.parse::<i64>()
		  .expect("ERR: Encountered something that is not a Number"))
	     .collect::<Vec<i64>>())
	.collect();

    let mut res_vec: Vec<Vec<i64>> = Vec::new();
    // init vec
    for y in 0..input_numbers.len(){
	let mut row = Vec::new();
	for _x in 0..input_numbers[y].len(){
	    row.push(i64::MAX);
	}
	res_vec.push(row);
    }

    // Solve the problem using D Y N A M I C - P R O G R A M M I N
    // init upper left point:
    res_vec[0][0] = 0;
    for i in 0..input_numbers.len() - 1{
	//Always only look at the upper left i*i square
	// Those have no choice
	res_vec[i+1][0] = res_vec[i][0]+input_numbers[i+1][0];
	res_vec[0][i+1] = res_vec[0][i]+input_numbers[0][i+1];
	for j in 1..=i{
	    // Look at the row and coloumn after the i*i square
	    res_vec[i+1][j] = input_numbers[i+1][j]
		+ std::cmp::min(res_vec[i][j], res_vec[i+1][j-1]);
	    res_vec[j][i+1] = input_numbers[j][i+1]
		+ std::cmp::min(res_vec[j][i], res_vec[j-1][i+1]);
	}
	// Here is a choice:
	print_matrix(&res_vec);
	println!("{} {} {}", input_numbers[i+1][i+1], res_vec[i+1][i], res_vec[i][i+1]);
	res_vec[i+1][i+1] = input_numbers[i+1][i+1]
	    + std::cmp::min(res_vec[i+1][i], res_vec[i][i+1]);
	
	println!("{}", i);
	print_matrix(&res_vec);
    }
}

fn print_matrix(x: &Vec<Vec<i64>>){
    for i in 0..x.len(){
	for j in 0..x[i].len(){
	    if x[i][j] == i64::MAX{
		print!("   ∞");
	    } else {
		print!(" {number:>3}", number=x[i][j]);
	    }
	}
	print!("\n");
    }
}
