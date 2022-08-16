use num::integer;
use std::clone::Clone;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Deref;
struct SnailfishNumber {
	is_regular: bool,
	left: Option<Box<SnailfishNumber>>,
	right: Option<Box<SnailfishNumber>>,
	value: i64,
}

//impl Copy for SnailfishNumber { }
impl Clone for SnailfishNumber {
	fn clone(&self) -> SnailfishNumber {
		if self.is_regular {
			SnailfishNumber {
				is_regular: self.is_regular,
				left: None,
				right: None,
				value: self.value,
			}
		} else {
			SnailfishNumber {
				is_regular: self.is_regular,
				left: Some(Box::new(
					self.left.as_ref().unwrap().deref().clone(),
				)),
				right: Some(Box::new(
					self.right.as_ref().unwrap().deref().clone(),
				)),
				value: self.value,
			}
		}
	}
}

impl std::fmt::Display for SnailfishNumber {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl SnailfishNumber {
	// Gets a snailfishNumber including wraping [], returns a string with stuff
	// left to parse
	fn parse(s: &str) -> (SnailfishNumber, &str) {
		// No anity-checks, cause this is aoc
		//println!("Parsing (sub)string {}", s);
		let regular_end: usize = std::cmp::min(
			s.find("]").unwrap_or(usize::MAX),
			s.find(",").unwrap_or(usize::MAX),
		);
		let to_number = s[0..regular_end].parse::<i64>();
		if to_number.is_ok() {
			return (
				SnailfishNumber {
					is_regular: true,
					left: None,
					right: None,
					value: to_number.unwrap(),
				},
				&s[regular_end..],
			);
		}
		//Not regular, hence has to be a pair
		// 1.. should strip the opening braces
		let l_res = SnailfishNumber::parse(&s[1..]);
		//1.. to strip the comma
		let r_res = SnailfishNumber::parse(&l_res.1[1..]);
		//1.. shoudl now strip the closing brace
		return (
			SnailfishNumber {
				is_regular: false,
				left: Some(Box::new(l_res.0)),
				right: Some(Box::new(r_res.0)),
				value: i64::MIN,
			},
			&r_res.1[1..],
		);
	}

	fn to_string(&self) -> String {
		if self.is_regular {
			return self.value.to_string();
		} else {
			return "[".to_owned()
				+ &self.left.as_ref().unwrap().to_string()
				+ "," + &self.right.as_ref().unwrap().as_ref().to_string()
				+ "]";
		}
	}

	fn reduce(&self) -> SnailfishNumber {
		let mut new_snailfish = self.clone();

		loop {
			let res = new_snailfish.explode_one(0);
			new_snailfish = res.1.expect("Somethings fubar");
			//println!("{}", new_snailfish);
			//break;
			if res.3 {
				//Explode did some changes
				continue;
			}
			let split_res = new_snailfish.split_one();
			new_snailfish = split_res.0;
			//println!("{}", new_snailfish);
			if split_res.1 {
				//Split did some changes
				continue;
			} else {
				// Check if a new explode changes anythingm otherwise exit
				let res = new_snailfish.explode_one(0);
				new_snailfish = res.1.expect("Somethings fubar");
				//println!("{}", new_snailfish);
				if res.3 {
					//Explode did some changes
					continue;
				} else {
					break;
				}
				// ToDo: I think i can just break here without the logic and explosion
			}
		}
		new_snailfish
	}

	fn add(first: SnailfishNumber, second: SnailfishNumber) -> SnailfishNumber {
		SnailfishNumber {
			is_regular: false,
			left: Some(Box::new(first.clone())),
			right: Some(Box::new(second.clone())),
			value: i64::MIN,
		}
		.reduce()
	}
	fn box_if_any(a: Option<SnailfishNumber>) -> Option<Box<SnailfishNumber>> {
		if a.is_some() {
			Some(Box::new(a.unwrap()))
		} else {
			None
		}
	}

	fn magnitude(&self) -> i64 {
		if self.is_regular {
			self.value
		} else {
			3 * self.left.as_ref().unwrap().magnitude()
				+ 2 * self.right.as_ref().unwrap().magnitude()
		}
	}

	fn split_one(&self) -> (SnailfishNumber, bool) {
		if self.is_regular {
			if self.value >= 10 {
				//println!(
				//	"Pair {} {} {} is split",
				//	self,
				//	self.left.is_some(),
				//	self.right.is_some()
				//);
				let left = SnailfishNumber {
					is_regular: true,
					left: None,
					right: None,
					value: integer::div_floor(self.value, 2),
				};
				let right = SnailfishNumber {
					is_regular: true,
					left: None,
					right: None,
					value: integer::div_ceil(self.value, 2),
				};
				return (
					SnailfishNumber {
						is_regular: false,
						left: Some(Box::new(left)),
						right: Some(Box::new(right)),
						value: i64::MAX,
					},
					true,
				);
			} else {
				return (self.clone(), false);
			}
		} else {
			let (left, left_changes) = self.left.as_ref().unwrap().split_one();
			if !left_changes {
				let (right, right_changes) =
					self.right.as_ref().unwrap().split_one();
				return (
					SnailfishNumber {
						is_regular: false,
						left: Some(Box::new(left)),
						right: Some(Box::new(right)),
						value: i64::MIN,
					},
					right_changes,
				);
			} else {
				let right = self.right.as_ref().unwrap().deref().clone();
				return (
					SnailfishNumber {
						is_regular: false,
						left: Some(Box::new(left)),
						right: Some(Box::new(right)),
						value: i64::MIN,
					},
					true,
				);
			}
		}
	}

