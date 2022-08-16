use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let file = File::open("input.txt").expect("ERR: file not found!");
	let lines = BufReader::new(file)
		.lines()
		.map(|x| x.unwrap())
		.collect::<Vec<String>>();
	for l in lines {
		let mut input_bits: Vec<u8> = Vec::new();
		for c in l.chars() {
			let val = u8::from_str_radix(&c.to_string(), 16).unwrap();
			for i in 0..4 {
				input_bits.push(1 & val >> (3 - i));
			}
		}
		println!("Result for {} is {}", l, parse_packet(&input_bits).0);
	}
}

fn parse_packet(packet: &[u8]) -> (u64, &[u8]) {
	// Version are the 2, 4 and 8 bit of the first nibble
	let _version = packet[0..3].iter().fold(0, |a, b| b | a << 1);
	let type_id = packet[3..6].iter().fold(0, |a, b| b | a << 1);
	let res = match type_id {
		4 => parse_literal(&packet[6..], 0),
		_ => parse_operator(&packet[6..], type_id),
	};
	(res.0 as u64, res.1)
}

fn parse_operator(packet: &[u8], type_id: u8) -> (u64, &[u8]) {
	let operation = |r: u64, x: &u64| match type_id {
		0 => r + *x,
		1 => r * *x,
		2 => std::cmp::min(*x, r),
		3 => std::cmp::max(*x, r),
		5 => {
			if r == u64::MAX {
				*x
			} else {
				if r > *x {
					1
				} else {
					0
				}
			}
		}
		6 => {
			if r == u64::MAX {
				*x
			} else {
				if r < *x {
					println!("{}", *x);
					1
				} else {
					0
				}
			}
		}
		7 => {
			if r == u64::MAX {
				*x
			} else {
				if r == *x {
					1
				} else {
					0
				}
			}
		}
		_ => u64::MAX,
	};
	let init_value = match type_id {
		0 => 0,
		1 => 1,
		2 | 5 | 6 | 7 => u64::MAX,
		3 => u64::MIN,
		_ => 42,
	};

	if packet[0] == 0 {
		// Type 0, next 15 bits specify how long the subpackets that are contained arelet mut bits_used = 0;
		let len_packets = packet[1..16].iter().fold(0, |a, b| b | a << 1);
		let mut res = init_value;
		let mut new_packets = &packet[16..];
		let start_len = new_packets.len();
		while new_packets.len() > start_len - len_packets as usize {
			let res_vec = parse_packet(new_packets);
			new_packets = res_vec.1;
			res = operation(res, &res_vec.0);
		}
		return (res, new_packets);
	} else {
		// Type 1, next 11 bits specify how many subpackets there are
		let num_packets = packet[1..12].iter().fold(0, |a, b| b | a << 1);
		let mut res = init_value;
		let mut new_packets = &packet[12..];
		for _i in 0..num_packets {
			let res_vec = parse_packet(new_packets);
			new_packets = res_vec.1;
			res = operation(res, &res_vec.0);
		}
		return (res, new_packets);
	}
}

fn parse_literal(packet: &[u8], init_value: u64) -> (u64, &[u8]) {
	let last_group = packet[0] == 0;

	let value = packet[1..5].iter().fold(0, |a, b| b | a << 1);
	let res = (init_value << 4) + value as u64;
	if last_group {
		//println!("Value of Package: {}", res);
		return (res, &packet[5..]);
	}
	return parse_literal(&packet[5..], res);
}
