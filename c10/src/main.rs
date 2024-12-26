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
    let a = heads.iter().map(|r| r.0.clone()).flatten().collect();
    let b = heads.iter().map(|r| r.1).sum();
    (a, b)
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec
        .iter()
        .map(|s| s.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();
    let clos = |(a1, b1), (a2, b2)| (a1 + a2, b1 + b2);
    let res = (0..map[0].len()).map(|x| {
        let row = (0..map.len()).map({
            // TODO without closure this clone is unnecessary
            let m = map.clone();
            move |y| {
                let heads = trailheads(&m, &(x, y), 0);
                (heads.0.len(), heads.1)
            }
        });
        row.reduce(clos).unwrap()
    });
    println!("{:?}", res.reduce(clos).unwrap());
}
