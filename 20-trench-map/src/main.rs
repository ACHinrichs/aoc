use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("example_input.txt").expect("file not found");
	let lines = &mut BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().to_string())
		.filter(|x| {
			if x.starts_with("//") {
				//Filter comments
				println!("Found comment in input: {}", &x[2..]);
				false
			} else {
				//Filter empty lines
				x != ""
			}
		})
		.collect::<Vec<String>>();
	
	//First line is the "algorithm"
	let parse_char = |x| match x {'#' => 1, '.' => 0, _ => i64::MAX};
	let gorithm = lines[0].chars().map(parse_char).collect::<Vec<i64>>();
	println!("{}", gorithm.len());

	let mut image = Vec::new();
	for l in lines[1..].iter(){
		image.push(l.chars().map(parse_char).collect::<Vec<i64>>());
	}
	let mut fill = 0;
	for i in 0..=2{
		println!("\nIteration {}", i);
		if i > 0{
			image = enhance(image, &gorithm, 0);
			fill = if fill == 1 {gorithm[8]} else {gorithm[0]};
		}
		println!("╔{}╗", "═".repeat(image.len()));
		for l in image.iter() {
			print!("║");
			for c in l{
				print!("{}", match *c {1 => "█", 0 => " ", _ => "E"});
			}
			println!("║");
		}
		println!("╚{}╝", "═".repeat(image.len()));
		println!("Count of light pixels is {}",
				 image.iter().fold(0,
								   |res, x| res + x.iter().fold(0, |inner_res, y| inner_res + y)));
	}
	
}

//Like they do in CSI
fn enhance(inp: Vec<Vec<i64>>, gorithm: &Vec<i64>, fill: i64) -> Vec<Vec<i64>>{
	let mut res = Vec::new();
	// These bounds are important, since we enlarge the image by 1 row and
	// column in each direction
	for i_iter in 0..inp.len()+2{
		let i = i_iter as i64 - 1;
		let mut row = Vec::new();
		for j_iter in 0..inp[0].len() + 2{
			let j = j_iter as i64 - 1;
			let mut gorithm_index = 0;
			for offset_i in [-1,0,1]{
				for offset_j in [-1,0,1]{
					// it is important that we first step through all points of one row
					let window_i = i + offset_i;
					let window_j = j + offset_j;
					let mut new_index = gorithm_index << 1; 
					if (window_i >= 0)
						&& (window_i < inp.len() as i64)
						&& (window_j >= 0)
						&& (window_j < inp[window_i as usize].len() as i64)
					{
						//An dieser stelle ein F an alle die ihre tab-length auf 8 stehen haben
						new_index = new_index |  inp[window_i as usize][window_j as usize];
					} else {
						new_index = new_index | fill;
					}
					//println!("{} {}", new_index, gorithm_index);
					assert!(new_index > gorithm_index || (new_index == 0 &&  gorithm_index == 0));
					gorithm_index = new_index;
				}
			}
			row.push(gorithm[gorithm_index as usize]);
		}
		res.push(row);
	}

	return res;
}
