use std::fs::read_to_string;
use itertools::Itertools;

fn line_to_pair_of_ints(line: &str) -> (i32, i32) {
    line
        .split("   ")
        .map(|number| number.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap()
}

fn main() {
    let input = read_to_string("inputs/2024/01/input.txt").unwrap();
    let (mut left, mut right) : (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(line_to_pair_of_ints)
        .unzip();

    left.sort();
    right.sort();

    let solution_1: i32 = std::iter::zip(left.iter(), right.iter())
        .map(|(left_value, right_value)| (left_value - right_value).abs())
        .sum();

    let mut frequencies = std::collections::HashMap::new();
    for value in right {
        *frequencies.entry(value).or_insert(0i32) += 1;
    }

    let solution_2: i32 = left
        .iter()
        .map(|value| value * frequencies.get(value).copied().unwrap_or(0i32))
        .sum();

    println!("Solution 1: {solution_1}");
    println!("Solution 2: {solution_2}")
}