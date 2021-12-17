use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();
    let mut p1_count = 0;
    let mut p2_count = 0;

    for l in &lines {
        let parts = l.split(" | ").collect::<Vec<&str>>();
        let signal_patterns = parts[0].split(" ").collect::<Vec<&str>>();
        let output_values = parts[1].split(" ").collect::<Vec<&str>>();
        assert_eq!(10, signal_patterns.len());
        assert_eq!(4, output_values.len());
        let mut mapping = HashMap::new();
        let mut yet_unknown: Vec<Vec<char>> = Vec::new();

        for pattern in signal_patterns {
            let mut sorted_pattern = pattern.chars().collect::<Vec<char>>();
            sorted_pattern.sort();
            println!("{:?}", pattern);
            if sorted_pattern.len() == 2 {
                // 1 is the only number using 2 segments
                mapping.insert(1, sorted_pattern);
            } else if sorted_pattern.len() == 3 {
                // 7 is the only number using 3 segments
                mapping.insert(7, sorted_pattern);
            } else if sorted_pattern.len() == 4 {
                // 4 is the only number using 4 segments
                mapping.insert(4, sorted_pattern);
            } else if sorted_pattern.len() == 7 {
                // 8 is the only number using all 8 segments
                mapping.insert(8, sorted_pattern);
            } else {
                yet_unknown.push(sorted_pattern);
            }
        }

        while yet_unknown.len() > 0 {
            //Those are always sorted

            /*
            Just to not fuck up, these are the dependencies:
            a -> b,c means a depends on b and c

            1,4,7,8 are always there

            9 -> 4
            6 -> 5
            0 -> 5
            5 -> 9
            2 -> 9
             */
            let sorted_pattern = yet_unknown.remove(0);
            //println!("{:?}", yet_unknown);
            if sorted_pattern.len() == 6 {
                // can be 0, 6 or 9
                // if 4 fits into it completly, it is a 9
                // 4 is always already there
                if sorted_contains(&sorted_pattern, mapping.get(&4).unwrap()) {
                    mapping.insert(9, sorted_pattern);
                    println!("Inserted 9");
                } else if mapping.contains_key(&5) {
                    // if 5 fits into the candidate it is a 6
                    // otherwise it has to be a 0
                    if sorted_contains(&sorted_pattern, mapping.get(&5).unwrap()) {
                        mapping.insert(6, sorted_pattern);
                        println!("Inserted 6");
                    } else {
                        mapping.insert(0, sorted_pattern);
                        println!("Inserted 0");
                    }
                } else {
                    // if 6 is not there yet, we add the pattern again at the end
                    yet_unknown.push(sorted_pattern);
                }
            } else if sorted_pattern.len() == 5 {
                // Can be 2, 3, 5
                // if 1 fits into it, its a 3
                if sorted_contains(&sorted_pattern, mapping.get(&1).unwrap()) {
                    mapping.insert(3, sorted_pattern);
                    println!("Inserted 3");
                } else if mapping.contains_key(&9) {
                    // 5 fits into the 9, but 2 doesnt
                    // same is true for 6, but that would be cyclic
                    // and cyclic bad
                    if sorted_contains(mapping.get(&9).unwrap(), &sorted_pattern) {
                        mapping.insert(5, sorted_pattern);
                        println!("Inserted 5");
                    } else {
                        mapping.insert(2, sorted_pattern);
                        println!("Inserted 2");
                    }
                } else {
                    // if 6 is not there yet, we add the pattern again at the end
                    yet_unknown.push(sorted_pattern);
                }
            } else {
                println!("Wtf");
            }
        }
        let mut scrambled_to_number = HashMap::new();
        for i in 0..=9 {
            println!(
                "{} {:?}",
                i,
                mapping.get(&i).expect("That really should be there")
            );
            scrambled_to_number.insert(mapping.get(&i).expect("That really should be there"), i);
        }

        let mut output_decoded = 0;
        for o in output_values {
            let mut o_chars = o.chars().collect::<Vec<char>>();
            o_chars.sort();
            //let sorted_o:String = o_chars.collect();

            if *scrambled_to_number.get(&o_chars).unwrap() == 1
                || *scrambled_to_number.get(&o_chars).unwrap() == 4
                || *scrambled_to_number.get(&o_chars).unwrap() == 7
                || *scrambled_to_number.get(&o_chars).unwrap() == 8
            {
                p1_count += 1;
            }
            output_decoded = output_decoded * 10 + *scrambled_to_number.get(&o_chars).unwrap();
        }
        p2_count += output_decoded;
    }
    println!("p1_count is {}", p1_count);
    println!("p2_count is {}", p2_count);
}

fn sorted_contains(a: &Vec<char>, b: &Vec<char>) -> bool {
    let mut i_b = 0;
    for i_a in 0..a.len() {
        if i_b >= b.len() {
            return true;
        }
        if a[i_a] == b[i_b] {
            //Halts at the first element in b not in a
            i_b += 1;
        }
    }
    i_b >= b.len() // iff we are at the end of b, all items in b are also in a
}
