use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("example1.txt").expect("ERR: file not found!");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut input_byte: Vec<u8> = lines[0].chars().map(decode_char).collect();
    println!("{:?}", parse_packet(&input_byte, 0));
}

fn parse_packet(packet: &[u8], bit_offset: u8) -> (u64, &[u8], u8){
    //First Nibble for version and type-id
    let first_byte = (packet[0] << 4 | packet[1] ) << bit_offset;
    println!("Parsing packet with first byte {:08b}, offset {}", first_byte, bit_offset);
    for p in packet {
	print!(" {:04b}", p);
    }
    println!("");
    // Version are the 2, 4 and 8 bit of the first nibble
    let version = ((first_byte) & 0b11100000) >> 1;

    //TypeID are the last bit of the first nibble, and the first two bits of the second nibble:
    let type_id = (first_byte & 0b00011100) >> 2;
    println!("Type_ID {}", type_id);
    
    let new_offset = (bit_offset + 2).rem_euclid(4);
    println!("New Offset {}", new_offset);
    let mut new_packet_start = 1;
    if new_offset == 0{
	new_packet_start = 2;
    }
    match type_id{
	4 => return parse_literal(&packet[new_packet_start..], new_offset, 0),
	_ => return parse_operator(&packet[new_packet_start..], new_offset)
    }
}

fn parse_operator(packet: &[u8], bit_offset: u8) -> (u64, &[u8], u8) {
    // length_type_id is the 2^1 bit of the second nibble
    if packet[0] & (1 << (bit_offset + 3)%4) != 0 {
	// Type 1, next 11 bits specify how many subpackets there are
	let num_packets = parse_next_n_bits(packet, bit_offset, 11);
	println!("a{} {:04b} {:04b}", num_packets, packet[0], (1 << ((bit_offset+3)%4)));
	return (0, &[], 0)
    } else {
	// Type 0, next 15 bits specify how long the subpackets that are contained arelet mut bits_used = 0;
	let len_packets = parse_next_n_bits(packet, (bit_offset + 3)%4, 15);
	println!("b{} {:04b} {:04b}", len_packets, packet[0], (1 << (bit_offset+3)%4));
	let used_bits = 0;
	let mut res = 0;
	let mut new_packets;
	if bit_offset > 2{
	    new_packets = &packet[3..];
	} else {
	    new_packets = &packet[4..];
	}
	let mut new_offset = (bit_offset);
	while used_bits < len_packets{
	    println!("{}", new_offset);
	    let res_vec = parse_packet(new_packets, new_offset);
	    new_packets = res_vec.1;
	    new_offset = (res_vec.2 + 3).rem_euclid(4) ;
	    res = res_vec.0;
	}
	return (res, new_packets, new_offset);
    }
}

fn parse_next_n_bits(packet: &[u8], bit_offset: u8, field_len: u8) -> u64{
    let mut res: u64 = 0;
    let mut i = 0;
    let mut bits_used = 0;
    while bits_used < field_len {
	//print!("\n{:04b}", packet[i]);
	if bits_used == 0 && bit_offset > 0{
	    res = (packet[i] & (0b111 >> (4-bit_offset))) as u64;
	    //print!(" {:04b}", 0b111 >> (4-bit_offset));
	    bits_used = bit_offset;
	} else if bits_used + 4 < field_len{
	    res = (res << 4) + packet[i] as u64;
	    bits_used += 4;
	} else {
	    let number_bits_in_last = (4 - (field_len - bits_used));
	    let mask = 0b1111 & (0b1111 << number_bits_in_last);
	    //println!("\n{:015b}", res);
	    res = (res << number_bits_in_last) | ((packet[i] & mask) >> number_bits_in_last) as u64;
	    //print!(" {:04b} {} {} {}", mask, bit_offset, bits_used, number_bits_in_last);
	    bits_used += 4;
	}
	i += 1;
    }
    //println!("\n{:015b}", res);
    return res
}

fn parse_literal(packet: &[u8], bit_offset: u8, init_value: u64) -> (u64, &[u8], u8){

    
    let mut last_indicator_mask: u8 = 0b00010000;

    let mask: u8 = 0b00001111;

    let first_byte = (packet[0] << 4 | packet[1] ) >> 3 - bit_offset;
    println!("");
    println!("parse literal: {:08b} (was shifted by {})", first_byte, bit_offset);
    println!("Mask for ind.: {:08b}", last_indicator_mask);
    println!("Mask for num.: {:08b}", mask);
    
    
    let value = (first_byte & mask);
    let res = (init_value << 4) + (value as u64); 
  
    println!("{:08b} {} {:08b}", value, bit_offset, mask & last_indicator_mask);
    if (first_byte & last_indicator_mask) == 0{
	println!("Apparently we are done with the literal number, result is {}", res);
	if bit_offset == 3 {
	    return (res, &packet[2..], bit_offset);
	} else {
	    return (res, &packet[1..], bit_offset);
	}
    }
    if bit_offset == 3{
	return parse_literal(&packet[2..], (bit_offset + 1).rem_euclid(4), res);
    } else { 
	return parse_literal(&packet[1..], (bit_offset + 1).rem_euclid(4), res)
    }
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
