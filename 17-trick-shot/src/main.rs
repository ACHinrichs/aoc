use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
	.collect::<Vec<String>>();

    //Attention, the (x=) has to be removed later!
    let x_area_regex = Regex::new(r"(x=)-?\d+..-?\d+").unwrap();
    let y_area_regex = Regex::new(r"(y=)-?\d+..-?\d+").unwrap();
    let x_area_string = x_area_regex.find(&lines[0]).unwrap().as_str();
    let y_area_string = y_area_regex.find(&lines[0]).unwrap().as_str();

    let x_area = x_area_string[2..].split("..").map(|x| i64::from_str_radix(&x.to_string(),
									    10)
						    .unwrap()).collect::<Vec<i64>>();
    let y_area = y_area_string[2..].split("..").map(|x| i64::from_str_radix(&x.to_string(),
									    10)
						    .unwrap()).collect::<Vec<i64>>();
    println!("{:?}", y_area);
    let mut initial_y_vel = y_area[0]-1;
    let mut y_max = 0;
    let mut winning_initial = (0,0);
    let mut vel_counts = 0;
    'y_loop: loop{
	let mut initial_x_vel = 0; 
	'x_loop: loop{
	    let mut x_pos = 0;
	    let mut y_pos = 0;
	    let mut x_vel = initial_x_vel;
	    let mut y_vel = initial_y_vel;
	    let mut this_y_max = 0;
	    'steps_loop: loop {
		x_pos = x_pos + x_vel;
		y_pos = y_pos + y_vel;
		if this_y_max < y_pos{
		    this_y_max = y_pos;
		}
		x_vel = std::cmp::max(0, x_vel - 1);
		y_vel = y_vel - 1;
		if x_area[0] <= x_pos && x_pos <= x_area[1] &&
		    y_area[0] <= y_pos && y_pos <= y_area[1]
		{
		    if y_max < this_y_max{
			y_max = this_y_max;
			winning_initial = (initial_x_vel, initial_y_vel);
			println!("Current candidate is {} {}, with a y-max of {}",
				 initial_x_vel,
				 initial_y_vel,
				 this_y_max);
		    } else {
			println!("Found {} {}",
				 initial_x_vel,
				 initial_y_vel);
		    }
		    vel_counts += 1;
		    break 'steps_loop;
		} 
		if x_pos > x_area[1] || y_pos < y_area[0]{
		    break 'steps_loop;
		}
	    }	    
	    
	    initial_x_vel = initial_x_vel + 1;
	    if initial_x_vel > x_area[1]{
		break 'x_loop;
	    }
	}
	initial_y_vel = initial_y_vel + 1;
	if -initial_y_vel < y_area[0]{
	    // I think we should overshoot in this case, but dunno lol
	    break 'y_loop;
	}
    }
    println!("\nFound Result: Max-Y = {} with initial vel. ({},{}).\nThere were {} candidates",
	     y_max,
	     winning_initial.0,
	     winning_initial.1,
	     vel_counts);
}
