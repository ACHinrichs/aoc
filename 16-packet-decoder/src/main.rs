use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("example2.txt").expect("ERR: file not found!");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap()).collect::<Vec<String>>();

    let mut input_byte: Vec<u8> = lines[0].chars().map(decode_char).collect();
    println!("{}", parse_packet(&input_byte));
}

fn parse_packet(packet: &Vec<u8>) -> u64{
    //First Nibble for version and type-id

    // Version are the 2, 4 and 8 bit of the first nibble
    let version = (packet[0] & 0b1110) >> 1;

    //TypeID are the last bit of the first nibble, and the first two bits of the second nibble:
    let type_id = (packet[0] & 0b0001) << 2 | (packet[1] & 0b1100) >> 2;

    
    match type_id{
	4 => return parse_literal(&packet[1..], 2, 0),
	_ => println!("Hö")
    }
    
    type_id.into()
}

fn parse_literal(packet: &[u8], bit_offset: i8, init_value: u64) -> u64{

    
    // the bit at bit_offset is our "last one" bit
    let mut last_indicator_mask: u8 = 0b00010000 << 3 - bit_offset;

    let mask: u8 = 0b00001111 << 3 - bit_offset;

    let value = ((packet[0] << 4 | packet[1]) & mask) >> (3 - bit_offset);
    let res = (init_value << 4) + (value as u64); 
  
    println!("{:08b} {} {:08b}", value, bit_offset, mask | last_indicator_mask);
    if (packet[0] << 4 | packet[1]) & last_indicator_mask == 0{
	return res;
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
