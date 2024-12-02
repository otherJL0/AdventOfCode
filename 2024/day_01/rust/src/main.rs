use std::collections::HashMap;
use std::fs;
use std::vec::Vec;

fn read_input() -> Vec<String> {
    fs::read_to_string("../input")
        .expect("Failed to read input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

fn parse_numbers(pairs: Vec<String>) -> (Vec<u32>, Vec<u32>) {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = pairs
        .into_iter()
        .map(|pair| {
            let mut nums = pair.split_whitespace();
            let left = nums
                .next()
                .expect("Expected left number")
                .parse::<u32>()
                .expect("Unable to parse left number");
            let right = nums
                .next()
                .expect("Expected right number")
                .parse::<u32>()
                .expect("Unable to parse right number");
            (left, right)
        })
        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn part_one(left: Vec<u32>, right: Vec<u32>) -> u32 {
    left.into_iter()
        .zip(right)
        .fold(0, |acc, pair| acc + pair.0.abs_diff(pair.1))
}

fn part_two(left: Vec<u32>, right: Vec<u32>) -> u32 {
    let counter: HashMap<u32, u32> =
        right
            .into_iter()
            .fold(HashMap::with_capacity(left.len()), |mut counter, num| {
                *counter.entry(num).or_insert(0) += 1;
                counter
            });
    left.into_iter()
        .fold(0, |acc, num| acc + num * counter.get(&num).unwrap_or(&0))
}

fn main() {
    let (left, right) = parse_numbers(read_input());
    let result_1 = part_one(left.clone(), right.clone());
    println!("Part One Result: {result_1}");
    let result_2 = part_two(left, right);
    println!("Part One Result: {result_2}");
}
