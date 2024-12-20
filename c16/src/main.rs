use pathfinding::prelude::astar;
use std::fs::File;
use std::io::{self, BufRead};

type Map = Vec<Vec<char>>;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    N,
    W,
    S,
    E,
}

fn turn(d: &Direction) -> Vec<Direction> {
    match d {
        Direction::N => vec![Direction::W, Direction::E],
        Direction::W => vec![Direction::S, Direction::N],
        Direction::S => vec![Direction::E, Direction::W],
        Direction::E => vec![Direction::N, Direction::S],
    }
}

fn step(p: &Pos) -> Pos {
    let mut p = p.clone();
    match p.d {
        Direction::N => p.y += 1,
        Direction::W => p.x -= 1,
        Direction::S => p.y -= 1,
        Direction::E => p.x += 1,
    }
    return p;
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Pos {
    x: usize,
    y: usize,
    d: Direction,
}

impl Pos {
    fn distance(&self, map: &Map) -> usize {
        return (map[0].len() - 2 - self.x) +
               (self.y - 1);
    }

    fn successors(&self, map: &Map) -> Vec<(Pos, usize)> {
        let mut res: Vec<_> = turn(&self.d)
            .iter()
            .map(|d| {
                let mut p = self.clone();
                p.d = d.clone();
                return (p, 1000);
            })
            .collect();
        let step = step(&self);
        let step_char = map[step.y][step.x];
        if step_char == '.' || step_char == 'E' {
            res.push((step, 1));
        }
        return res;
    }
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec.iter().map(|s| s.chars().collect()).collect();

    let res = astar(
        &Pos { x: 1, y: map.len() - 2, d: Direction::E },
        |p| p.successors(&map),
        |p| p.distance(&map),
        |p| p.x == map[0].len() - 2 && p.y == 1,
    );
    println!("{}", res.unwrap().1);
}
