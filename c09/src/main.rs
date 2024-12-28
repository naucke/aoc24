use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let input: Vec<_> = line_vec[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut fs = Vec::new();
    let mut free_blk = VecDeque::new();
    let mut id = 0;
    for (&taken, &free) in input.iter().tuples() {
        fs.extend(iter::repeat(id).take(taken));
        let len = fs.len();
        free_blk.extend(len..len + free);
        fs.extend(iter::repeat(-1).take(free));
        id += 1;
    }
    if input.len() % 2 == 1 {
        fs.extend(iter::repeat(id).take(input[input.len() - 1]));
    }

    let mut right = fs.len() - 1;
    while let Some(free) = free_blk.pop_front() {
        while fs[right] < 0 {
            right -= 1;
        }
        if free >= right {
            break;
        }
        fs[free] = fs[right];
        fs[right] = -1;
    }

    let chk = fs.iter().take(right + 1).enumerate();
    let a: usize = chk.map(|(i, &b)| i * (b as usize)).sum();
    println!("{}", a);
}
