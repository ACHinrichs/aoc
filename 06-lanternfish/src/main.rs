use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


fn main() {
    
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file).lines()
	.map(|x| x.unwrap())
	.collect::<Vec<String>>();

    let lanternfish = &lines[0].split(",")
	.map(|x| x.parse::<i64>().unwrap_or(0))
	.collect::<Vec<i64>>();
    let LIFETIME = 256;
    
    let mut offsets: Vec<u64>=vec![0;7];

    for l in lanternfish{
	offsets[*l as usize] += 1;
    }
    
    let mut next_new_fish: Vec<u64>=vec![0;7];
    let mut generation = 1;
    while generation <= LIFETIME{
	let mut new_fish: Vec<u64>=next_new_fish;
	next_new_fish=vec![0;7];
	//println!("{}", generation);
	for (offset, count) in offsets.iter().enumerate() {
	    //println!(" {} {}",offset, count);
	    if offset + 2 < 7 {
		new_fish[(offset+2) as usize] = *count;
	    } else {
		next_new_fish[(offset + 2 - 7) as usize] = *count;
	    }
	    generation += 1;
	    if generation > LIFETIME{
		break;
	    }
	}
	for i in 0..offsets.len(){
	    offsets[i] = offsets[i]+new_fish[i]
	}
    }
    println!("Number of lanternfish after {} steps is {}",
	     LIFETIME,
	     offsets.iter().fold(0, |x,y| x+y));
}
