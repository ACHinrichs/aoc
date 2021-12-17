use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file).lines()
	.map(|x| x.unwrap())
	.collect::<Vec<String>>();

    let heights = lines[0].split(",")
	.map(|x| x.parse::<i64>().unwrap_or(0))
	.collect::<Vec<i64>>();

    let max_height = heights.iter().max().unwrap_or(&0);

    let mut crab_count = Vec::new();

    for _i in 0..=*max_height{
	crab_count.push(0);
    }

    for h in heights.iter(){
	crab_count[*h as usize] += 1
    }
    
    let mut optimal_heights = Vec::new();
    for height_candidate in 0..=*max_height{
	let mut current_sum = 0;
	for i in 0..height_candidate{
	    current_sum += crab_count[i as usize] * sum_from_0_to_n(height_candidate - i) ;
	}
	for i in height_candidate+1..max_height+1{
	    current_sum += crab_count[i as usize]* sum_from_0_to_n(i - height_candidate);
	}
	optimal_heights.push(current_sum);
    }
    
    println!("{:?}", optimal_heights);
    println!("Best is {:?}", optimal_heights.iter().enumerate().reduce(|(x,y),(a,b)| if y < b {(x,y)} else {(a,b)}).unwrap_or((0,&-1)));
}

fn sum_from_0_to_n(n: i64) -> i64{
    (n * (n+1))/2
}
