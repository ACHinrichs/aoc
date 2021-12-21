use itertools::Itertools;
use nalgebra::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Pointcloud {
	points: Vec<Point3<f64>>,
}

impl Pointcloud {
	fn from_points_ref(points_ref: &Vec<Point3<f64>>) -> Pointcloud {
		let mut points = Vec::new();
		for p in points_ref.iter() {
			points.push(*p);
		}
		Pointcloud { points: points }
	}

	fn from_points(points: Vec<Point3<f64>>) -> Pointcloud {
		Pointcloud { points: points }
	}

	fn merge_from(&mut self, cloud_b: &Pointcloud) -> bool {
		// Find some correlating points, for this we hope that out data is nicely formed
		let mut correlations = Vec::new();
		// Iter over all possible initial correlations
		for (seed_a, seed_b) in
			self.points.iter().cartesian_product(cloud_b.points.iter())
		{
			let distances_a = self
				.points
				.iter()
				.map(|x| nalgebra::distance_squared(x, seed_a));
			let distances_b = cloud_b
				.points
				.iter()
				.map(|x| nalgebra::distance_squared(x, seed_b));
			// Could be done more efficently, but for now i just want a solution tbqf
			let correlating = distances_a
				.cartesian_product(distances_b)
				.filter(|p| p.0 == p.1)
				.collect::<Vec<(f64, f64)>>();
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
			Matrix3::<f64>::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
		'outermost: for axis in 0..3 {
			for axis_direction in [1.0, -1.0] {
				for only_mirror in [true, false] {
					for axis_rotation in 0..4 {
						transform_matrix = Matrix3::new(
							1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
						);
						let theta =
							axis_rotation as f64 / 2.0 * std::f64::consts::PI;
						let rot_matrix = Matrix2::new(
							theta.cos().round(),
							-(theta.sin().round()),
							theta.sin().round(),
							theta.cos().round(),
						);
						//println!("{}", rot_matrix);
						let mut rotx = 0;

						for i in 0..=2 {
							if i == axis {
								continue;
							}
							let mut roty = 0;
							for j in 0..=2 {
								if j == axis {
									continue;
								}
								transform_matrix[(i, j)] =
									rot_matrix[(rotx, roty)];
								roty += 1;
							}
							rotx += 1;
						}

						// Switch the two axis around when we are looking into the other direction
						let mut flip_matrix = Matrix3::new(
							1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
						);
						if axis_direction < 0.0 && !only_mirror {
							let neg_axis = match axis {
								0 => [0, 2],
								1 => [0, 1],
								2 => [1, 2],
								_ => [0, 0],
							};
							for ax in neg_axis {
								flip_matrix[(ax, ax)] = axis_direction;
							}
						} else {
							flip_matrix[(axis, axis)] = axis_direction;
						}
						transform_matrix = transform_matrix * flip_matrix;
						offset = correlations[0].0
							- (transform_matrix * *correlations[0].1);

						let count = correlations.iter().fold(0, |r, x| {
							if nalgebra::distance_squared(
								x.0,
								&((transform_matrix * x.1) + offset),
							) < 0.25
							{
								r + 1
							} else {
								r
							}
						});
						//println!("{}", count);
						if count >= 12 {
							println!("FOUND");
							found_something = true;
							//println!("")
							break 'outermost;
						} else {
							if count > 0 {
								println!(
									"{}/{} are matching",
									count,
									correlations.len()
								)
							}
						}
					}
				}
			}
		}
		if !found_something {
			println!("Somethings fubar!");
		}
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
		//self.points.dedup();
		//println!("{:?}", new_points);
		//Pointcloud::from_points(new_points)
		true
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
			let components: Vec<f64> =
				l.split(",").map(|x| x.parse::<f64>().unwrap()).collect();
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
}
