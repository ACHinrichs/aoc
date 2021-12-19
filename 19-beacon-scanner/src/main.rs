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
		let mut candidates_b = cloud_b.points.iter();
		//let correlating = Vec::new();
		let mut correlations = Vec::new();
		println!("{:?}\n {:?}", self.points, cloud_b.points);
		loop {
			let candidate_b_optn = candidates_b.nth(0);
			if candidate_b_optn.is_none() {
				break;
			}
			let candidate_b = candidate_b_optn.unwrap();
			let mut distances_b: Vec<f64> = cloud_b
				.points
				.iter()
				.map(|x| nalgebra::distance_squared(x, candidate_b)) // is faster
				.collect::<Vec<f64>>();
			distances_b.sort_by(|a, b| a.partial_cmp(b).unwrap());
			for candidate_a in self.points.iter() {
				let mut distances_a = self
					.points
					.iter()
					.map(|x| nalgebra::distance_squared(x, candidate_a)) // is faster
					.collect::<Vec<f64>>();
				distances_a.sort_by(|a, b| a.partial_cmp(b).unwrap());
				let overlap_count =
					count_overlap_sorted(&distances_a, &distances_b);
				//Fortunately our input is well-formed, hence we can just take all pairs where there are 12 overlapping distances
				if overlap_count >= 12 {
					println!(
						"{} and {} have {} overlaps",
						candidate_a, candidate_b, overlap_count
					);
					correlations.push((candidate_a, candidate_b))
				}
			}
		}

		// We could do this smart, or we brute-force it. Since bruteforcing can be done
		// in adequate time, I opt for letting my brain sleep
		let scales = [
			Point3::new(1.0, 1.0, 1.0),
			Point3::new(1.0, 1.0, -1.0),
			Point3::new(1.0, -1.0, 1.0),
			Point3::new(1.0, -1.0, -1.0),
			Point3::new(-1.0, 1.0, 1.0),
			Point3::new(-1.0, 1.0, -1.0),
			Point3::new(-1.0, -1.0, 1.0),
			Point3::new(-1.0, -1.0, -1.0),
		];
		let mut final_scale: Point3<f64> = scales[0];
		let mut final_transformation: Similarity3<f64> = Similarity3::new(
			correlations[0].0 - comp_mul(final_scale, *correlations[0].1),
			Vector3::new(0.0, 0.0, 0.0),
			1.0,
		);
		'outermost: for scale in scales {
			for rot_x in 0..4 {
				for rot_y in 0..4 {
					for rot_z in 0..4 {
						let rotation = Vector3::new(
							rot_x as f64 * std::f64::consts::PI,
							rot_y as f64 * std::f64::consts::PI,
							rot_z as f64 * std::f64::consts::PI,
						);

						let transformation = Similarity3::new(
							correlations[0].0
								- comp_mul(scale, *correlations[0].1),
							rotation,
							1.0,
						);
						if correlations.iter().all(|x| {
							final_transformation = transformation;
							final_scale = scale;
							*x.0 == transformation
								.transform_point(&(comp_mul(scale, *x.1)))
						}) {
							println!("FOUND");
							break 'outermost;
						} else {
						}
					}
				}
			}
		}

		let mut new_points = Vec::new();
		new_points.extend(self.points.iter());
		new_points.extend(
			cloud_b
				.points
				.iter()
				.filter(|y| correlations.iter().all(|x| x.1 != *y)) //Remove duplicats
				.map(|x| {
					final_transformation
						.transform_point(&(comp_mul(final_scale, *x)))
				}),
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
	let mut iter = pointclouds.iter();
	let mut first = Pointcloud::from_points_ref(&iter.nth(0).unwrap().points);
	let mut merged_cloud =
		iter.filter(|x| !Pointcloud::merge_from(&mut first, &x));
}
