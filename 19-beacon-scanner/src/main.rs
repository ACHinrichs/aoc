use nalgebra::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
struct Pointcloud {
	points: Vec<Point3<f64>>,
	//position: Vector3<f64>,
	//rotation: Rotation3<f64>,
	//fixed: bool
}

fn count_overlap_sorted(a: &Vec<f64>, b: &Vec<f64>) -> i64 {
	if a.len() == 0 || b.len() == 0 {
		return 0;
	}
	let mut res = 0;
	let mut iter_a = a.iter();
	let mut elem_a = iter_a.nth(0).unwrap();
	for elem_b in b.iter() {
		while elem_b > elem_a {
			let elem_a_optn = iter_a.nth(0);
			if elem_a_optn.is_some() {
				elem_a = elem_a_optn.unwrap();
			} else {
				break;
			}
		}
		if elem_a == elem_b {
			res += 1;
			let elem_a_optn = iter_a.nth(0);
			if elem_a_optn.is_some() {
				elem_a = elem_a_optn.unwrap();
			} else {
				break;
			}
		}
	}
	res
}

fn comp_mul(a: Point3<f64>, b: Point3<f64>) -> Point3<f64> {
	Point3::new(a.x * b.x, a.y * b.y, a.z * b.z)
}

impl Pointcloud {
	fn from_points_ref(points_ref: &Vec<Point3<f64>>) -> Pointcloud {
		let mut points = Vec::new();
		for p in points_ref.iter() {
			points.push(*p);
		}
		Pointcloud {
			points: points,
			//position: Vector3::new(0.0, 0.0, 0.0),
			//rotation: Rotation3::new(vector!(0.0, 0.0, 0.0)),
			//fixed: false,
		}
	}

	fn from_points(points: Vec<Point3<f64>>) -> Pointcloud {
		Pointcloud {
			points: points,
			//position: Vector3::new(0.0, 0.0, 0.0),
			//rotation: Rotation3::new(vector!(0.0, 0.0, 0.0)),
			//fixed: false,
		}
	}

	fn merge_from(&mut self, cloud_b: &Pointcloud) -> bool {
		// Find some correlating points, for this we hope that out data is nicely formed
		//let correlating = Vec::new();
		let mut correlations = Vec::new();
		println!("{} {}", self.points.len(), cloud_b.points.len());
		//loop {
		for candidate_a in self.points.iter() {
			for candidate_b in cloud_b.points.iter() {
				let mut aligned = Vec::new();
				for chck_a in self.points.iter(){
					if chck_a == candidate_a {
						continue;
					}
					let dist_a = nalgebra::distance_squared(chck_a, candidate_a);
					for chck_b in cloud_b.points.iter(){
						if chck_b == candidate_b{
							continue;
						}
						let dist_b = nalgebra::distance_squared(chck_b, candidate_b);
						if dist_a == dist_b {
							aligned.push((chck_a, chck_b));
						} else if dist_a - dist_b < 0.5 && dist_b - dist_a < 0.5  {
							assert!(false);
							println!("Yeah, you're just retarded, thats why it didnt work");
						}
					}
				}
				let overlap_count = aligned.len() + 1;
				if overlap_count >= 6 {
					println!(
						"{} and {} have {} overlaps",
						candidate_a, candidate_b, overlap_count
					);
					correlations.push((candidate_a, candidate_b));
					correlations.extend(aligned.iter());
					break;
				} else {
					//println!("{} {} {}", self.points.len(), cloud_b.points.len(), overlap_count);
				}
			}
		}
		println!("{} Correlations", correlations.len());
		if correlations.len() < 12 {
			return false;
		}
		// We could do this smart, or we brute-force it. Since bruteforcing can be done
		// in adequate time, I opt for letting my brain sleep
		//let mut final_scale: Point3<f64> = scales[0];
		//let mut final_transformation: Similarity3<f64> = Similarity3::new(
		//	correlations[0].0 - comp_mul(final_scale, *correlations[0].1),
		//	Vector3::new(0.0, 0.0, 0.0),
		//	1.0,
		//);
		let mut offset = Vector3::<f64>::new(0.0, 0.0, 0.0);
		let mut transform_matrix =
			Matrix3::<f64>::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0);
		let mut found_something = false;
		'outermost: for axis in 0..3 {
			for axis_direction in [1.0, -1.0] {
				for axis_rotation in 0..4 {
					transform_matrix = Matrix3::new(
						1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0,
					);
					let theta =
						axis_rotation as f64 / 2.0 * std::f64::consts::PI;
					let rot_matrix = Matrix2::new(
						theta.cos().round(),
						-theta.sin().round(),
						theta.sin().round(),
						theta.cos().round(),
					);
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
							transform_matrix[(i, j)] = rot_matrix[(rotx, roty)];
							roty += 1;
						}
						rotx += 1;
					}
					transform_matrix[(axis, axis)] = axis_direction;
					if axis_direction < 0.0 {
						let switch_pair = match axis {
							0 => (1, 2),
							1 => (0, 2),
							2 => (1, 2),
							_ => (0, 0),
						};
						for i in 0..3 {
							let puf = transform_matrix[(i, switch_pair.0)];
							transform_matrix[(i, switch_pair.0)] =
								transform_matrix[(i, switch_pair.1)];
							transform_matrix[(i, switch_pair.1)] = puf;
						}
					}

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
					println!("{}", count);
					if count > 12 {
						println!("FOUND, {} {}", transform_matrix, offset);
						found_something = true;
						break 'outermost;
					} else {
					}
				}
			}
		}
		if !found_something {
			println!("Somethings fubar!");
			//for c in correlations {
			//	println!("{};{}", c.0, c.1)
			//}
			//return false;
		}
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
			println!("{:?}", cloud.points);
			if !first.merge_from(&cloud) {
				not_matched.push(cloud);
			}
		}
		to_add = not_matched;
	}

	println!("Number of points is: {}", first.points.len());
}
