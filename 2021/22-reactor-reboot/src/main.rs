use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Block {
	point: (i64, i64, i64),
	edgelengths: (i64, i64, i64),
}

impl Clone for Block {
	fn clone(&self) -> Block {
		return Block {
			point: self.point,
			edgelengths: self.edgelengths,
		};
	}
}
impl Copy for Block {}

impl Block {
	fn new_from_points(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Block {
		assert!((p1.0 - p2.0).abs() != 0);
		assert!((p1.1 - p2.1).abs() != 0);
		assert!((p1.2 - p2.2).abs() != 0);
		return Block {
			point: (
				std::cmp::min(p1.0, p2.0),
				std::cmp::min(p1.1, p2.1),
				std::cmp::min(p1.2, p2.2),
			),
			edgelengths: (
				(p1.0 - p2.0).abs(),
				(p1.1 - p2.1).abs(),
				(p1.2 - p2.2).abs(),
			),
		};
	}

	fn point_inside(&self, p: (i64, i64, i64)) -> bool {
		return self.point.0 <= p.0
			&& p.0 < self.point.0 + self.edgelengths.0
			&& self.point.1 <= p.1
			&& p.1 < self.point.1 + self.edgelengths.1
			&& self.point.2 <= p.2
			&& p.2 < self.point.2 + self.edgelengths.2;
	}

	// yea, i know that inside vec is bullshit, but i cant be bothered to fix this
	//fn corners(&self) -> dyn std::iter::Iterator<Item = Vec<&i64>> {
	//	let corners_temp =
	//		[[self.x1, self.point.0 + self.edgelength], [self.point.1, self.point.1 + self.edgelength], [self.point.2, self.point.2 + self.edgelength]];
	//	return dyn corners_temp.iter().multi_cartesian_product().collect();
	//}

	// returns all corners from self that are contained in other
	fn corners_inside(&self, other: &Block) -> Vec<(i64, i64, i64)> {
		let corners_temp = [
			[self.point.0, self.point.0 + self.edgelengths.0],
			[self.point.1, self.point.1 + self.edgelengths.1],
			[self.point.2, self.point.2 + self.edgelengths.2],
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

	fn remove_intersection(&self, other: &Block) -> Vec<Block> {
		// returns [self] islef is larger than other
		let mut corners_inside = other.corners_inside(self);
		if corners_inside.len() == 0 {
			//switch the corners around, so that this works if slef is larger than other
			todo!("think about this case");
			println!("Switching around, because self might be larger");
			corners_inside = self.corners_inside(other);
			if corners_inside.len() == 8 {
				// if 8 corners of self are insode of other, we are completly contained, return []
				return Vec::new();
			} else if corners_inside.len() == 0 {
				// if 0 corners of self are inside other, and 0 corners of other are inside self we are compeltly disjoint, return self
				return vec![*self];
			}
		} else if corners_inside.len() == 8 {
			let corners_inside_flipped = self.corners_inside(other);
			if corners_inside_flipped.len() == 8 {
				return Vec::new();
			} else {
				println!("What the fuck self {:?}, \n other:{:?}", self, other);
			}
		}
		println!("corners {:?}", corners_inside);
		println!(">=== {:?}", other);
		println!("partially contained {}", corners_inside.len());
		println!("{:?}", corners_inside);
		let mut octtree = self.octtree(
			corners_inside
				.clone()
				.into_iter()
				.filter(|x| {
					x.0 != self.point.0
						&& x.1 != self.point.1 && x.2 != self.point.2
				})
				.fold(
					(
						self.point.0 + self.edgelengths.0,
						self.point.1 + self.edgelengths.1,
						self.point.2 + self.edgelengths.2,
					),
					|r, x| {
						(
							std::cmp::min(x.0, r.0),
							std::cmp::min(x.1, r.1),
							std::cmp::min(x.2, r.2),
						)
					},
				),
		);
		/*if corners_inside.len() == 8 {
			let new_octtree = Vec::new();
			for o in octtree.into_iter() {

		}
			new_octtree = octtree
			.into_iter()
			.filter(|x| x.corners_inside(other).len() != 8)
			.collect(); // If all corners are inside the other we have to remove it
			// Create hole in self
		} else if corners_inside.len() == 4 {
			// similarly create hole, but keep one subblock less
		} else if corners_inside.len() == 1 {*/
		// just remove one quarter of self
		println!("Octtree: {:?}", octtree);
		octtree = octtree
			.into_iter()
			.filter(|x| x.corners_inside(other).len() > 0)
			.collect(); // If all corners are inside the other we have to remove it

		let mut res_blocks = Vec::new();
		for o in octtree {
			res_blocks.extend(o.remove_intersection(other))
		}
		//} else {
		// Should actually never happen
		//	unreachable!()
		//}
		return res_blocks;
	}

	fn octtree(&self, p1: (i64, i64, i64)) -> Vec<Block> {
		// Attention, those blocks overlap at point p!
		if !self.point_inside(p1) {
			println!("{:?} not in {:?}", p1, self);
			return Vec::with_capacity(0);
		}
		/*
		let corners_temp =
			[[self.point.0, self.point.0 + self.edgelength], [self.point.1, self.point.1 + self.edgelength], [self.point.2, self.point.2 + self.edgelength]];
		let corners = corners_temp.iter().multi_cartesian_product();*/
		let mut res = Vec::with_capacity(8);
		let x1 = self.point;
		let x2 = (
			self.point.0 + self.edgelengths.0,
			self.point.1 + self.edgelengths.1,
			self.point.2 + self.edgelengths.2,
		);

		let cubes = [
			((x1.0, x1.1, x1.2), (p1.0 + 1, p1.1 + 1, p1.2 + 1)),
			((x1.0, x1.1, p1.2 + 1), (p1.0 + 1, p1.1 + 1, x2.2)),
			((x1.0, p1.1 + 1, x1.2), (p1.0 + 1, x2.1, p1.2 + 1)),
			((x1.0, p1.1 + 1, p1.2 + 1), (p1.0 + 1, x2.1, x2.2)),
			((p1.0 + 1, x1.1, x1.2), (x2.0, p1.1 + 1, p1.2 + 1)),
			((p1.0 + 1, x1.1, p1.2 + 1), (x2.0, p1.1 + 1, x2.2)),
			((p1.0 + 1, p1.1 + 1, x1.2), (x2.0, x2.1, p1.2 + 1)),
			((p1.0 + 1, p1.1 + 1, p1.2 + 1), (x2.0, x2.1, x2.2)),
		];
		println!("Octtree of {:?}, with pivot {:?}", self, p1);
		for c in cubes {
			println!("{:?}", c);
			res.push(Block::new_from_points(c.0, c.1)); // Works, cause new reorders the corners
		}
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
		let block1 = Block::new_from_points((0, 0, 0), (10, 10, 10));

		println!("=== Check if occtree-buildung works ===");
		let p = (5, 5, 5);
		let blocks = block1.octtree(p);
		for b in blocks.iter() {
			println!("{}    {:?}", b.point_inside(p), b);
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

		println!("=== Check if remove-intersection works ===");
		let block2 = Block::new_from_points((0, 0, 0), (2, 2, 2));
		let mut blocks = block1.remove_intersection(&block2);
		blocks.push(block2);
		println!("New cubes:");
		for b in blocks.iter() {
			println!("{}    {:?}", b.point_inside(p), b);
		}
		println!("Verify every point is covered exactly once:");
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
					println!("Testing ({},{},{}) {}", x, y, z, contained_count);
					assert!(contained_count == 1);
				}
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
	}
}
