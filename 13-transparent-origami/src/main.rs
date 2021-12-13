use regex::Match;
use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let file = File::open("input.txt").expect("file not found");
    let mut lines = &mut BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().to_string())
    //.collect::<Vec<String>>()
	;

    let mut dots: Vec<(i64, i64)> = lines
        .take_while(|x| *x != "".to_string())
        .map(|x| x.split(",").map(|x| x.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
        .iter()
        .map(|x| (x[0].parse::<i64>().unwrap(), x[1].parse::<i64>().unwrap()))
        .collect();

    let x_fold_re = Regex::new(r"(x=)\d+").unwrap();
    let y_fold_re = Regex::new(r"(y=)\d+").unwrap();
    let mut folds: Vec<(i64, i64)> = Vec::new();
    for l in lines {
        let x_match = x_fold_re.find(&l);
        if x_match.is_some() {
            folds.push((x_match.unwrap().as_str()[2..].parse::<i64>().unwrap(), 0))
        }
        let y_match = y_fold_re.find(&l);
        if y_match.is_some() {
            folds.push((0, y_match.unwrap().as_str()[2..].parse::<i64>().unwrap()))
        }
    }

    println!("{:?}", dots);
    println!("{:?}", folds);

    dots = fold_paper(dots, folds[0]);

    dots.sort();
    dots.dedup();
    println!("{:?} {}", dots, dots.len());
}

fn fold_paper(dots: Vec<(i64, i64)>, fold_along: (i64, i64)) -> Vec<(i64, i64)> {
    let mut res: Vec<(i64, i64)> = Vec::new();
    for dot in dots {
        if dot.0 > fold_along.0 && fold_along.0 > 0 {
            res.push((fold_along.0 - (dot.0 - fold_along.0), dot.1));
        } else if dot.1 > fold_along.1 && fold_along.1 > 1 {
            res.push((dot.0, fold_along.1 - (dot.1 - fold_along.1)));
        } else {
            res.push((dot.0, dot.1));
        }
    }
    res
}
