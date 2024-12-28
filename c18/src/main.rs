use pathfinding::prelude::astar;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const DEPTH: usize = 1024;
const DIM: i16 = 70;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(i16, i16);

impl Pos {
    fn distance(&self) -> i16 {
        2 * DIM - (self.0 + self.1)
    }

    fn successors(&self, blocked: &[Pos]) -> Vec<(Pos, i16)> {
        let &Pos(x, y) = self;
        let surround = [
            (x > 0, Pos(x - 1, y)),
            (x < DIM, Pos(x + 1, y)),
            (y > 0, Pos(x, y - 1)),
            (y < DIM, Pos(x, y + 1)),
        ];
        let succ = surround.iter().filter(|(b, _)| *b).map(|(_, p)| p);
        let succ_set: HashSet<&Pos> = succ.collect();
        let binding = blocked.iter().collect();
        let diff = succ_set.difference(&binding);
        diff.cloned().map(|x| (x.clone(), 1)).collect()
    }

    fn search(&self, blocked: &[Pos]) -> Option<(Vec<Pos>, i16)> {
        astar(
            self,
            |p| p.successors(blocked),
            |p| p.distance(),
            |p| *p == Pos(DIM, DIM),
        )
    }
}

fn collect_blocked(blocked: Vec<String>) -> Vec<Pos> {
    blocked
        .iter()
        .map(|s| {
            let sp: Vec<_> = s.split(',').map(|i| i.parse().unwrap()).collect();
            Pos(sp[0], sp[1])
        })
        .collect()
}

fn task_a(blocked: &Vec<Pos>) -> i16 {
    Pos(0, 0).search(&blocked[..DEPTH]).unwrap().1
}

fn task_b(blocked: &Vec<Pos>) -> &Pos {
    let idx = (0..blocked.len())
        .collect::<Vec<_>>()
        .binary_search_by(|i| match Pos(0, 0).search(&blocked[..*i]) {
            Some(_) => Ordering::Less,
            _ => Ordering::Greater,
        })
        .unwrap_err();
    &blocked[idx - 1]
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let blocked = collect_blocked(lines.collect::<Result<_, _>>().unwrap());
    println!("{}", task_a(&blocked));
    println!("{:?}", task_b(&blocked));
}
