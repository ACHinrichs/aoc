use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Debug)]
struct Vent{
    start: Vec<u64>,
    end: Vec<u64>  
}

impl Vent{
    fn straight_intersections(&self, other: &Vent) -> Vec<Vec<u64>>{
	if !self.is_straight() || ! other.is_straight(){
	    //println!("{:?} or {:?} is not straight", self, other);
	    return Vec::new()
	}
	let mut intersections = Vec::new();
	
	for x in self.start[0]..self.end[0]+1{
	    for y in self.start[1]..self.end[1]+1{
		if x >= other.start[0] && x <= other.end[0] && y >= other.start[1] && y <= other.end[1]{
		    intersections.push(vec![x, y]);
		}
	    }
	}
	intersections
    }

    fn parse(s: &str) -> Self {
	let split: Vec<String> = s.split(" -> ").map(|x| x.to_string()).collect();
	let p1 = split[0].split(",").map(|x| x.parse::<u64>().unwrap_or(0)).collect::<Vec<u64>>();
	let p2 =split[1].split(",").map(|x| x.parse::<u64>().unwrap_or(0)).collect::<Vec<u64>>();

	if p2[0] <= p1[0] && p2[1] <= p1[1]{
	    return Vent {start:p2, end:p1}
	}
	Vent {start:p1, end:p2}
    }

    fn is_straight(&self) -> bool{
	self.start[0] == self.end[0] || self.start[1] == self.end[1]
    }
}

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let lines = BufReader::new(file).lines().map(|x| x.unwrap());
    let mut vents: Vec<Vent> = Vec::new();
    
    for l in lines{
	vents.push(Vent::parse(&l));	
    }

    let mut intersections = Vec::new();
    for (i,a) in vents.iter().enumerate(){
	for b in vents.iter().skip(i+1){
	    println!("{:?} {:?}",a,b);
	    if a.start == b.start && a.end == b.end{
		println!("You fucked up mate")
	    }
	    intersections.append(&mut a.straight_intersections(b));
	}
    }
    intersections.sort();
    intersections.dedup();
    for i in &intersections{
	println!("{:?}", i);
    }

    println!("Number of straight Intersections is {}", &intersections.len())
}
