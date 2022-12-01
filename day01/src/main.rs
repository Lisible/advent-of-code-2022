#![warn(clippy::pedantic)]
use std::{fs::File, io::prelude::*, io::BufReader};

fn main() {
    let file = File::open("input").expect("Cannot open input file");
    let reader = BufReader::new(file);

    let mut sums = vec![];
    let mut current_sum = 0;

    for line in reader.lines() {
        let line = line.expect("Line read is not valid UTF-8");
        if line.is_empty() {
            sums.push(current_sum);
            current_sum = 0;
            continue;
        }

        current_sum += line
            .parse::<i32>()
            .expect("Line read doesn't contain a number");
    }

    sums.sort_unstable();

    println!("Part1: {}", sums.last().unwrap());
    println!("Part2: {}", &sums[sums.len() - 3..].iter().sum::<i32>());
}
