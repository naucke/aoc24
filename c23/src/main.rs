use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<String> = lines.collect::<Result<_, _>>().unwrap();
    let mut nodes = HashSet::new();
    let mut bi_edges = HashSet::new();

    for s in &line_vec {
        let split: Vec<_> = s.split('-').collect();
        nodes.extend([split[0], split[1]]);
        bi_edges.extend([(split[0], split[1]), (split[1], split[0])]);
    }

    let a = nodes
        .iter()
        .combinations(3)
        .filter(|t| {
            t.iter().any(|s| s.starts_with('t'))
                && HashSet::from([(*t[0], *t[1]), (*t[1], *t[2]), (*t[0], *t[2])])
                    .is_subset(&bi_edges)
        })
        .count();
    println!("{}", a);

    let mut nets: Vec<_> = nodes.iter().map(|&n| HashSet::from([n])).collect();
    for n in &mut nets {
        for c in &nodes {
            if n.iter().all(|d| bi_edges.contains(&(c, d))) {
                n.insert(c);
            }
        }
    }
    let max = nets.iter().max_by_key(|s| s.len()).unwrap();
    let mut b: Vec<_> = max.iter().collect();
    b.sort();
    println!("{}", b.iter().join(","));
}
