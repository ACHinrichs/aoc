use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string());

    let mut elements: String = lines.nth(0).unwrap().to_string();
    assert_eq!(lines.nth(0).unwrap(),"");
    let mut rules = HashMap::new();
    for l in lines{
	let mut split = l.split(" -> ").map(|x| x.to_string());
	rules.insert(split.nth(0).unwrap(), split.nth(0).unwrap());
    }
    println!("{:?}", rules);


    // Do the polimerisation
    for _i in 0..10{
	let mut new_elements = elements[0..=0].to_string();
	// Starting at 1 is correct, we are looking behind!
    	for i in 1..elements.len() {
	    let p = &elements[i-1..=i];
	    if rules.contains_key(p) {
		let rule_res = rules.get(p).unwrap();
		new_elements = new_elements.to_owned() +
		    //&elements[i-1..=i-1] +
		    rule_res +
		    &elements[i..=i];
	    } else {
		println!("Rule {} not found!", p);
	    }
	}
	elements = new_elements.to_string();
	println!("{}", elements);
    }
    // Count elements
    let mut counts: Vec<(String,u64)> = Vec::new();
    for i in 0..elements.len(){
	add_to_value(&mut counts, &elements[i..=i].to_string(), 1);
    }
    counts.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for (k,v) in counts{
	println!("{} contained {} times",k,v);
    }
}

fn add_to_value(vec: &mut Vec::<(String,u64)>, key: &str, value: u64){
    let mut contained = false;
    for i in 0..vec.len(){
	if vec[i].0 == key {
	    contained = true;
	    vec[i].1 += value;
	}
    }
    if !contained {
	vec.push((key.to_string(), value));
    }
}
