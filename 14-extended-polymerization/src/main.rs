use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn main() {
    let file = File::open("example.txt").expect("file not found");
    let lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string());

    let polymer_template_string: String = lines.nth(0).unwrap();
    assert_eq!(lines.nth(0).unwrap(),"");
    let mut rules = HashMap::new();
    for l in lines{
	let mut split = l.split(" -> ").map(|x| x.to_string());
	rules.insert(split.nth(0).unwrap(), split.nth(0).unwrap());
    }
    println!("{:?}", rules);

    let mut pairs: Vec<(String, u64)> = Vec::new();
    for i in 1..polymer_template_string.len(){
	// How often the pair is already contained, failsafe 0 if not
	let mut contained = false;
	let pair = polymer_template_string[i-1..=i].to_string();
	for i in 0..pairs.len(){
	    if pairs[i].0 == pair {
		contained = true;
		pairs[i].1 += 1;
	    }
	}
	if !contained {
	    pairs.push((pair, 1));
	}
    }
    println!("{:?}", pairs);


    // Do the polimerisation
    for _i in 0..10{
	let mut new_pairs = Vec::new();
    	for (p, count) in pairs.iter() {
	    if rules.contains_key(p) {
		let rule_res = rules.get(p).unwrap();
		add_to_value(&mut new_pairs,
			     &(p[0..=0].to_string() + &rule_res.to_string()),
			     *count);
		add_to_value(&mut new_pairs,
			     &(rule_res.to_string() + &p[1..=1].to_string()),
			     *count);
		//new_pairs.push((from.to_string(), 0));
	    } else {
		println!("Rule {} not found!", p);
	    }
	}
	pairs = new_pairs;
	println!("{:?}", pairs);
    }
    // Count elements
    let mut counts: Vec<(String,u64)> = Vec::new();
    for (k,v) in pairs{
	add_to_value(&mut counts, &k[0..=0].to_string(), v);
	add_to_value(&mut counts, &k[1..=1].to_string(), v);
    }
    counts.sort();
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
