use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Cuboid {
	start: (i64, i64, i64),
	end: (i64, i64, i64),
}

impl Clone for Cuboid {
	fn clone(&self) -> Cuboid {
		return Cuboid {
			start: self.start,
			end: self.end,
		};
	}
}
impl Copy for Cuboid {}

impl Cuboid {
	fn new_from_points(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Cuboid {
		return Cuboid {
			start: (
				std::cmp::min(p1.0, p2.0),
				std::cmp::min(p1.1, p2.1),
				std::cmp::min(p1.2, p2.2),
			),
			end: (
				std::cmp::max(p1.0, p2.0),
				std::cmp::max(p1.1, p2.1),
				std::cmp::max(p1.2, p2.2),
			),
		};
	}

	fn count_cubes(&self) -> i64 {
		return ((self.end.0 - self.start.0).abs() + 1)
			* ((self.end.1 - self.start.1).abs() + 1)
			* ((self.end.2 - self.start.2).abs() + 1);
	}

	fn point_inside(&self, p: (i64, i64, i64)) -> bool {
		return self.start.0 <= p.0
			&& p.0 <= self.end.0
			&& self.start.1 <= p.1
			&& p.1 <= self.end.1
			&& self.start.2 <= p.2
			&& p.2 <= self.end.2;
	}

	// yea, i know that inside vec is bullshit, but i cant be bothered to fix this
	//fn corners(&self) -> dyn std::iter::Iterator<Item = Vec<&i64>> {
	//	let corners_temp =
	//		[[self.x1, self.point.0 + self.edgelength], [self.point.1, self.point.1 + self.edgelength], [self.point.2, self.point.2 + self.edgelength]];
	//	return dyn corners_temp.iter().multi_cartesian_product().collect();
	//}

	// returns all corners from self that are contained in other
	fn corners_inside(&self, other: &Cuboid) -> Vec<(i64, i64, i64)> {
		let corners_temp = [
			[self.start.0, self.end.0],
			[self.start.1, self.end.1],
			[self.start.2, self.end.2],
		];
		let corners = corners_temp.iter().multi_cartesian_product();
		let mut corners_inside = Vec::new();
		for c in corners {
			if other.point_inside((*c[0], *c[1], *c[2])) {
				corners_inside.push((*c[0], *c[1], *c[2]));
			}
		}

		//.map(|x| x.collect())
		//.collect();
		return corners_inside;
	}

	fn cube_minus(&self, other: &Cuboid) -> Vec<Cuboid> {
		// Removes the part of self that is intersected with other,
		// Returns a list of subcubes __all contained in self and not in other__, e.g. no subcube intersects other
		let mut res = self.decompose(other);
		let len_before = res.len();
		res.retain(|cuboid| cuboid.corners_inside(other).len() == 0);
		let len_after = res.len();

		// we should have removed at most one subcube
		assert!(len_before - len_after == 0 || len_before - len_after == 1);

		return res;
	}

	fn decompose(&self, other: &Cuboid) -> Vec<Cuboid> {
		let mut res = Vec::new();
		if other.corners_inside(self).len() == 0 {
			// no corners of other are in self, so we do not need to split self
			res.push(*self);
			return res;
		}
		// new cuboid within our dimensions
		let c = &Cuboid::new_from_points(
			(
				std::cmp::max(self.start.0, other.start.0),
				std::cmp::max(self.start.1, other.start.1),
				std::cmp::max(self.start.2, other.start.2),
			),
			(
				std::cmp::min(self.end.0, other.end.0),
				std::cmp::min(self.end.1, other.end.1),
				std::cmp::min(self.end.2, other.end.2),
			),
		);
		//Cubes on lowest layer (z=0)
		res.push(Cuboid::new_from_points(
			(self.start.0, self.start.1, self.start.2),
			(c.start.0 - 1, c.start.1 - 1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, self.start.1, self.start.2),
			(c.end.0, c.start.1 - 1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, self.start.1, self.start.2),
			(self.end.0, c.start.1 - 1, c.start.2 - 1),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.start.1, self.start.2),
			(c.start.0 - 1, c.end.1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.start.1, self.start.2),
			(c.end.0, c.end.1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.start.1, self.start.2),
			(self.end.0, c.end.1, c.start.2 - 1),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.end.1 + 1, self.start.2),
			(c.start.0 - 1, self.end.1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.end.1 + 1, self.start.2),
			(c.end.0, self.end.1, c.start.2 - 1),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.end.1 + 1, self.start.2),
			(self.end.0, self.end.1, c.start.2 - 1),
		));

		// cubes on middel layer (z dimensions like c)
		res.push(Cuboid::new_from_points(
			(self.start.0, self.start.1, c.start.2),
			(c.start.0 - 1, c.start.1 - 1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, self.start.1, c.start.2),
			(c.end.0, c.start.1 - 1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, self.start.1, c.start.2),
			(self.end.0, c.start.1 - 1, c.end.2),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.start.1, c.start.2),
			(c.start.0 - 1, c.end.1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.start.1, c.start.2),
			(c.end.0, c.end.1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.start.1, c.start.2),
			(self.end.0, c.end.1, c.end.2),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.end.1 + 1, c.start.2),
			(c.start.0 - 1, self.end.1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.end.1 + 1, c.start.2),
			(c.end.0, self.end.1, c.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.end.1 + 1, c.start.2),
			(self.end.0, self.end.1, c.end.2),
		));
		// cubes on uppermost layer (z ends at self.end)
		res.push(Cuboid::new_from_points(
			(self.start.0, self.start.1, c.end.2 + 1),
			(c.start.0 - 1, c.start.1 - 1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, self.start.1, c.end.2 + 1),
			(c.end.0, c.start.1 - 1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, self.start.1, c.end.2 + 1),
			(self.end.0, c.start.1 - 1, self.end.2),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.start.1, c.end.2 + 1),
			(c.start.0 - 1, c.end.1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.start.1, c.end.2 + 1),
			(c.end.0, c.end.1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.start.1, c.end.2 + 1),
			(self.end.0, c.end.1, self.end.2),
		));

		res.push(Cuboid::new_from_points(
			(self.start.0, c.end.1 + 1, c.end.2 + 1),
			(c.start.0 - 1, self.end.1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.start.0, c.end.1 + 1, c.end.2 + 1),
			(c.end.0, self.end.1, self.end.2),
		));
		res.push(Cuboid::new_from_points(
			(c.end.0 + 1, c.end.1 + 1, c.end.2 + 1),
			(self.end.0, self.end.1, self.end.2),
		));

		//Very hacky
		// Delete cuboids with wrong lower/upper bounds:
		res.retain(|cuboid| {
			self.point_inside(cuboid.start) && self.point_inside(cuboid.end)
		});

		return res;
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		panic!("Please supply the problem-input and the task to solve as arguments!");
	}
	let task = &args[2];
	if task == "test" {
		let block1 = Cuboid::new_from_points((0, 0, 0), (10, 10, 10));

		println!("=== Check if occtree-buildung works ===");
		let points = [
			[(5, 5, 5), (5, 5, 5)],
			[(2, 2, 2), (9, 9, 9)],
			[(4, 4, 4), (6, 6, 6)],
			[(0, 0, 0), (5, 5, 5)],
			[(10, 10, 10), (10, 10, 10)],
			[(10, 0, 10), (10, 10, 10)],
			[(10, 11, 10), (12, 12, 12)],
		];
		for p in points.iter() {
			let blocks = block1.decompose(&Cuboid::new_from_points(p[0], p[1]));
			println!("\nSplit cuboid by: {:?}", p);
			for b in blocks.iter() {
				println!("Result: {:?}", b);
			}

			for x in 0..10 {
				for y in 0..10 {
					for z in 0..10 {
						let contained_count = blocks.iter().fold(0, |r, b| {
							if b.point_inside((x, y, z)) {
								r + 1
							} else {
								r
							}
						});
						//println!("Testing ({},{},{}) {}", x, y, z, contained_count);
						assert!(contained_count == 1);
					}
				}
			}
		}

		println!("=== Check if remove-intersection works ===");
		for p in points.iter() {
			let block2 = Cuboid::new_from_points((0, 0, 0), (10, 10, 10));

			println!("\n Remove cuboid {:?}", p);
			let blocks =
				block2.cube_minus(&Cuboid::new_from_points(p[0], p[1]));
			for b in blocks.iter() {
				println!("Result: {:?}", b);
			}
		}
	} else {
		let file = File::open(&args[1])
			.expect(&format!("Inputfile {} not found", args[1]));
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

		//Parse to blocks
		let mut blocks = Vec::new();
		let number_regex = Regex::new(r"-?\d+").unwrap();
		for line in lines {
			println!("Parse command {}", line);
			let matches = number_regex
				.find_iter(line)
				.map(|m| {
					m.as_str().parse::<i64>().expect(
						"Your regex matched some non-numbers, you morron",
					)
				})
				.collect::<Vec<i64>>();
			let p1 = (matches[0], matches[2], matches[4]);
			let p2 = (matches[1], matches[3], matches[5]);
			println!("Points: {:?} {:?}", p1, p2);
			if line.starts_with("on") {
				//create an on-block, that does not have intersections with any other block already existent
				let mut to_add = Vec::new();

				to_add.push(Cuboid::new_from_points(p1, p2));
				for existing_block in blocks.iter() {
					let mut to_add_buffer = Vec::new();
					for b in to_add {
						let mut new_blocks = b.cube_minus(existing_block);
						to_add_buffer.append(&mut new_blocks);
					}
					to_add = to_add_buffer;
				}
				println!("Adding the following Cuboids:");
				for c in to_add.iter() {
					println!(" {:?}", c);
				}
				blocks.append(&mut to_add);
			} else if line.starts_with("off") {
				let mut new_blocks = Vec::new();
				let off_block = Cuboid::new_from_points(p1, p2);

				for block in blocks.iter() {
					let mut blocks_after_deletion =
						block.cube_minus(&off_block);
					new_blocks.append(&mut blocks_after_deletion);
				}
				blocks = new_blocks;
			}
			println!("{}", blocks.iter().fold(0, |n, b| n + b.count_cubes()));
		}

		println!("{}", blocks.len());
		blocks.retain(|b| {
			b.start.0 <= 50
				&& b.start.1 <= 50
				&& b.start.2 <= 50
				&& b.start.0 >= -50
				&& b.start.1 >= -50
				&& b.start.2 >= -50
		});
		println!("{}", blocks.len());
		let number_of_on = blocks.iter().fold(0, |n, b| n + b.count_cubes());
		println!("Number of activated Cubes is {}", number_of_on);
	}
}
