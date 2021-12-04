use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp;

fn read_file(filename: &str) -> (Vec<i32>, Vec<Vec<Vec<i32>>>){
    let file = File::open(filename).expect("ERR: file not found!");
    let mut lines = BufReader::new(file).lines().map(|x| x.unwrap());

    //Parse first line (The drawn numbers)
    let res_numbers:Vec<i32> = lines.next().expect("ERR: Numbers expected here")
	.split(',')
	.map(|s| s.parse::<i32>().expect("ERR: Encountered something that is not a Number"))
	.collect::<Vec<i32>>();
    
    //Parse boards
    let mut res_boards:Vec<Vec<Vec<i32>>> = Vec::new();
    for l in lines{
	if l == ""{
	    //Between the boards we generate the new ones
	    res_boards.push(Vec::new());
	} else {
	    //Parse one line of a board
	    res_boards.last_mut().expect("ERR Board invalid").push(
		l.split(' ')
		    .filter(|x| !x.is_empty())
		    .map(|s| s.parse::<i32>().expect("ERR: Encountered something that is not a Number"))
		    .collect::<Vec<i32>>()
	    );
	}
    }

    (res_numbers, res_boards)
}

fn main() {
    let (numbers, mut boards) = read_file("input.txt");
    println!("Got {} boards and {} numbers", boards.len(), numbers.len());

    let mut finished = false;
    let mut winning_board = boards.len();
    let mut number_index = 0;
    
    while !finished && number_index < numbers.len()  {
	mark_number(&mut boards, numbers[number_index]);

	let res = check_for_winning_board(&boards);
	if res.is_ok(){
	    winning_board = res.unwrap();
	    finished = true;
	} else {
	    number_index += 1;
	}
    }

    println!("Winning board has index {:?}", winning_board);

    // Collect all positive numbers of winning board

    let sum = boards[winning_board].iter().map(|y| y.iter().fold(0, |b, x| b + cmp::max(*x,0))).fold(0, |b, x| b + cmp::max(x,0));
    println!("Sum is {}", sum);
    println!("Sum * last number is {}", sum * numbers[number_index]);
}

fn mark_number(boards: &mut Vec<Vec<Vec<i32>>>, number: i32){
    for i in 0..boards.len(){
	for j in 0..boards[i].len(){
	    for k in 0..boards[i][j].len(){
		if boards[i][j][k] == number{
		    boards[i][j][k] = -number; //Negative number indicates already hit
		}
	    }
	}
    }
}
    
fn check_for_winning_board(boards: &Vec<Vec<Vec<i32>>>) -> Result<usize, bool>{
    for i in 0..boards.len(){
	if board_finished(&boards[i]){
	    return Ok(i);
	}
    }
    Err(false)
}

fn board_finished(board: &Vec<Vec<i32>>) -> bool{
    let rows    = board.iter().map(|x| x.iter().fold(-1, |x,y| x & *y)).fold(false, |b , x| (b || 0 > x));
    let columns = board.iter().fold((0..board[0].len()).map(|_x| true).collect::<Vec<bool>>(),
				    |b, x| b.iter().zip(x.iter()).map(|(x,y)| *x && y < &0).collect::<Vec<bool>>())
	.iter().fold(false, |b,x| b || *x);
	// (|b, x| b.iter().zip(x.iter()).map(|(u,v)| u & v).collect::<Vec<i32>>().to_vec());
    //println!("{} {:?}", rows, columns);
    rows || columns
}
