use std::env;
//use std::fs::File;
//use std::io::BufRead;
//use std::io::BufReader;
use std::collections::HashMap;

#[derive(Eq, Hash, Debug)]
struct Hallway {
	//length: i64;
	fields: [Option<Amphipod>; 11],
	rooms: [Room; 4],
}

#[derive(Eq, Hash, Debug)]
struct Room {
	fields: [Option<Amphipod>; 4],
	target_of: PodColor,
	position: usize,
}

#[derive(Eq, Hash, Debug)]
struct Amphipod {
	color: PodColor,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum PodColor {
	Amber,
	Bronze,
	Copper,
	Desert,
}

impl Amphipod {
	fn cost_per_move(&self) -> i64 {
		match self.color {
			PodColor::Amber => 1,
			PodColor::Bronze => 10,
			PodColor::Copper => 100,
			PodColor::Desert => 1000,
		}
	}

	fn get_target_room_index(&self) -> usize {
		// Is harcoded, not pretty but works fine for this
		match self.color {
			PodColor::Amber => 0,
			PodColor::Bronze => 1,
			PodColor::Copper => 2,
			PodColor::Desert => 3,
		}
	}
}

impl PartialEq for Room {
	fn eq(&self, other: &Room) -> bool {
		self.fields
			.iter()
			.zip(other.fields.iter())
			.all(|(a, b)| a.eq(b))
	}
}

impl PartialEq for Hallway {
	fn eq(&self, other: &Hallway) -> bool {
		self.fields
			.iter()
			.zip(other.fields.iter())
			.all(|(a, b)| a.eq(b))
			&& self
				.rooms
				.iter()
				.zip(other.rooms.iter())
				.all(|(a, b)| a.eq(b))
	}
}

impl PartialEq for Amphipod {
	fn eq(&self, other: &Amphipod) -> bool {
		self.color == other.color
	}
}

impl Clone for Room {
	fn clone(&self) -> Room {
		let mut fields = [None, None, None, None];
		for i in 0..self.fields.len() {
			if self.fields[i].is_some() {
				fields[i] = Some(self.fields[i].as_ref().unwrap().clone());
			}
		}
		Room {
			fields: fields, // it is ok if those are the same elemnts
			position: self.position,
			target_of: self.target_of,
		}
	}
}

impl Clone for Hallway {
	fn clone(&self) -> Self {
		let mut fields: [Option<Amphipod>; 11] = eleven_nones();
		for i in 0..self.fields.len() {
			if self.fields[i].is_some() {
				fields[i] = Some(self.fields[i].as_ref().unwrap().clone());
			}
		}
		Hallway {
			rooms: self.rooms.clone(),
			fields: fields,
		}
	}
}

impl Clone for Amphipod {
	fn clone(&self) -> Self {
		Amphipod { color: self.color }
	}
}

impl std::fmt::Display for Hallway {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut res = "".to_owned();
		for f in self.fields.iter() {
			if f.is_none() {
				res = res + ".";
			} else {
				res = res
					+ match f.as_ref().unwrap().color {
						PodColor::Amber => "A",
						PodColor::Bronze => "B",
						PodColor::Copper => "C",
						PodColor::Desert => "D",
					};
			}
		}

		for f_index in 0..self.rooms[0].fields.len() {
			res = res + "\n  ";
			for r in self.rooms.iter() {
				let field = r.fields[f_index].as_ref();
				if field.is_none() {
					res = res + ".";
				} else {
					res = res
						+ match field.as_ref().unwrap().color {
							PodColor::Amber => "A",
							PodColor::Bronze => "B",
							PodColor::Copper => "C",
							PodColor::Desert => "D",
						};
				}
				res = res + " ";
			}
		}
		write!(f, "Hallway: \n{}", res)
	}
}

