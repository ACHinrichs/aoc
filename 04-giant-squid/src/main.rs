use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn read_file(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>) {
	let file = File::open(filename).expect("ERR: file not found!");
	let mut lines = BufReader::new(file).lines().map(|x| x.unwrap());

	//Parse first line (The drawn numbers)
	let res_numbers: Vec<i32> = lines
		.next()
		.expect("ERR: Numbers expected here")
		.split(',')
		.map(|s| {
			s.parse::<i32>()
				.expect("ERR: Encountered something that is not a Number")
		})
		.collect::<Vec<i32>>();

	//Parse boards
	let mut res_boards: Vec<Vec<Vec<i32>>> = Vec::new();
	for l in lines {
		if l == "" {
			//Between the boards we generate the new ones
			res_boards.push(Vec::new());
		} else {
			//Parse one line of a board
			res_boards.last_mut().expect("ERR Board invalid").push(
				l.split(' ')
					.filter(|x| !x.is_empty())
					.map(|s| {
						s.parse::<i32>().expect(
							"ERR: Encountered something that is not a Number",
						)
					})
					.collect::<Vec<i32>>(),
			);
		}
	}

	(res_numbers, res_boards)
}

fn main() {
	let (numbers, boards) = read_file("input.txt");
	println!("Got {} boards and {} numbers", boards.len(), numbers.len());

	part2(numbers, boards);
}

fn part2(numbers: Vec<i32>, mut boards: Vec<Vec<Vec<i32>>>) {
	let mut finished_order: Vec<(Vec<Vec<i32>>, i32)> = Vec::new();
	let mut number_index = 0;

	while number_index < numbers.len() && boards.len() > 0 {
		mark_number(&mut boards, numbers[number_index]);

		let mut new_boards: Vec<Vec<Vec<i32>>> = Vec::new();
		for i in 0..boards.len() {
			if board_finished(&boards[i]) {
				finished_order
					.push((copy_board(&boards, i), numbers[number_index]));
			} else {
				new_boards.push(copy_board(&boards, i));
			}
		}
		boards = new_boards;

		number_index += 1;
	}
	let winning_board = &finished_order[0].0;
	println!("Winning board is {:?}", winning_board);
	// Collect all positive numbers of winning board
	let winning_sum = collect_positive(winning_board);
	println!("Winning-Sum is {:?}", winning_sum);
	println!(
		"Winning-Sum * last number is {}",
		winning_sum * finished_order[0].1
	);

	let loosing_sum =
		collect_positive(&finished_order[finished_order.len() - 1].0);
	println!("Loosing-Sum is {:?}", loosing_sum);
	println!(
		"Loosing-Sum * last number is {}",
		loosing_sum * finished_order[finished_order.len() - 1].1
	);
}

fn collect_positive(board: &Vec<Vec<i32>>) -> i32 {
	(*board)
		.iter()
		.map(|y| y.iter().fold(0, |b, x| b + cmp::max(x, &0)))
		.fold(0, |b, x| b + cmp::max(x, 0))
}

fn copy_board(boards: &Vec<Vec<Vec<i32>>>, i: usize) -> Vec<Vec<i32>> {
	boards[i]
		.iter()
		.map(|x| x.iter().map(|x| *x).collect::<Vec<i32>>())
		.collect::<Vec<Vec<i32>>>()
}

fn mark_number(boards: &mut Vec<Vec<Vec<i32>>>, number: i32) {
	for i in 0..boards.len() {
		for j in 0..boards[i].len() {
			for k in 0..boards[i][j].len() {
				if boards[i][j][k] == number {
					boards[i][j][k] = -number; //Negative number indicates already hit
				}
			}
		}
	}
}

fn board_finished(board: &Vec<Vec<i32>>) -> bool {
	let rows = board
		.iter()
		.map(|x| x.iter().fold(-1, |x, y| x & *y))
		.fold(false, |b, x| (b || 0 > x));
	let columns = board
		.iter()
		.fold(
			(0..board[0].len()).map(|_x| true).collect::<Vec<bool>>(),
			|b, x| {
				b.iter()
					.zip(x.iter())
					.map(|(x, y)| *x && y < &0)
					.collect::<Vec<bool>>()
			},
		)
		.iter()
		.fold(false, |b, x| b || *x);
	// (|b, x| b.iter().zip(x.iter()).map(|(u,v)| u & v).collect::<Vec<i32>>().to_vec());
	//println!("{} {:?}", rows, columns);
	rows || columns
}
