use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("ERR: file not found!");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    for l in lines{
	let mut input_bits: Vec<u8> = Vec::new();
	for c in l.chars(){
	    let val = decode_char(c);
	    for i in 0..4{
		input_bits.push(1 & decode_char(c) >> (3 - i));
	    }
	}
	println!("Result for {} is {}", l, parse_packet(&input_bits).0);
    }
}

fn parse_packet(packet: &[u8]) -> (u64, &[u8]){
    // Version are the 2, 4 and 8 bit of the first nibble
    let version = packet[0..3].iter().fold(0, |a, b| b | a << 1);
    //TypeID are the last bit of the first nibble, and the first two bits of the second nibble:
    let type_id = packet[3..6].iter().fold(0, |a, b| b | a << 1);
    let res = match type_id{
	4 => parse_literal(&packet[6..], 0),
	_ => parse_operator(&packet[6..])
    };
    (res.0 + version as u64, res.1)
}

fn parse_operator(packet: &[u8]) -> (u64, &[u8]) {
    if packet[0] == 0{
	// Type 0, next 15 bits specify how long the subpackets that are contained arelet mut bits_used = 0;
	let len_packets = packet[1..16].iter().fold(0, |a, b| b | a << 1);
	let mut res = 0;
	let mut new_packets = &packet[16..];
	let start_len = new_packets.len();
	while new_packets.len() > start_len - len_packets as usize {
	    let res_vec = parse_packet(new_packets);
	    new_packets = res_vec.1;
	    res += res_vec.0;
	}
	return (res, new_packets);
    } else {
	// Type 1, next 11 bits specify how many subpackets there are
	let num_packets = packet[1..12].iter().fold(0, |a, b| b | a << 1);
	let mut res = 0;
	let mut new_packets = &packet[12..];
	for _i in 0..num_packets{
	    let res_vec = parse_packet(new_packets);
	    new_packets = res_vec.1;
	    res += res_vec.0;
	}
	return (res, new_packets);
    } 
}


fn parse_literal(packet: &[u8], init_value: u64) -> (u64, &[u8]){
    let last_group = packet[0] == 0;
    
    let value = packet[1..5].iter().fold(0, |a, b| b | a << 1);
    let res = (init_value << 4) + value as u64; 
    if last_group{
	return (0, &packet[5..]);
    }
    return parse_literal(&packet[5..], res);
}

fn decode_char(c: char) -> u8{
    match c{
	'0' => 0,
	'1' => 1,
	'2' => 2,
	'3' => 3,
	'4' => 4,
	'5' => 5,
	'6' => 6,
	'7' => 7,
	'8' => 8,
	'9' => 9,
	'A' => 10,
	'B' => 11,
	'C' => 12,
	'D' => 13,
	'E' => 14,
	'F' => 15,
	_ => 0b11111111
    }
}
