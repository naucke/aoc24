use pathfinding::prelude::astar;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

const DEPTH: usize = 1024;
const DIM: i16 = 70;

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos(i16, i16);

impl Pos {
    fn distance(&self) -> i16 {
        let &Pos(x, y) = self;
        return 2 * DIM - (x + y);
    }

    fn successors(&self, blocked: &[Pos]) -> Vec<(Pos, i16)> {
        let &Pos(x, y) = self;
        let succ: HashSet<_> = [
            (x > 0, Pos(x - 1, y)),
            (x < DIM, Pos(x + 1, y)),
            (y > 0, Pos(x, y - 1)),
            (y < DIM, Pos(x, y + 1)),
        ]
        .iter()
        .filter(|(b, _)| *b)
        .map(|(_, p)| p)
        .cloned()
        .collect();
        return succ
            .difference(&(blocked.into_iter().cloned().collect()))
            .cloned()
            .map(|x| (x, 1))
            .collect();
    }
}

fn collect_blocked(blocked: Vec<String>) -> Vec<Pos> {
    return blocked
        .iter()
        .map(|s| {
            let sp: Vec<_> = s.split(',').map(|i| i.parse().unwrap()).collect();
            return Pos(sp[0], sp[1]);
        })
        .collect();
}

fn task_a(blocked: &Vec<Pos>) -> i16 {
    return astar(
        &Pos(0, 0),
        |p| p.successors(&blocked[..=DEPTH]),
        |p| p.distance(),
        |p| *p == Pos(DIM, DIM),
    )
    .unwrap()
    .1;
}

fn main() {
    let blocked = collect_blocked(
        io::BufReader::new(File::open("input").unwrap())
            .lines()
            .into_iter()
            .map(|s| s.unwrap())
            .collect(),
    );
    println!("{}", task_a(&blocked));
}
