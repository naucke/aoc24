use std::fs::File;
use std::io::{self, BufRead};

const BEAT_BY: i16 = 100;

type Pos = (usize, usize);
type Map = Vec<Vec<char>>;
type Dist = Vec<Vec<i16>>;

fn legal<T>(map: &Vec<Vec<T>>, pos: &Pos) -> bool {
    return map.len() > pos.1 && map[0].len() > pos.0;
}

fn get_at<T: Copy>(map: &Vec<Vec<T>>, pos: &Pos) -> T {
    return map[pos.1][pos.0];
}

fn set_at<T>(map: &mut Vec<Vec<T>>, pos: &Pos, data: T) {
    map[pos.1][pos.0] = data;
}

fn no_cheat(map: &Map) -> Dist {
    let mut start = (0, 0);
    'outer: for y in 0..map.len() {
        for x in 0..map[y].len() {
            if get_at(map, &(x, y)) == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }
    let mut out = vec![vec![-1; map[0].len()]; map.len()];
    set_at(&mut out, &start, 0);
    let mut count = 0;
    let mut pred = (0, 0);
    let mut pos = start;
    while get_at(map, &pos) != 'E' {
        let neighb = vec![
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ];
        let new = neighb
            .iter()
            .filter(|p| {
                let c = get_at(map, p);
                return **p != pred && (c == '.' || c == 'E');
            })
            .next()
            .unwrap();
        pred = pos;
        pos = *new;
        set_at(&mut out, &pos, count);
        count += 1;
    }
    return out;
}

fn cheat(map: &Dist) -> usize {
    let mut out = 0;
    for y in 1..(map.len() - 1) {
        for x in 1..(map[y].len() - 1) {
            if get_at(map, &(x, y)) != -1 {
                continue;
            }
            let left_pred = get_at(map, &(x - 1, y));
            let up_pred = get_at(map, &(x, y - 1));
            let mut jump = Vec::new();
            if left_pred >= 0 {
                let mut jump_right = vec![(x + 1, y + 1), (x + 1, y - 1), (x + 2, y)];
                if get_at(map, &(x, y + 1)) >= 0 {
                    jump_right.push((x, y + 2));
                }
                if get_at(map, &(x, y - 1)) >= 0 {
                    jump_right.push((x, y - 2));
                }
                jump.append(&mut (jump_right.iter().map(|&p| (left_pred, p)).collect()));
            }
            if up_pred >= 0 {
                let mut jump_down = vec![(x + 1, y + 1), (x - 1, y + 1), (x, y + 2)];
                if get_at(map, &(x + 1, y)) >= 0 {
                    jump_down.push((x + 2, y));
                }
                if get_at(map, &(x - 1, y)) >= 0 {
                    jump_down.push((x - 2, y));
                }
                jump.append(&mut (jump_down.iter().map(|&p| (up_pred, p)).collect()));
            }
            out += jump
                .iter()
                .filter(|(_, p)| legal(map, p) && get_at(map, p) >= 0)
                .map(|(pred, pos)| (pred - get_at(map, pos)).abs())
                .filter(|&a| a >= BEAT_BY)
                .count();
        }
    }
    return out;
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let map: Vec<Vec<_>> = line_vec.iter().map(|s| s.chars().collect()).collect();

    let dist = no_cheat(&map);
    let cheat = cheat(&dist);
    println!("{}", cheat);
}