	fn explode_one(
		&self,
		depth: u64,
	) -> (Option<i64>, Option<SnailfishNumber>, Option<i64>, bool) {
		//println!("Looking at pair {} at depth {}", self, depth);
		assert!(depth <= 4, "You fucked up big time");
		if self.is_regular {
			return (None, Some(self.clone()), None, false);
		} else if depth == 4
			&& self.left.as_ref().unwrap().is_regular
			&& self.right.as_ref().unwrap().is_regular
		{
			// Explode this
			// »Exploding pairs will always consist of two regular numbers«
			//println!("Pair {} explodes!", self);
			return (
				Some(self.left.as_ref().unwrap().value),
				Some(SnailfishNumber {
					is_regular: true,
					left: None,
					right: None,
					value: 0,
				}),
				Some(self.right.as_ref().unwrap().value),
				true,
			);
		} else {
			// Das was links vom left.reduce rausfällt muss nach "oben" weiter gegeben
			// werden, das was rechts vom rechten rausfällt auch, den rest können wir
			// selber verwurschten
			let (
				left_explosion_fragment,
				mut new_left,
				left_right_explosion_fragment,
				left_did_a_thing,
			) = self.left.as_ref().unwrap().explode_one(depth + 1);
			let mut new_right = self.right.as_ref().unwrap().deref().clone();
			let mut right_explosion_fragment = None;
			let mut right_did_a_thing = false;
			if left_did_a_thing {
				if left_right_explosion_fragment.is_some() {
					new_right = new_right.add_to_leftmost_regular(
						left_right_explosion_fragment.unwrap(),
					);
				}
			} else {
				let mut r = self.right.as_ref().unwrap().explode_one(depth + 1);
				new_right = r.1.unwrap();
				if r.0.is_some() {
					new_left = Some(
						self.left
							.as_ref()
							.unwrap()
							.add_to_rightmost_regular(r.0.unwrap()),
					);
				}
				right_explosion_fragment = r.2;
				right_did_a_thing = r.3
			}
			return (
				left_explosion_fragment,
				Some(SnailfishNumber {
					is_regular: false,
					left: SnailfishNumber::box_if_any(new_left),
					right: SnailfishNumber::box_if_any(Some(new_right)),
					value: i64::MIN,
				}),
				right_explosion_fragment,
				left_did_a_thing || right_did_a_thing,
			);
		}
	}

	fn add_to_leftmost_regular(&self, value: i64) -> SnailfishNumber {
		if self.is_regular {
			SnailfishNumber {
				is_regular: true,
				left: None,
				right: None,
				value: self.value + value,
			}
		} else {
			SnailfishNumber {
				is_regular: false,
				left: Some(Box::new(
					self.left.as_ref().unwrap().add_to_leftmost_regular(value),
				)),
				right: Some(Box::new(
					self.right.as_ref().unwrap().deref().clone(),
				)),
				value: self.value,
			}
		}
	}

	fn add_to_rightmost_regular(&self, value: i64) -> SnailfishNumber {
		if self.is_regular {
			assert!(self.value <= i64::MAX);
			assert!(value <= i64::MAX);
			SnailfishNumber {
				is_regular: true,
				left: None,
				right: None,
				value: self.value + value,
			}
		} else {
			SnailfishNumber {
				is_regular: false,
				right: Some(Box::new(
					self.right
						.as_ref()
						.unwrap()
						.add_to_rightmost_regular(value),
				)),
				left: Some(Box::new(
					self.left.as_ref().unwrap().deref().clone(),
				)),
				value: self.value,
			}
		}
	}
}

fn main() {
	let part = 2;
	let file = File::open("input.txt").expect("file not found");
	let lines = &mut BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().to_string())
		.filter(|x| {
			if x.starts_with("//") {
				println!("Found comment in input: {}", &x[2..]);
				false
			} else {
				true
			}
		})
		.collect::<Vec<String>>();
	
	if part == 1{
		println!("Part 1");
		let res = lines
			.iter()
			.map(|x| SnailfishNumber::parse(x).0)
			.reduce(SnailfishNumber::add)
			.unwrap();
		println!("Result is {} with Magnitude {}", res, res.magnitude());
	} else {
		println!("Part 2");
		let snailfish_numbers = lines
			.iter()
			.map(|x| SnailfishNumber::parse(x).0).collect::<Vec<SnailfishNumber>>();
		let mut max = i64::MIN;
		// Fucking bruteforce every good damn permutation
		for i in 0..snailfish_numbers.len(){
			for j in 0..snailfish_numbers.len(){
				if i == j{
					continue
				}
				max = std::cmp::max(max, SnailfishNumber::add(snailfish_numbers[i].clone(),
															  snailfish_numbers[j].clone()).magnitude())
			}
			println!("{}/{}",i,snailfish_numbers.len()-1);
		}
		println!("Result for P2 is {}", max);
	}
}
