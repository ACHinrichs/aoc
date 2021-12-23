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
	fields: [Option<Amphipod>; 2],
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
		let mut fields = [None, None];
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
					],
					position: 8,
					target_of: PodColor::Desert,
				},
			],
		}
	}

	fn create_input() -> Hallway {
		Hallway::create_example()
	}

	fn get_mincost_move(
		&self,
		cache: &mut HashMap<Hallway, i64>,
		abort_at: i64,
	) -> i64 {
		//Check if we are done:
		if self.rooms.iter().all(|room| {
			room.fields.iter().all(|f| {
				if f.is_some() && f.as_ref().unwrap().color == room.target_of {
					//println!("{:?}", room.target_of);
					true
				} else {
					/*println!(
						"{:?} {:?}",
						if f.is_some() {
							Some(f.as_ref().unwrap().color)
						} else {
							None
						},
						room.target_of
					);// */
					false
				}
			})
		}) {
			println!("Found a winning path");
			return 0;
		}

		let in_cache_optn = cache.get(self);
		if in_cache_optn.is_some() {
			/*println!(
				"Already visited  {:?},\n\n {:?}",
				self,
				in_cache_optn.unwrap()
			);// */
			return *in_cache_optn.unwrap();
		}

		let mut min_cost_winning_path = i64::MAX;
		// Setting self to the max, to prevent unbounded recursion
		cache.insert(self.clone(), i64::MAX);
		// First, check if any pod is infront of a room, this one has to move then
		for (room_index, room) in self.rooms.iter().enumerate() {
			if self.fields[room.position].is_some() {
				// Possible moves: The Pod moves away from the opening,
				// or the pod moves into the room if possible.
				let cur_pod_ref = self.fields[room.position].as_ref().unwrap();
				let move_cost = cur_pod_ref.cost_per_move();
				if self.fields[room.position - 1].is_none() {
					// Pod moves away to the left
					let mut new_hallway = self.clone();
					new_hallway.fields[room.position - 1] =
						Some(cur_pod_ref.clone());
					new_hallway.fields[room.position] = None;
					let path_cost =
						new_hallway.get_mincost_move(cache, abort_at);
					if path_cost < i64::MAX {
						min_cost_winning_path = std::cmp::min(
							min_cost_winning_path,
							path_cost + move_cost,
						);
					}
					//cache.insert(new_hallway, path_cost);
				}
				if self.fields[room.position + 1].is_none() {
					// Pod moves away to the right
					// ToDo remove code duplication
					let mut new_hallway = self.clone();
					new_hallway.fields[room.position + 1] =
						Some(cur_pod_ref.clone());
					new_hallway.fields[room.position] = None;
					let path_cost =
						new_hallway.get_mincost_move(cache, abort_at);
					if path_cost < i64::MAX {
						min_cost_winning_path = std::cmp::min(
							min_cost_winning_path,
							path_cost + move_cost,
						);
					}
					//cache.insert(new_hallway, path_cost);
				}
				if cur_pod_ref.color == room.target_of
					&& room.fields[0].is_none() // Upper one has to be empty
					&& (room.fields[1].is_none() // lower one has to be empty …
						|| room.fields[1].as_ref().unwrap().color == room.target_of)
				//… or the pod there has to belong there
				{
					//println!("Move into room");
					let mut new_hallway = self.clone();
					new_hallway.rooms[room_index].fields[0] =
						Some(cur_pod_ref.clone());
					new_hallway.fields[room.position] = None;
					let path_cost =
						new_hallway.get_mincost_move(cache, abort_at);
					if path_cost < i64::MAX {
						min_cost_winning_path = std::cmp::min(
							min_cost_winning_path,
							path_cost + move_cost,
						);
					}
					//cache.insert(new_hallway, path_cost);
				}
				cache.insert(self.clone(), min_cost_winning_path);
				return min_cost_winning_path; // If we have found one of those, we may not make any other moves now!
			}
		}
		// Check if we can move the ones in the rooms
		for (room_index, room) in self.rooms.iter().enumerate() {
			if room.fields[0].is_some() {
				let cur_pod = room.fields[0].as_ref().unwrap().clone();
				let move_cost = cur_pod.cost_per_move();
				if cur_pod.color == room.target_of {
					if room.fields[1].is_none() {
						// if the one on 0 belongs into the room move down…
						//println!("Move down");
						let mut new_hallway = self.clone();
						new_hallway.rooms[room_index].fields[0] = None;
						new_hallway.rooms[room_index].fields[1] = Some(cur_pod);
						let path_cost =
							new_hallway.get_mincost_move(cache, abort_at);
						if path_cost < i64::MAX {
							min_cost_winning_path = std::cmp::min(
								min_cost_winning_path,
								path_cost + move_cost,
							);
						}
					//cache.insert(new_hallway, path_cost);
					} else if room.fields[1].as_ref().unwrap().color
						!= room.target_of && self.fields[room.position]
						.is_none()
					{
						/*println!(
							"Move {:?} out of room {} to give way to {:?}",
							cur_pod,
							room_index,
							room.fields[1].as_ref().unwrap().color
						);// */
						let mut new_hallway = self.clone();
						new_hallway.rooms[room_index].fields[0] = None;
						new_hallway.fields[room.position] = Some(cur_pod);
						let path_cost =
							new_hallway.get_mincost_move(cache, abort_at);
						if path_cost < i64::MAX {
							min_cost_winning_path = std::cmp::min(
								min_cost_winning_path,
								path_cost + move_cost,
							);
						}
						// … or up to not block in a wrong-colored one
					}
				} else {
					// if the one on 0 does not belong into the room only move up
					if self.fields[room.position].is_none() {
						/*println!(
							"Move {:?} out of room {}",
							cur_pod, room_index
						);// */
						let mut new_hallway = self.clone();
						new_hallway.rooms[room_index].fields[0] = None;
						new_hallway.fields[room.position] = Some(cur_pod);
						let path_cost =
							new_hallway.get_mincost_move(cache, abort_at);
						if path_cost < i64::MAX {
							min_cost_winning_path = std::cmp::min(
								min_cost_winning_path,
								path_cost + move_cost,
							);
						}
						//cache.insert(new_hallway, path_cost);
					}
				}
			} else {
				if room.fields[1].is_some() {
					let cur_pod = room.fields[1].as_ref().unwrap().clone();
					// if the one on 1 does not belong into the room we can move it up
					if cur_pod.color != room.target_of {
						//println!("Moving up");
						let move_cost = cur_pod.cost_per_move();
						let mut new_hallway = self.clone();
						new_hallway.rooms[room_index].fields[1] = None;
						new_hallway.rooms[room_index].fields[0] = Some(cur_pod);
						let path_cost =
							new_hallway.get_mincost_move(cache, abort_at);
						if path_cost < i64::MAX {
							min_cost_winning_path = std::cmp::min(
								min_cost_winning_path,
								path_cost + move_cost,
							);
						}
					}
					//cache.insert(new_hallway, path_cost);
				}
			}
		}
		// Move the ones that are on the hallway (not infront of a room)
		for field_index in 0..self.fields.len() {
			if self.fields[field_index].is_none() {
				continue;
			}
			let cur_pod = self.fields[field_index].as_ref().unwrap();
			let move_cost = cur_pod.cost_per_move();
			if field_index >= 1 {
				// Move to the left
				let mut new_hallway = self.clone();
				new_hallway.fields[field_index] = None;
				new_hallway.fields[field_index - 1] = Some(cur_pod.clone());
				let path_cost = new_hallway.get_mincost_move(cache, abort_at);
				if path_cost < i64::MAX {
					min_cost_winning_path = std::cmp::min(
						min_cost_winning_path,
						path_cost + move_cost,
					);
				}
				//cache.insert(new_hallway, path_cost);
			}
			if field_index < self.fields.len() - 1 {
				// Move to the right
				let mut new_hallway = self.clone();
				new_hallway.fields[field_index] = None;
				new_hallway.fields[field_index + 1] = Some(cur_pod.clone());
				let path_cost = new_hallway.get_mincost_move(cache, abort_at);
				if path_cost < i64::MAX {
					min_cost_winning_path = std::cmp::min(
						min_cost_winning_path,
						path_cost + move_cost,
					);
				}
				//cache.insert(new_hallway, path_cost);
			}
		}
		// Update cost before terminating
		cache.insert(self.clone(), min_cost_winning_path);
		if min_cost_winning_path == i64::MAX {
			//println!("The only winning move is not to play");
		} else {
			//println!("Winning strategy found!");
		}
		return min_cost_winning_path;
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

	if task == 1 {
		let mut cache = HashMap::new();
		println!(
			"Min-Cost strategy has cost {}",
			start_configuration.get_mincost_move(&mut cache, i64::MAX)
		);
	//println!("Cache is: {:?}", cache);
	} else if task == 2 {
	} else {
		panic!("{} is not a valid task, please supply either 1 or 2!", task);
	}
}
