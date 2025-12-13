use std::fs::read_to_string;

fn parse_bank(line: &str) -> Vec<u8>
{
    line.chars().map(|char| char.to_digit(10).unwrap() as u8).collect::<Vec<_>>()
}

fn get_largest_joltage(bank: &[u8], digits: usize) -> u64
{
    let mut digits_left = digits;
    let mut lower_bound = 0; // From this index we can start searching new digits
    let mut upper_bound;
    let mut result = 0;
    while digits_left > 0
    {
        upper_bound = bank.len() - (digits_left - 1); // From this index, we can't search further or we won't have enough digits.

        // max_by_key finds the largest and latest in the sequence, so reverse it to get the earliest instead.
        let (max_digit_index, max_digit_available) = bank[lower_bound..upper_bound].iter().enumerate().rev().max_by_key(|(_index, item)| *item).unwrap();
        result += *max_digit_available as u64 * 10_u64.pow(digits_left as u32 - 1);
        lower_bound += max_digit_index + 1;
        
        digits_left -= 1;
    }

    return result;
}

fn main()
{
    let banks = read_to_string("inputs/2025/03/input.txt").unwrap()
        .lines()
        .map(parse_bank)
        .collect::<Vec<_>>();

    let solution1: u64 = banks.iter().map(|bank| get_largest_joltage(&bank, 2)).sum();
    let solution2: u64 = banks.iter().map(|bank| get_largest_joltage(&bank, 12)).sum();
    println!("Solution 1: {solution1}");
    println!("Solution 2: {solution2}");
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn get_largest_joltage_adjacent_beginning()
    {
        assert_eq!(get_largest_joltage(&vec![9, 8, 1], 2), 98);
    }

    #[test]
    fn get_largest_joltage_same_digit()
    {
        assert_eq!(get_largest_joltage(&vec![9, 9, 1], 2), 99);
    }

    #[test]
    fn get_largest_joltage_extremes()
    {
        assert_eq!(get_largest_joltage(&vec![9, 1, 8], 2), 98);
    }

    #[test]
    fn get_largest_joltage_sparse()
    {
        assert_eq!(get_largest_joltage(&vec![1, 9, 1, 8, 1], 2), 98);
    }

    #[test]
    fn get_larget_joltage_twelve_digits()
    {
        assert_eq!(get_largest_joltage(&vec![8,1,8,1,8,1,9,1,1,1,1,2,1,1,1], 12), 888911112111);
    }
}