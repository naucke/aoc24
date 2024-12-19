use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut lines = io::BufReader::new(File::open("input").unwrap())
        .lines()
        .into_iter();
    let binding = lines.next().unwrap().unwrap();
    let towels = format!("^({})*$", binding.replace(", ", "|"));
    let re = Regex::new(&towels).unwrap();

    lines.next();
    let patterns: Vec<_> = lines.map(|s| s.unwrap()).collect();

    let matches = patterns
        .iter()
        .map(|s| re.captures(s))
        .filter(|m| m.is_some())
        .count();
    println!("{}", matches);
}
