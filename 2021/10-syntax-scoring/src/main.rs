use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt").expect("file not found");
	let lines = BufReader::new(file)
		.lines()
		.map(|x| x.unwrap().chars().collect())
		.collect::<Vec<Vec<char>>>();

	let mut part_1_score = 0;
	let mut part_2_scores: Vec<u64> = Vec::new();
	for l in lines {
		let (syntaxcheck_points, completion_points) = parse(l);
		part_1_score += syntaxcheck_points;
		if completion_points > 0 {
			part_2_scores.push(completion_points);
		}
	}
	println!("Score for Part 1 is {}", part_1_score);
	part_2_scores.sort();
	println!(
		"Score for Part 2 is {}",
		part_2_scores[part_2_scores.len() / 2]
	);
}

// Returns (x,y), x are the points for syntax checking (p1), so 0 if correct,
// y are the points for autocomplete (Part 2) so 0 if correct
fn parse(line: Vec<char>) -> (u64, u64) {
	let mut stack: Vec<char> = Vec::new();

	for cur in line.iter() {
		match cur {
			'(' | '[' | '{' | '<' => {
				stack.push(*cur);
			}
			')' | ']' | '}' | '>' => {
				if stack.is_empty() {
					return (0, 0);
				} else {
					let last = stack
						.pop()
						.expect("There has to be smthng on the stack");
					if !matching(last, *cur) {
						// Zeile genau so korrupt wie die CDU
						//println!("Cant close a {} with {}", last, cur);
						return (syntax_checker_points(*cur), 0);
					}
				}
			}
			_ => continue,
		}
	}
	// Stack is empty iff line is correct, otherwise there are only opening brackets on the stack
	let mut points = 0;
	while !stack.is_empty() {
		let s = stack.pop().expect("There has to be smthng on the stack");
		points = points * 5 + autocompletion_points(s);
	}
	(0, points)
}

fn autocompletion_points(s: char) -> u64 {
	match s {
		'(' => 1,
		'[' => 2,
		'{' => 3,
		'<' => 4,
		_ => u64::MAX,
	}
}

fn syntax_checker_points(s: char) -> u64 {
	match s {
		')' => 3,
		']' => 57,
		'}' => 1197,
		'>' => 25137,
		_ => u64::MAX,
	}
}

fn matching(a: char, b: char) -> bool {
	(a == '(' && b == ')')
		|| (a == '[' && b == ']')
		|| (a == '{' && b == '}')
		|| (a == '<' && b == '>')
}
