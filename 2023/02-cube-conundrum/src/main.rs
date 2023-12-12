use std::cmp;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct DiceSet {
    red: u64,
    green: u64,
    blue: u64,
}

struct Game {
    game_id: u64,
    dice_sets: Vec<DiceSet>,
}

impl Game {
    fn build_from_input(line: String) -> Self {
        let id = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap(); //not very nice, but will do the trick

        let mut result = Game {
            game_id: id,
            dice_sets: Vec::new(),
        };
        let game_sets: Vec<String> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split("; ")
            .map(|x| x.to_string())
            .collect();

        for set in game_sets {
            let mut cur_set = DiceSet {
                red: 0,
                blue: 0,
                green: 0,
            };
            for s in set.split(", ") {
                let num: u64 = s.split(" ").nth(0).unwrap().parse().unwrap();
                let color = s.split(" ").nth(1).unwrap();
                if color == "green" {
                    cur_set.green = num
                } else if color == "blue" {
                    cur_set.blue = num
                } else if color == "red" {
                    cur_set.red = num
                } else {
                    panic!("Encountered an unexpected color {}", color);
                }
            }
            result.dice_sets.push(cur_set);
        }
        return result;
    }

    fn get_minimum_set(&self) -> DiceSet {
        let mut result = DiceSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in &self.dice_sets {
            result.red = cmp::max(result.red, set.red);
            result.green = cmp::max(result.green, set.green);
            result.blue = cmp::max(result.blue, set.blue);
        }
        return result;
    }
}
fn main() {
    println!("AOC 2023, Day 01 – Trebuchet\n");
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Please supply the task and the problem-input to solve as arguments!");
    }
    let task = &args[1];

    let file = File::open(&args[2]).expect(&format!("Inputfile {} not found", args[1]));
    let games = &mut BufReader::new(file)
        .lines()
        .map(|x| Game::build_from_input(x.unwrap().to_string()))
        .collect::<Vec<Game>>();

    if task == "1" {
        let res: Vec<u64> = games
            .iter()
            .filter(|g| {
                g.dice_sets
                    .iter()
                    .all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14)
            })
            .map(|g| g.game_id)
            .collect();
        println!("Valid games are: {:?}", res);
        println!("The Sum of the IDs of Games that can be played wth 12 red, 13 green and 14 blue dice is {}", res.iter().sum::<u64>());
    } else if task == "2" {
        let res: u64 = games
            .iter()
            .map(|g| g.get_minimum_set())
            .map(|s| s.red * s.green * s.blue)
            .sum();
        println!("The Sum of the powers of the minimum sets is {}", res);
    } else {
        panic!("Task unknown, please specify as first argument")
    }
}
