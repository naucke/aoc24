use std::fs::File;
use std::io::{self, BufRead};

fn match_count(design: &str, towels: &Vec<&str>) -> u64 {
    let mut cache = vec![0; design.len() + 1];
    cache[0] = 1;
    for i in 1..=design.len() {
        towels
            .iter()
            .filter(|t| i >= t.len() && design[..i].ends_with(*t))
            .for_each(|t| cache[i] += cache[i - t.len()])
    }
    cache[design.len()]
}

fn main() {
    let file = io::BufReader::new(File::open("input").unwrap());
    let lines: Vec<_> = file.lines().collect::<Result<_, _>>().unwrap();
    let towels: Vec<_> = lines[0].split(", ").collect();
    let designs = &lines[2..];
    let counts = designs
        .iter()
        .map(|s| match_count(s, &towels))
        .filter(|c| *c > 0);
    println!("{}", counts.clone().count());
    println!("{}", counts.sum::<u64>());
}
