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

    let mut pairs = HashMap::<String,u64>::new();
    for i in 1..polymer_template_string.len(){
	// How often the pair is already contained, failsafe 0 if not
	let count = pairs.get(&polymer_template_string[i-1..=i].to_string())
	    .unwrap_or(&0);
	pairs.insert(polymer_template_string[i-1..=i].to_string(), count + 1);
    }
    println!("{:?}", pairs);


    // Do the polimerisation
    for _i in 0..10{
	let mut new_pairs = HashMap::<String, u64>::new();
    	for (from, to) in rules.iter() {
	    if pairs.contains_key(from){
		let count = *pairs.get(from).unwrap_or(&0);
		println!("{} contained {} times", from, count);
		add_to_value(&mut pairs, &(from[0..=0].to_string() + to), count);
		add_to_value(&mut pairs, &(from[1..=1].to_string() + to), count);
		pairs.insert(from.to_string(), 0);
	    }
	}
	println!("{:?}", pairs);
    }

}

fn add_to_value(map: &mut HashMap::<String,u64>, key: &str, value: u64){
    map.insert(key.to_string(), value + map.get(key).unwrap_or(&0));
}
