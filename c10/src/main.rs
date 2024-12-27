use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

type Pos = (usize, usize);
type Map = Vec<Vec<u8>>;

fn trailheads(map: &Map, &(x, y): &Pos, start: u8) -> (HashSet<Pos>, u16) {
    if x >= map[0].len() || y >= map.len() || map[y][x] != start {
        return (HashSet::new(), 0);
    }
    if start == 9 {
        return (HashSet::from([(x, y)]), 1);
    }
    let neighb = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
    let heads: Vec<_> = neighb
        .iter()
        .map(|p| trailheads(map, p, start + 1))
        .collect();
    let mut a = HashSet::new();
    let mut b = 0;
    for h in heads {
        a.extend(h.0);
        b += h.1;
    }
    (a, b)
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec
        .iter()
        .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    let mut res = Vec::new();
    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let (a, b) = trailheads(&map, &(x, y), 0);
            res.push((a.len(), b));
        }
    }
    let res_iter = res.iter().copied();
    let res_red = res_iter.reduce(|(a1, b1), (a2, b2)| (a1 + a2, b1 + b2));
    println!("{:?}", res_red.unwrap());
}
