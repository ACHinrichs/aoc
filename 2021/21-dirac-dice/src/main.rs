use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
trait Die {
	fn roll(&mut self) -> u64;
}

struct DeterministicDie {
	rolls: u64,
}

impl DeterministicDie {
	fn new() -> DeterministicDie {
		DeterministicDie { rolls: 0 }
	}
}

impl Die for DeterministicDie {
	fn roll(&mut self) -> u64 {
		let res = self.rolls % 100 + 1; // Because we are offset
		self.rolls = self.rolls + 1;
		return res;
	}
}

static mut finished_runs: u64 = 0;
static mut from_hashmap: u64 = 0;

static winning_score: u64 = 21;

fn get_wins(
	points: Vec<u64>,
	positions: Vec<u64>,
	cur_player: usize,
	known_configurations: &mut HashMap<(Vec<u64>, Vec<u64>, usize), Vec<u64>>,
) -> Vec<u64> {
	// points and positions are the state __after__ cur_player made his move
	let map_entry = known_configurations.get(&(
		points.clone(),
		positions.clone(),
		(cur_player + 1) % points.len(),
	));
	if map_entry.is_some() {
		unsafe {
			finished_runs += 1;
			from_hashmap += 1;
			if finished_runs % 10_000 == 0 {
				println!("Already finished {} games ({} from Hashmap), HashMap-Size is {}", finished_runs, from_hashmap, known_configurations.len());
			}
		}
		return map_entry.unwrap().clone();
	} else {
		let mut res = Vec::new();
		if points[cur_player] >= winning_score {
			for i in 0..points.len() {
				res.push(if i == cur_player { 1 } else { 0 });
			}
		} else {
			res = play_quantum(
				points.clone(),
				positions.clone(),
				(cur_player + 1) % points.len(),
				known_configurations,
			);
		}
		unsafe {
			finished_runs += 1;
			if finished_runs % 10_000 == 0 {
				println!(
					"Already finished {} games, HashMap-Size is {}",
					finished_runs,
					known_configurations.len()
				);
			}
		}
		known_configurations.insert(
			(
				points.clone(),
				positions.clone(),
				(cur_player + 1) % points.len(),
			),
			res.clone(),
		);
		return res;
	}
}

fn play_quantum(
	points: Vec<u64>,
	positions: Vec<u64>,
	cur_player: usize,
	known_configurations: &mut HashMap<(Vec<u64>, Vec<u64>, usize), Vec<u64>>,
) -> Vec<u64> {
	let possible_rolls = [1, 2, 3];

	let mut res = Vec::new();
	//println!("{:?}", points);
	for p in points.clone() {
		res.push(0);
	}
	//for roll in [3,4,5,6,7,8,9] {
	for roll1 in possible_rolls {
		for roll2 in possible_rolls {
			for roll3 in possible_rolls {
				let roll = roll1 + roll2 + roll3;

				let mut points_tmp = points.clone();
				let mut positions_tmp = positions.clone();

				positions_tmp[cur_player] =
					(positions_tmp[cur_player] + roll) % 10;
				points_tmp[cur_player] += positions_tmp[cur_player] + 1;

				let subtree_res = get_wins(
					points_tmp,
					positions_tmp,
					cur_player,
					known_configurations,
				);

				res = res.iter().zip(subtree_res).map(|(x, y)| x + y).collect();
			}
		}
	}
	//if res[cur_player] % 100000 == 0{
	//	println!("Player {} has won in {} Universes",cur_player, res[cur_player] )
	//}
	//println!("{:?}, {:?}, {:?}",positions.clone(),
	//							 points.clone(),
	//							 cur_player );
	return res;
}

fn main() {
	let part = 2;
	let file = File::open("input.txt").expect("file not found");
	let mut positions = BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().to_string())
		.filter(|x| {
			if x.starts_with("//") {
				//Filter comments
				//println!("Found comment in input: {}", &x[2..]);
				false
			} else {
				//Filter empty lines
				x != ""
			}
		})
		.map(|x| x.split(": ").nth(1).unwrap().parse::<u64>().unwrap() - 1) //MIND THE OFFSET
		.collect::<Vec<u64>>();
	let mut points = Vec::new();
	for i in 0..positions.len() {
		points.push(0);
	}

	if part == 1 {
		let mut die = DeterministicDie::new();
		'game_loop: loop {
			println!("----- New round -----");
			for player in 0..positions.len() {
				let roll = die.roll() + die.roll() + die.roll();
				positions[player] = (positions[player] + roll) % 10;
				points[player] += positions[player] + 1; //Mind the offset!
				println!(
					"Player {} \n Position: {} Points: {}",
					player, positions[player], points[player]
				);
				if points[player] >= 1000 {
					break 'game_loop;
				}
			}
		}
		println!(
			"rolls * score of loosing player (Sol. Pt. 1) is {}",
			die.rolls
				* points.iter().reduce(|x, y| std::cmp::min(x, y)).unwrap()
		);
	} else {
		println!("Playing the Quantum-Version.");
		let winning_universes =
			play_quantum(points, positions, 0, &mut HashMap::new());
		println!("Done");
		for p in 0..winning_universes.len() {
			println!(
				"Player {} winns in {} universes.",
				p, winning_universes[p]
			);
		}
	}
}
