use std::fs::read_to_string;

use itertools::Itertools;

struct Range
{
    begin: u64,
    end: u64
}

fn to_range(text: &str) -> Range
{
    let (begin, end) = text.split("-").map(&str::parse::<u64>).collect_tuple().unwrap();
    return Range { begin: begin.unwrap(), end: end.unwrap() };
}

fn is_valid(text: &str) -> bool
{
    let mid_point = text.len() / 2;
    let left = &text[..mid_point];
    let right = &text[mid_point..];
    return left != right;
}

fn is_valid_any_length(text: &str) -> bool
{
    for substr_length in 1..text.len()
    {
        if text.len() % substr_length != 0
        {
            continue;
        }

        let mut is_invalid = true;
        let (mut current, mut next) = text.split_at(substr_length);
        while is_invalid && !next.is_empty()
        {
            is_invalid = current == &next[..substr_length];
            (current, next) = next.split_at(substr_length);
        }

        if is_invalid
        {
            return false;
        }
    }    

    return true;
}

fn sum_invalid_indices<'a, I, V>(ranges: I, validator: V) -> u64
where 
I: IntoIterator<Item = &'a Range>,
V: Fn(&str) -> bool
{
    let mut sum = 0;
    for index in ranges.into_iter().map(|range| range.begin..range.end).flatten()
    {
        let index_as_string = index.to_string();
        if !validator(&index_as_string)
        {
            sum += index;
        }
    }
    return sum;
}

fn main()
{
    let ranges = read_to_string("inputs/2025/02/input.txt").unwrap()
        .trim_end()
        .split(",")
        .map(to_range)
        .collect::<Vec<_>>();

    let solution1 = sum_invalid_indices(&ranges, &is_valid);
    let solution2 = sum_invalid_indices(&ranges, &is_valid_any_length);
    println!("Solution 1: {solution1}");
    println!("Solution 2: {solution2}");
}

#[cfg(test)]
mod test
{
    use crate::*;

    #[test]
    fn is_invalid_two_characters()
    {
        assert_eq!(is_valid("22"), false);
    }

    #[test]
    fn is_valid_no_repeated_characters()
    {
        assert_eq!(is_valid("1234"), true);
    }

    #[test]
    fn is_invalid_multiple_characters()
    {
        assert_eq!(is_valid("123123"), false);
    }

    #[test]
    fn is_valid_odd_characters()
    {
        assert_eq!(is_valid("1231234"), true);
    }

    #[test]
    fn is_valid_single_character()
    {
        assert_eq!(is_valid("1"), true);
    }
}