use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

trait Die {
	fn roll(&mut self) -> u64; 
}

struct DeterministicDie{
	rolls: u64,
}

impl DeterministicDie{
	fn new() -> DeterministicDie{
		DeterministicDie {
			rolls: 0,
		}
	}
}

impl Die for DeterministicDie{
	fn roll(&mut self) -> u64{
		let res = self.rolls % 100 + 1;// Because we are offset
		self.rolls = self.rolls + 1;
		return res;
	}
}


fn main() {
	let file = File::open("input.txt").expect("file not found");
	let mut positions = &mut BufReader::new(file)
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
		}).map(|x| x.split(": ").nth(1).unwrap().parse::<u64>().unwrap() - 1)//MIND THE OFFSET
		.collect::<Vec<u64>>();
	let mut points = Vec::new();
	for i in 0..positions.len(){
		points.push(0);
	}

	let mut die = DeterministicDie::new();
	
	'game_loop: loop{
		println!("----- New round -----");
		for player in 0..positions.len(){
			let roll = die.roll() + die.roll() + die.roll();
			positions[player] = (positions[player] + roll) % 10;
			points[player] += positions[player] + 1; //Mind the offset!
			println!("Player {} \n Position: {} Points: {}", player, positions[player], points[player]);
			if points[player] >= 1000{
				break 'game_loop;
			}
		}
	}
	println!("rolls * score of loosing player (Sol. Pt. 1) is {}", die.rolls * points.iter().reduce(|x,y| std::cmp::min(x,y)).unwrap());
	
}
