use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Vent {
	start: Vec<i64>,
	end: Vec<i64>,
}

impl Vent {
	fn get_intersections(&self, other: &Vent) -> Vec<Vec<i64>> {
		//println!("{:?}, {:?}",self,other);
		let mut intersections = Vec::new();
		let directions = self.get_directions();
		for i in 0..self.length() {
			let x = self.start[0] + i * directions[0];
			let y = self.start[1] + i * directions[1];
			//print!("{} ({}.{})", i, x,y);
			if ((x >= other.start[0] && x <= other.end[0])
				|| (x <= other.start[0] && x >= other.end[0]))
				&& ((y >= other.start[1] && y <= other.end[1])
					|| (y <= other.start[1] && y >= other.end[1]))
				&& (other.is_straight()
					|| ((other.start[0] - x).abs()
						== (other.start[1] - y).abs()
						&& (other.end[0] - x).abs()
							== (other.end[1] - y).abs()))
			{
				intersections.push(vec![x, y]);
				//print!("INTERSECTION")
			}
			//print!("\n")
		}
		intersections
	}

	fn parse(s: &str) -> Self {
		let split: Vec<String> =
			s.split(" -> ").map(|x| x.to_string()).collect();
		let p1 = split[0]
			.split(",")
			.map(|x| x.parse::<i64>().unwrap_or(0))
			.collect::<Vec<i64>>();
		let p2 = split[1]
			.split(",")
			.map(|x| x.parse::<i64>().unwrap_or(0))
			.collect::<Vec<i64>>();

		if p2[0] <= p1[0] && p2[1] <= p1[1] {
			return Vent { start: p2, end: p1 };
		}
		Vent { start: p1, end: p2 }
	}

	fn is_straight(&self) -> bool {
		self.start[0] == self.end[0] || self.start[1] == self.end[1]
	}

	//Returns the length (e.g. the number of blocked fields) of the line
	fn length(&self) -> i64 {
		std::cmp::max(self.end[0] - self.start[0], self.end[1] - self.start[1])
			+ 1
	}

	fn get_directions(&self) -> Vec<i64> {
		let mut dirs = Vec::new();
		for i in 0..1 + 1 {
			if self.start[i] > self.end[i] {
				dirs.push(-1);
			} else if self.start[i] < self.end[i] {
				dirs.push(1);
			} else {
				dirs.push(0);
			}
		}
		dirs
	}
}

fn main() {
	let file = File::open("input.txt").expect("file not found");
	let lines = BufReader::new(file).lines().map(|x| x.unwrap());
	let mut vents: Vec<Vent> = Vec::new();

	for l in lines {
		vents.push(Vent::parse(&l));
	}

	let mut intersections = Vec::new();
	for (i, a) in vents.iter().enumerate() {
		for b in vents.iter().skip(i + 1) {
			//println!("{:?} {:?}",a,b);
			if a.start == b.start && a.end == b.end {
				println!("You fucked up mate")
			}
			intersections.append(&mut a.get_intersections(b));
		}
	}
	intersections.sort();
	intersections.dedup();
	for i in &intersections {
		println!("{:?}", i);
	}

	println!("Number of Intersections is {}", &intersections.len())
}
