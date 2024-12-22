use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const MAX_CHEATS: u8 = 2;
const BEAT_BY: usize = 100;

type Pos = (usize, usize);

struct Map {
    map: Vec<Vec<char>>,
    // how many steps without cheating
    no_cheat: usize,
    // redundant but frequently used
    width: usize,
    height: usize,
}

impl Map {
    fn at(&self, pos: &Pos) -> char {
        return self.map[pos.1][pos.0];
    }

    fn search(&self, pos: Pos, visited: HashSet<Pos>, cheats: u8) -> (u64, usize) {
        if visited.len() > self.no_cheat - BEAT_BY || cheats > MAX_CHEATS {
            return (0, usize::MAX);
        }
        if self.at(&pos) == 'E' {
            return (1, visited.len());
        }
        let mut our_visited = visited.clone();
        our_visited.insert(pos);
        let neighb = vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ];
        let recurse = neighb
            .iter()
            .filter(|p @ (x, y)| {
                !visited.contains(p)
                    && *x > 0
                    && *x < self.width - 1
                    && *y > 0
                    && *y < self.height - 1
            })
            .map(|p| {
                self.search(
                    *p,
                    our_visited.clone(),
                    cheats
                        + match self.at(p) {
                            '#' => 1,
                            _ => 0,
                        },
                )
            });
        return recurse
            .reduce(|(acc_paths, acc_best_length), (paths, best_length)| {
                (acc_paths + paths, min(acc_best_length, best_length))
            })
            // the solution is too generic to be efficient, but also this unwrap is wrong
            .unwrap();
    }
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map_vec: Vec<Vec<_>> = line_vec.iter().map(|s| s.chars().collect()).collect();
    let width = map_vec[0].len();
    let height = map_vec.len();

    // inefficient start position determination
    let start_y = map_vec.iter().position(|x| x.contains(&'S')).unwrap();
    let start_x = map_vec[start_y].iter().position(|x| *x == 'S').unwrap();
    let start = (start_x, start_y);

    let mut map = Map {
        map: map_vec,
        no_cheat: usize::MAX,
        width,
        height,
    };
    map.no_cheat = map.search(start, HashSet::new(), MAX_CHEATS).1;
    println!("{}", map.no_cheat);
    println!("{}", map.search(start, HashSet::new(), 0).0);
}