fn eleven_nones() -> [Option<Amphipod>; 11] {
	// I cant beleive i really have to do this this way
	[
		None, None, None, None, None, None, None, None, None, None, None,
	]
}
impl Hallway {
	fn create_example() -> Hallway {
		Hallway {
			fields: eleven_nones(),
			rooms: [
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
					],
					position: 2,
					target_of: PodColor::Amber,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
					],
					position: 4,
					target_of: PodColor::Bronze,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
					],
					position: 6,
					target_of: PodColor::Copper,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
					],
					position: 8,
					target_of: PodColor::Desert,
				},
			],
		}
	}

	fn create_input() -> Hallway {
		Hallway {
			fields: eleven_nones(),
			rooms: [
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Amber,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
					],
					position: 2,
					target_of: PodColor::Amber,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
					],
					position: 4,
					target_of: PodColor::Bronze,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Bronze,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
						Some(Amphipod {
							color: PodColor::Desert,
						}),
					],
					position: 6,
					target_of: PodColor::Copper,
				},
				Room {
					fields: [
						Some(Amphipod {
							color: PodColor::Desert,
						}),
						Some(Amphipod {
							color: PodColor::Amber,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
						Some(Amphipod {
							color: PodColor::Copper,
						}),
					],
					position: 8,
					target_of: PodColor::Desert,
				},
			],
		}
	}

	fn get_mincost_move(
		&self,
		cache: &mut HashMap<Hallway, (i64, Vec<Hallway>)>,
		abort_at: i64,
		cost_this_path: i64,
	) -> (i64, Vec<Hallway>) {
		//Check if we are done:
		if cost_this_path >= abort_at {
			//println!("Aborted because path is worse than a found solution");
			//return (i64::MAX, vec![]);
		}

		if self.rooms.iter().all(|room| {
			room.fields.iter().all(|f| {
				f.is_some() && f.as_ref().unwrap().color == room.target_of
			})
		}) {
			println!("Found a winning path with cost {}", cost_this_path);
			return (0, vec![self.clone()]);
		}

		let in_cache_optn = cache.get(self);
		if in_cache_optn.is_some() {
			let unwrapped = in_cache_optn.unwrap();
			if false && unwrapped.0 != i64::MAX {
				println!("Found a winning strategie in cache");
			}
			if unwrapped.1.len() > 0 && false {
				println!(
					"{}\n{}\n\n\n",
					self,
					unwrapped.1[unwrapped.1.len() - 1].clone()
				);
			}
			return (unwrapped.0, unwrapped.1.clone());
		}

		let mut min_cost_winning_path = i64::MAX;
		let mut path = Vec::new();
		// Setting self to the max, to prevent unbounded recursion
		cache.insert(self.clone(), (i64::MAX, Vec::new()));
		// First, check if any pod is infront of a room, should not be the case
		for room in self.rooms.iter() {
			if self.fields[room.position].is_some() {
				panic!("This should not happen with the new logic!");
			}
		}
		// Check if we can move the ones in the rooms
		for (room_index, room) in self.rooms.iter().enumerate() {
			for room_field_index in 0..room.fields.len() {
				if room.fields[room_field_index].is_some() {
					let cur_pod =
						room.fields[room_field_index].as_ref().unwrap().clone();
					let move_cost = cur_pod.cost_per_move();
					if room_field_index < room.fields.len() - 1
						&& cur_pod.color == room.target_of
						&& room.fields[room_field_index + 1..room.fields.len()]
							.iter()
							.all(|x| {
								x.is_none()
									|| x.as_ref().unwrap().color
										== room.target_of
							}) {
						if room.fields[room_field_index + 1].is_none() {
							// Do this only if there is room
							// if the one on 0 belongs into the room and the room is empty move it down…
							//println!("Move down");
							let mut new_hallway = self.clone();
							new_hallway.rooms[room_index].fields
								[room_field_index] = None;
							new_hallway.rooms[room_index].fields
								[room_field_index + 1] = Some(cur_pod.clone());
							let (path_cost, found_path) = new_hallway
								.get_mincost_move(
									cache,
									std::cmp::min(
										abort_at,
										min_cost_winning_path,
									),
									cost_this_path + move_cost,
								);
							if path_cost < i64::MAX
								&& path_cost + move_cost < min_cost_winning_path
							{
								min_cost_winning_path = path_cost + move_cost;
								path = found_path;
							}
						}
					}
					if cur_pod.clone().color != room.target_of
						|| room.fields[room_field_index + 1..room.fields.len()]
							.iter()
							.any(|x| {
								x.is_none()
									|| x.as_ref().unwrap().color
										!= room.target_of
							}) {
						//In this case, either the current pod does not belong here,
						// or there is some pod not belonging here below us
						if room_field_index > 0 {
							// so we move it up
							if room.fields[room_field_index - 1].is_none() {
								// Do this only if there is roomy
								// if the one on 0 belongs into the room and the room is empty move it down…
								//println!("Move down");
								let mut new_hallway = self.clone();
								new_hallway.rooms[room_index].fields
									[room_field_index] = None;
								new_hallway.rooms[room_index].fields
									[room_field_index - 1] = Some(cur_pod);
								let (path_cost, found_path) = new_hallway
									.get_mincost_move(
										cache,
										std::cmp::min(
											abort_at,
											min_cost_winning_path,
										),
										cost_this_path + move_cost,
									);
								if path_cost < i64::MAX
									&& path_cost + move_cost
										< min_cost_winning_path
								{
									min_cost_winning_path =
										path_cost + move_cost;
									path = found_path;
								}
							}
						} else {
							// … or out if we are at 0
							for target_field in 0..self.fields.len() {
								if !self.infront_of_room(target_field)
									&& self.fields_empty(
										target_field,
										room.position,
									) {
									let mut new_hallway = self.clone();
									new_hallway.rooms[room_index].fields[0] =
										None;
									new_hallway.fields[target_field] =
										Some(cur_pod.clone());
									let move_dist = (target_field as i64
										- room.position as i64)
										.abs() + 1;

									let (path_cost, found_path) = new_hallway
										.get_mincost_move(
											cache,
											std::cmp::min(
												abort_at,
												min_cost_winning_path,
											),
											cost_this_path
												+ move_cost * move_dist,
										);
									if path_cost < i64::MAX
										&& path_cost + move_cost * move_dist
											< min_cost_winning_path
									{
										min_cost_winning_path =
											path_cost + move_cost * move_dist;
										path = found_path;
									}
								}
							}
						}
					}
				}
			}
		}

		// Move the ones that are on the hallway (not infront of a room)
		for field_index in 0..self.fields.len() {
			// We only have to move into room because auf condition 3
			if self.fields[field_index].is_none() {
				continue;
			}
			let cur_pod = self.fields[field_index].as_ref().unwrap();
			let target_index = cur_pod.get_target_room_index();

			if self.can_move_into_room(field_index, target_index) {
				let move_cost = ((field_index as i64
					- self.rooms[target_index].position as i64)
					.abs() + 1) * cur_pod.cost_per_move();

				let mut new_hallway = self.clone();
				new_hallway.fields[field_index] = None;
				new_hallway.rooms[target_index].fields[0] =
					Some(cur_pod.clone());

				let (path_cost, found_path) = new_hallway.get_mincost_move(
					cache,
					std::cmp::min(abort_at, min_cost_winning_path),
					cost_this_path + move_cost,
				);
				if path_cost < i64::MAX
					&& path_cost + move_cost < min_cost_winning_path
				{
					min_cost_winning_path = path_cost + move_cost;
					path = found_path;
				}
			}
		}

		// Update cost before terminating
		path.push(self.clone());
		cache.insert(self.clone(), (min_cost_winning_path, path.clone()));
		if min_cost_winning_path == i64::MAX {
			//println!("The only winning move is not to play");
		} else {
			//println!("Winning strategy found!");
		}
		return (min_cost_winning_path, path);
	}

	fn can_move_into_room(
		&self,
		field_index: usize,
		room_index: usize,
	) -> bool {
		if self.fields[field_index].as_ref().unwrap().color
			!= self.rooms[room_index].target_of
		{
			return false;
		}
		// if the room contains a wrong pod at the lower pos, or there is a pod at
		// the upper position, return false
		if self.rooms[room_index].fields.iter().any(|x| {
			x.is_some()
				&& x.as_ref().unwrap().color != self.rooms[room_index].target_of
		}) || self.rooms[room_index].fields[0].is_some()
		{
			return false;
		}
		// Since the room is applicable now, we can move into it iff every field between
		// the room and us is empty:
		return self.fields[std::cmp::min(
			self.rooms[room_index].position,
			field_index + 1,
		)
			..std::cmp::max(self.rooms[room_index].position + 1, field_index)]
			.iter()
			.all(|f| f.is_none());
	}

	fn infront_of_room(&self, field: usize) -> bool {
		self.rooms.iter().any(|x| x.position == field)
	}

	fn fields_empty(&self, start: usize, end: usize) -> bool {
		self.fields[std::cmp::min(start, end)..std::cmp::max(start, end) + 1]
			.iter()
			.all(|x| x.is_none())
	}
}

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 3 {
		panic!("Please supply the problem-input and the task to solve as arguments!");
	}
	let task: u64 = args[2].parse().expect(&format!(
		"{} is not a valid task, please supply either 1 or 2!",
		args[1]
	));
	//let file = File::open(&args[1])
	//	.expect(&format!("Inputfile {} not found", args[1]));
	let start_configuration;
	if args[1] == "input" {
		println!("Now solving Part {} on real input", args[2]);
		start_configuration = Hallway::create_input();
	} else {
		println!("Now solving Part {} on example-input", args[2]);
		start_configuration = Hallway::create_example();
	}

	if task == 2 {
		let mut cache = HashMap::new();
		let (res, path) =
			start_configuration.get_mincost_move(&mut cache, i64::MAX, 0);
		println!("Min-Cost strategy has cost {}", res);
		println!("Path:");
		for h in path.iter().rev() {
			let cached = cache.get(h);

			println!("{}", h);
			if cached.is_some() {
				println!("Cost Remaining: {}", cached.unwrap().0);
			}
		}
	//println!("Cache is: {:?}", cache);
	} else if task == 1 {
	} else {
		panic!("{} is not a valid task, please supply either 1 or 2!", task);
	}
}
