use itertools::Itertools;
use nalgebra::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Pointcloud {
	points: Vec<Point3<i64>>,
	scanners: Vec<Point3<i64>>,
}

fn distance_squared(a: &Point3<i64>, b: &Point3<i64>) -> i64 {
	return (b.x - a.x).pow(2) + (b.y - a.y).pow(2) + (b.z - a.z).pow(2);
}
fn manhattan_distance(a: &Point3<i64>, b: &Point3<i64>) -> i64 {
	return (b.x - a.x).abs() + (b.y - a.y).abs() + (b.z - a.z).abs();
}

impl Pointcloud {
	fn from_points_ref(points_ref: &Vec<Point3<i64>>) -> Pointcloud {
		let mut points = Vec::new();
		let mut scanners = Vec::new();
		scanners.push(Point3::new(0, 0, 0));
		for p in points_ref.iter() {
			points.push(*p);
		}
		Pointcloud {
			points: points,
			scanners: scanners,
		}
	}

	fn from_points(points: Vec<Point3<i64>>) -> Pointcloud {
		let mut scanners = Vec::new();
		scanners.push(Point3::new(0, 0, 0));
		Pointcloud {
			points: points,
			scanners: scanners,
		}
	}

	fn merge_from(&mut self, cloud_b: &Pointcloud) -> bool {
		// Find some correlating points, for this we hope that out data is nicely formed
		let mut correlations = Vec::new();
		// Iter over all possible initial correlations
		for (seed_a, seed_b) in
			self.points.iter().cartesian_product(cloud_b.points.iter())
		{
			let distances_a =
				self.points.iter().map(|x| distance_squared(x, seed_a));
			let distances_b =
				cloud_b.points.iter().map(|x| distance_squared(x, seed_b));
			// Could be done more efficently, but for now i just want a solution tbqf
			let correlating = distances_a
				.cartesian_product(distances_b)
				.filter(|p| p.0 == p.1)
				.collect::<Vec<(i64, i64)>>();
			if correlating.len() >= 12 {
				correlations.push((seed_a, seed_b));
				//println!("ADDED ONE");
			} /* else {
				 if correlating.len() > 2 {
					 println!("{}", correlating.len());
				 }
			 }*/
		}

		if correlations.len() > 0 {
			println!("{} Correlations", correlations.len());
		} else {
			println!(
				"You only got {} correlations, thats not quite enough",
				correlations.len()
			);
			return false;
		}
		// If we have come this far, there __has__ to exist a transformation and offset
		// such that this stuff aligns

		// We could do this smart, or we brute-force it. Since bruteforcing can be done
		// in adequate time, I opt for letting my brain sleep
		let mut found_something = false;
		let mut offset = correlations[0].0 - correlations[0].1;
		//assert!(correlations.iter().all(|x| *x.0 == x.1 - offset));
		let mut transform_matrix =
			Matrix3::<i64>::new(0, 0, 0, 0, 0, 0, 0, 0, 0);

		'outer: for x_maps_to in [0, 1, 2] {
			for x_sign in [-1, 1] {
				'y_map: for y_maps_to in [0, 1, 2] {
					if y_maps_to == x_maps_to {
						continue 'y_map;
					}
					for y_sign in [-1, 1] {
						let z_maps_to = match (x_maps_to, y_maps_to) {
							(0, 1) | (1, 0) => 2,
							(0, 2) | (2, 0) => 1,
							_ => 0,
						};
						for z_sign in [-1, 1] {
							transform_matrix =
								Matrix3::new(0, 0, 0, 0, 0, 0, 0, 0, 0);
							transform_matrix[(x_maps_to, 0)] = x_sign;
							transform_matrix[(y_maps_to, 1)] = y_sign;
							transform_matrix[(z_maps_to, 2)] = z_sign;
							offset = correlations[0].0
								- transform_matrix * correlations[0].1;
							if correlations.iter().all(|x| {
								*x.0 == (transform_matrix * x.1) + offset
							}) {
								println!("FOUND");
								//println!("{:?}", correlations);
								//println!("Inside {:?}", transform_matrix);
								found_something = true;
								break 'outer;
							}
						}
					}
				}
			}
		}
		if !found_something {
			println!("Somethings fubar!");
		}
		//println!("Outside {:?}", transform_matrix);
		assert!(correlations
			.iter()
			.all(|x| *x.0 == (transform_matrix * x.1) + offset));
		let mut new_points = Vec::new();
		new_points.extend(self.points.iter());
		//new_points.extend(cloud_b.points.iter());
		new_points.extend(
			cloud_b
				.points
				.iter()
				.filter(|y| correlations.iter().all(|x| x.1 != *y)) //Remove duplicats
				.map(|x| transform_matrix * x + offset),
		);
		self.points = new_points;
		self.scanners.push(Point3::new(0, 0, 0) + offset);
		//self.points.dedup();
		//println!("{:?}", new_points);
		//Pointcloud::from_points(new_points)
		true
	}
}

fn main() {
	let file = File::open("input.txt").expect("file not found");
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

	let mut pointclouds: Vec<Pointcloud> = Vec::new();
	let mut points = Vec::new();
	for l in lines {
		if l.starts_with("---") {
			//New Pointcloud, we dont care which number (i hope)
			if &points.len() > &0 {
				// Catch first pointcloud
				pointclouds.push(Pointcloud::from_points(points));
				points = Vec::new();
			}
		} else {
			let components: Vec<i64> =
				l.split(",").map(|x| x.parse::<i64>().unwrap()).collect();
			points.push(Point::from(vector!(
				components[0],
				components[1],
				components[2]
			)));
		}
	}
	pointclouds.push(Pointcloud::from_points(points));
	//let mut iter = pointclouds.iter();
	println!("We have {} scanners", pointclouds.len());
	let mut first = Pointcloud::from_points_ref(&pointclouds[0].points);
	let mut to_add = pointclouds;
	to_add.remove(0);
	while to_add.len() > 0 {
		println!("Still {} to match", to_add.len());
		let mut not_matched = Vec::new();
		for cloud in to_add {
			//println!("{:?}", cloud.points);
			if !first.merge_from(&cloud) {
				not_matched.push(cloud);
			}
		}
		to_add = not_matched;
	}

	println!("Number of points is: {}", first.points.len());

	// Find max manhattan distance
	// Using slow solution, cause it is faster to implement
	println!(
		"Max Manhattan distance is: {}",
		first
			.scanners
			.iter()
			.cartesian_product(first.scanners.iter())
			.fold(0, |r, x| std::cmp::max(r, manhattan_distance(x.0, x.1)))
	);
}
