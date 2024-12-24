use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();

    let mut nodes = HashSet::new();
    let mut bi_edges = HashSet::new();

    for s in line_vec {
        let split: Vec<_> = s.split('-').map(|s| s.to_string()).collect();
        nodes.insert(split[0].clone());
        nodes.insert(split[1].clone());
        bi_edges.insert((split[0].clone(), split[1].clone()));
        bi_edges.insert((split[1].clone(), split[0].clone()));
    }

    let a = nodes
        .iter()
        .combinations(3)
        .filter(|t| {
            t.iter().any(|s| s.starts_with('t'))
                && HashSet::from([
                    (t[0].clone(), t[1].clone()),
                    (t[1].clone(), t[2].clone()),
                    (t[0].clone(), t[2].clone()),
                ])
                .is_subset(&bi_edges)
        })
        .count();
    println!("{}", a);

    let mut nets: Vec<_> = nodes.iter().map(|n| HashSet::from([n.clone()])).collect();
    for n in &mut nets {
        for c in &nodes {
            if n.iter().all(|d| bi_edges.contains(&(c.clone(), d.clone()))) {
                n.insert(c.to_string());
            }
        }
    }
    let max = nets.iter().max_by_key(|s| s.len()).unwrap();
    let mut b: Vec<_> = max.iter().map(|s| s.clone()).collect();
    b.sort();
    println!("{}", b.join(","));
}
