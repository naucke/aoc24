use std::fs::File;
use std::io::{self, BufRead};

const BEAT_BY: usize = 100;
const JUMP: usize = 2;

type Pos = (usize, usize);
type Map = Vec<Vec<char>>;
type Dist = Vec<Pos>;

fn no_cheat(map: &Map) -> Dist {
    let (mut x, mut y) = (0, 0);
    map.iter().enumerate().for_each(|(j, row)| {
        if let Some(i) = row.iter().position(|&c| c == 'S') {
            (x, y) = (i, j);
        }
    });
    let mut out = vec![(0, 0), (x, y)];
    // TODO unfold?
    while map[y][x] != 'E' {
        let neighb = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
        let ni = neighb.iter();
        let mut next = ni.filter(|&p| *p != out[out.len() - 2] && map[p.1][p.0] != '#');
        (x, y) = *(next.next().unwrap());
        out.push((x, y));
    }
    out.remove(0);
    return out;
}

fn cheat(track: &Dist) -> usize {
    let pos_dists = track.iter().enumerate().map(|(i, &(x, y))| {
        (i + JUMP..track.len()).into_iter().map(move |j| {
            let dist = (x as i64 - track[j].0 as i64).abs() + (y as i64 - track[j].1 as i64).abs();
            ((i, j), dist as usize)
        })
    });
    let far_enough = pos_dists.flatten().filter(|&(_, d)| d <= JUMP);
    let dists = far_enough.map(|((i, j), d)| j - i - d);
    return dists.filter(|&a| a >= BEAT_BY).count();
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec.iter().map(|s| s.chars().collect()).collect();

    let dist = no_cheat(&map);
    let cheat = cheat(&dist);
    println!("{}", cheat);
}
