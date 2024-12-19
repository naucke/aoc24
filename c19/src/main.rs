use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn match_count(pattern: String, towels: &Vec<&str>) -> u32 {
    let mut res = 0;
    let mut matches = vec![String::from("")];
    while let Some(candidate) = matches.pop() {
        if candidate == pattern {
            res += 1;
            continue;
        }
        let mut sth: Vec<_> = towels
            .iter()
            .map(|t| candidate.clone() + t)
            .filter(|p| pattern.starts_with(p))
            .collect();
        matches.append(&mut sth);
    }
    return res;
}

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

    // TODO if (b) works can refactor to not use regex
    let other_towels: Vec<_> = binding.split(", ").collect();
    let res: u32 = patterns.iter().map(|s| match_count(s.to_string(), &other_towels)).sum();
    println!("{}", res);
}
