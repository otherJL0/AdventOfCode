use std::fs;

fn read_input() -> Vec<String> {
    fs::read_to_string("../input")
        .expect("Failed to read input")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect()
}

fn parse_numbers(report: Vec<String>) -> Vec<Vec<i32>> {
    report
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Unable to parse number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

fn is_valid(line: Vec<i32>) -> bool {
    let deltas = line.windows(2).all(|w| w[0].abs_diff(w[1]) <= 3);
    let strictly_increasing_or_decreasiing =
        line.is_sorted_by(|a, b| a < b) || line.is_sorted_by(|a, b| a > b);
    deltas && strictly_increasing_or_decreasiing
}

fn part_one(report: Vec<Vec<i32>>) -> usize {
    report
        .into_iter()
        .map(|line| usize::from(is_valid(line)))
        .sum()
}

/// TODO: Find more efficient method
fn part_two(report: Vec<Vec<i32>>) -> usize {
    report
        .into_iter()
        .map(|line| {
            for i in 0..line.len() {
                let mut candidate = line.clone();
                candidate.remove(i);
                if is_valid(candidate) {
                    return 1;
                }
            }
            0
        })
        .sum()
}

fn main() {
    let part_one_result = part_one(parse_numbers(read_input()));
    println!("{part_one_result}");
    let part_two_result = part_two(parse_numbers(read_input()));
    println!("{part_two_result}");
}
