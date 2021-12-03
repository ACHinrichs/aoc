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

    let mut oxyrating = bits_vec.iter().map(|x| x).collect::<Vec<&Vec<u32>>>(); //wtf
    let mut pos = 0;

    //println!("{}", pos);
    while oxyrating.len() > 1 && pos < *len {
	println!("{} {}", pos, oxyrating.len());
	println!("{:?}", oxyrating);
	let ones = count_ones(&oxyrating.iter().map(|x| x[pos]).collect());
	let zeros = oxyrating.len() as u32 - ones;
	
	let most_common;  
	
	if ones >= zeros{
	    most_common = 1;
	    println!("1");
	}else{
	    most_common = 0;
	    println!("0");
	}
	
	oxyrating = (oxyrating.iter()).filter(|x| x[pos] == most_common)
	    .map(|x| *x)
	    .collect::<Vec<&Vec<u32>>>();

	pos += 1;
    }
    println!("{} {}", pos, oxyrating.len());
    println!("\noxyrating: {:?}", oxyrating);
	
    let mut co2rating = bits_vec.iter().map(|x| x).collect::<Vec<&Vec<u32>>>(); //wtf
    pos = 0;
    while co2rating.len() > 1 && pos < *len {
	let ones = count_ones(&co2rating.iter().map(|x| x[pos]).collect());
	let zeros = co2rating.len() as u32 - ones;
	
	let mut least_common = 2;  
	
	if ones >= zeros{
	    least_common = 0;
	}else if zeros > ones{
	    least_common = 1;
	}
	
	co2rating = (co2rating.iter()).filter(|x| x[pos] == least_common)
	    .map(|x| *x)
	    .collect::<Vec<&Vec<u32>>>();

	pos += 1;
    }
    
    print!("\n");

    println!("Results:\n");
    println!("\ngamma: {}", gamma);
    println!("\nepsilon: {}", epsilon);

    
    println!("gamma * epsilon = {}", gamma * epsilon);

    println!("\noxyrating: {:?}, {}", oxyrating, bin_to_dec(oxyrating[0]));
    println!("co2rating: {:?}, {}", co2rating, bin_to_dec(co2rating[0]));
    println!("combined: {}", bin_to_dec(oxyrating[0]) * bin_to_dec(co2rating[0]));

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


fn bin_to_dec(bits: &Vec<u32>) -> u32{
    bits.iter().fold(0,|x,y| x*2+*y)
}
