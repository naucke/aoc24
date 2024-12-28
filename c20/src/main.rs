use std::fs::File;
use std::io::{self, BufRead};

const BEAT_BY: usize = 100;

type Pos = (usize, usize);
type Map = Vec<Vec<char>>;
type Dist = Vec<Pos>;

fn no_cheat(map: &Map) -> Dist {
    let (mut x, mut y) = (0, 0);
    for (j, row) in map.iter().enumerate() {
        if let Some(i) = row.iter().position(|&c| c == 'S') {
            (x, y) = (i, j);
        }
    }
    let mut out = vec![(0, 0), (x, y)];
    while map[y][x] != 'E' {
        let neighb = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        let ni = neighb.iter();
        let mut next = ni.filter(|&p| *p != out[out.len() - 2] && map[p.1][p.0] != '#');
        (x, y) = *(next.next().unwrap());
        out.push((x, y));
    }
    out.remove(0);
    out
}

fn cheat(track: &Dist, jump: usize) -> usize {
    let pos_dists = track.iter().enumerate().map(|(i, &(x, y))| {
        (i + jump..track.len())
            .into_iter()
            .map(move |j| ((i, j), x.abs_diff(track[j].0) + y.abs_diff(track[j].1)))
    });
    let allowed = pos_dists.flatten().filter(|&(_, d)| d <= jump);
    let dists = allowed.map(|((i, j), d)| j - i - d);
    dists.filter(|&a| a >= BEAT_BY).count()
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec.iter().map(|s| s.chars().collect()).collect();

    let dist = no_cheat(&map);
    println!("{}", cheat(&dist, 2));
    println!("{}", cheat(&dist, 20));
}
