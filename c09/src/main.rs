use itertools::Itertools;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;

fn chk(fs: &Vec<i32>) -> usize {
    let chk = fs.iter().enumerate().filter(|(_, &b)| b >= 0);
    chk.map(|(i, &b)| i * (b as usize)).sum()
}

fn main() {
    let lines = io::BufReader::new(File::open("input").unwrap()).lines();
    let line_vec: Vec<_> = lines.collect::<Result<_, _>>().unwrap();
    let input_c = line_vec[0].chars();
    let input: Vec<_> = input_c.map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut fs_a = Vec::new();
    let mut free_c = VecDeque::new();
    let mut id = 0;
    for (&taken, &free) in input.iter().tuples() {
        fs_a.extend(iter::repeat(id).take(taken));
        let len = fs_a.len();
        free_c.extend(len..len + free);
        fs_a.extend(iter::repeat(-1).take(free));
        id += 1;
    }
    if input.len() % 2 == 1 {
        fs_a.extend(iter::repeat(id).take(input[input.len() - 1]));
    }

    let mut right = fs_a.len() - 1;
    while let Some(free) = free_c.pop_front() {
        while fs_a[right] < 0 {
            right -= 1;
        }
        if free >= right {
            break;
        }
        fs_a[free] = fs_a[right];
        fs_a[right] = -1;
    }

    println!("{}", chk(&fs_a));
}
