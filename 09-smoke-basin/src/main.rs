use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let ground = BufReader::new(file)
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();

    let flow_matrix = flow_matrix(&ground);
    print_flow_matrix(&ground, &flow_matrix);
    let mut basin_counts:Vec<i64> = Vec::new();  
    for i in 0..flow_matrix.len() {
        for j in 0..flow_matrix[i].len() {
	    if flow_matrix[i][j] == (0,0) && ground[i][j] != 9 {
		println!("basin around {},{}",i,j);
		basin_counts.push(1 + explore_basin((i,j), &flow_matrix));
	    }
	}
    }
    basin_counts.sort();
    println!("Basin Counts: {:?}", basin_counts);
    println!("Low Level Sum (Part 1) is {}", low_level_sum(&ground));
}

fn low_level_sum(ground: &Vec<Vec<u32>>) -> u32 {
    let mut low_level_sum: u32 = 0;
    for i in 0..ground.len() {
        for j in 0..ground[i].len() {
            if (i <= 0 || ground[i][j] < ground[i - 1][j])
                && (i >= ground.len() - 1 || ground[i][j] < ground[i + 1][j])
                && (j <= 0 || ground[i][j] < ground[i][j - 1])
                && (j >= ground[i].len() - 1 || ground[i][j] < ground[i][j + 1])
            {
                low_level_sum += ground[i][j] + 1;
            }
        }
    }
    low_level_sum
}

fn flow_matrix(ground: &Vec<Vec<u32>>) -> Vec<Vec<(i64, i64)>> {
    let mut res: Vec<Vec<(i64,i64)>> = Vec::new();
    for i in 0..ground.len() {
	let mut row: Vec<(i64,i64)> = Vec::new();
        for j in 0..ground[i].len() {
            if ground[i][j] == 9 {
		row.push((0,0));
	    } else {
                //9 is never in a basin
                let mut flow: (i64, i64) = (0, 0);
                let dir_array: [(i64, i64); 4] = [(0, 1),
						  (0, -1),
						  (1, 0),
						  (-1, 0)];
                for direction in dir_array {
                    if 0 <= (i as i64) + direction.0
                        && (i as i64) + direction.0 < ground.len() as i64
                        && 0 <= (j as i64) + direction.1
                        && (j as i64) + direction.1 < ground[i].len() as i64
                        && ground[(i as i64 + direction.0) as usize][
			    (j as i64 + direction.1) as usize]
			< ground[(i as i64 + flow.0) as usize][
			    (j as i64 + flow.1) as usize]
			
                    {
                        flow = direction;
                    }
                }
		row.push(flow);
            }
        }
	res.push(row);
    }
    res
}

fn explore_basin(coord: (usize,usize), flow_matrix: &Vec<Vec<(i64,i64)>>) -> i64{
    let mut res = 0;
    let mut to_explore: Vec<(usize,usize)> = Vec::new();
    to_explore.push(coord);
    
    while !to_explore.is_empty(){
	let cur = to_explore.pop().expect("HOW THE FUCK DOES THIS FAIL?");
        let dir_array: [(i64, i64); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];
        for dir in dir_array {
            if 0 <= (cur.0 as i64) + dir.0
                && (cur.0 as i64) + dir.0 < flow_matrix.len() as i64
                && 0 <= (cur.1 as i64) + dir.1
                && (cur.1 as i64) + dir.1 < flow_matrix[cur.0].len() as i64
		&& flow_matrix[(cur.0 as i64 + dir.0) as usize][(cur.1 as i64 +dir.1) as usize] == (-dir.0, -dir.1)
	    {
		println!("{:?} + {:?}", cur, dir);
                to_explore.push(((cur.0 as i64 + dir.0) as usize,
				 ((cur.1 as i64 + dir.1) as usize)));
		res += 1;
	    }
	}
    }
    res
}

fn print_flow_matrix(ground: &Vec<Vec<u32>>, flow_matrix: &Vec<Vec<(i64,i64)>>){
    for i in 0..flow_matrix.len(){
	for j in 0..flow_matrix[i].len(){
	    print!("{}", ground[i][j]);
	    if (flow_matrix[i][j] == (1,0)){
		print!("↓ ");
	    } else if(flow_matrix[i][j] == (-1,0)){
		print!("↑ ");
	    } else if(flow_matrix[i][j] == (0,1)){
		print!("→ ");
	    } else if(flow_matrix[i][j] == (0,-1)){
		print!("← ");
	    } else if(flow_matrix[i][j] == (0,0)){
		print!("· ");
	    } 
	}
	print!("\n");
    }
}
