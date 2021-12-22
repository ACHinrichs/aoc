use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Block {
	x1: i64,
	y1: i64,
	z1: i64,
	x2: i64,
	y2: i64,
	z2: i64,
}

impl Clone for Block {
	fn clone(&self) -> Block {
		return Block::new(
			(self.x1, self.y1, self.z1),
			(self.x2, self.y2, self.z2),
		);
	}
}
impl Copy for Block {}

impl Block {
	fn new(p1: (i64, i64, i64), p2: (i64, i64, i64)) -> Block {
		return Block {
			x1: std::cmp::min(p1.0, p2.0),
			y1: std::cmp::min(p1.1, p2.1),
			z1: std::cmp::min(p1.2, p2.2),
			x2: std::cmp::max(p1.0, p2.0),
			y2: std::cmp::max(p1.1, p2.1),
			z2: std::cmp::max(p1.2, p2.2),
		};
	}

	fn point_inside(&self, p: (i64, i64, i64)) -> bool {
		return self.x1 <= p.0
			&& p.0 <= self.x2
			&& self.y1 <= p.1
			&& p.1 <= self.y2
			&& self.z1 <= p.2
			&& p.2 <= self.z2;
	}

	// yea, i know that inside vec is bullshit, but i cant be bothered to fix this
	//fn corners(&self) -> dyn std::iter::Iterator<Item = Vec<&i64>> {
	//	let corners_temp =
	//		[[self.x1, self.x2], [self.y1, self.y2], [self.z1, self.z2]];
	//	return dyn corners_temp.iter().multi_cartesian_product().collect();
	//}

	// returns all corners from self that are contained in other
	fn corners_inside(&self, other: &Block) -> Vec<(i64, i64, i64)> {
		let corners_temp =
			[[self.x1, self.x2], [self.y1, self.y2], [self.z1, self.z2]];
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
		let mut corners_inside = self.corners_inside(other);
		if corners_inside.len() == 8 {
			// self lies completly within other, so an empty vec is returned
			println!("{:?}", self);
			return Vec::new();
		} else if corners_inside.len() == 0 {
			//switch the corners around, so that this works if slef is larger than other
			corners_inside = other.corners_inside(self);
			if corners_inside.len() == 0 {
				return vec![*self];
			}
		}
		println!("partially contained {}", corners_inside.len());
		let mut octtree =
			self.octtree(corners_inside.clone().into_iter().fold(
				(self.x2, self.y2, self.z2),
				|r, x| {
					(
						std::cmp::min(x.0, r.0),
						std::cmp::min(x.1, r.1),
						std::cmp::min(x.2, r.2),
					)
				},
			));
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

		// Adapt octtree coordinates for those cubes that should not have an intersection
		for index in 0..octtree.len() {
			if other.contains((octtree[index].x2,octtree[index].y2, octtree[index].z2)
			if octtree[index].x2 == other.x1 {
				octtree[index].x2 -= 1;
			}
			if octtree[index].y2 == other.y1 {
				octtree[index].y2 -= 1;
			}
			if octtree[index].z2 == other.z1 {
				octtree[index].z2 -= 1;
			}
		}
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
			[[self.x1, self.x2], [self.y1, self.y2], [self.z1, self.z2]];
		let corners = corners_temp.iter().multi_cartesian_product();*/
		let mut res = Vec::with_capacity(8);
		let x1 = (self.x1, self.y1, self.z1);
		let x2 = (self.x2, self.y2, self.z2);
		let cubes = [
			((x1.0, x1.1, x1.2), (p1.0, p1.1, p1.2)),
			((p1.0, x1.1, x1.2), (x2.0, p1.1, p1.2)),
			((p1.0, p1.1, x1.2), (x2.0, x2.1, p1.2)),
			((x1.0, p1.1, x1.2), (p1.0, x2.1, p1.2)),
			((x1.0, x1.1, p1.2), (p1.0, p1.1, x2.2)),
			((p1.0, x1.1, p1.2), (x2.0, p1.1, x2.2)),
			((p1.0, p1.1, p1.2), (x2.0, x2.1, x2.2)),
			((x1.0, p1.1, p1.2), (p1.0, x2.1, x2.2)),
		];
		println!("Octtree of {:?}, with pivot {:?}", self, p1);
		for c in cubes {
			println!("{:?}", c);
			res.push(Block::new(c.0, c.1)); // Works, cause new reorders the corners
		}
		return res;
	}
}

fn main() {
	let file = File::open("example.txt").expect("file not found");
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

	let block1 = Block::new((2, 2, 2), (0, 0, 0));
	let block2 = Block::new((0, 0, 0), (10, 10, 10));
	println!("{:?}\n", block1);
	println!("{:?}\n", block2);
	println!("{:?}", block2.remove_intersection(&block1));
	println!("{:?}", block2.octtree((5, 5, 5)));
}
