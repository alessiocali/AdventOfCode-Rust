use std::fs::read_to_string;

fn modulus(a: i32, b: i32) -> i32
{
    (a % b + b) % b
}

fn parse_line(line: &str) -> i32
{
    let (direction, value) = line.split_at(1);
    let value = value.trim().parse::<i32>().unwrap();
    if direction == "R" { value } else { -value }
}

fn zero_counts<'a, I>(values: I) -> i32
where I: Iterator<Item = &'a i32>
{
    let mut count: i32 = 0;
    let mut dial: i32 = 50;
    for value in values
    {
        dial = (dial + value) % 100;
        if dial == 0
        {
            count += 1;
        }
    }
    count
}

fn zero_counts_two<'a, I>(values: I) -> i32
where I: Iterator<Item = &'a i32>
{
    let mut count: i32 = 0;
    let mut dial: i32 = 50;
    for value in values.into_iter()
    {
        let loops_this_time = (value / 100).abs();
        let actual_rotation = value % 100;
        let unclamped_rotation = dial + actual_rotation;
        // If there's a smarter way to represent this mathematically and not logically, I don't know it.
        let overflow: i32 = (dial != 0 && (unclamped_rotation <= 0 || unclamped_rotation >= 100)) as i32;

        dial = modulus(unclamped_rotation, 100);
        count += loops_this_time + overflow;
    }
    count
}

fn main()
{
    let input = read_to_string("inputs/2025/01/input.txt").unwrap();
    let values = input.lines().map(parse_line).collect::<Vec<_>>();
    let solution1 = zero_counts(values.iter());
    let solution2 = zero_counts_two(values.iter());
    println!("Solution 1: {solution1}");
    println!("Solution 2: {solution2}");
}


#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn parse_left()
    {
        assert_eq!(parse_line("R15"), 15);
    }

    #[test]
    fn parse_right()
    {
        assert_eq!(parse_line("L21"), -21);
    }

    #[test]
    fn parse_with_spaces()
    {
        assert_eq!(parse_line("R3 "), 3);
    }

    #[test]
    fn zero_count_one_zero()
    {
        assert_eq!(zero_counts([-50].iter()), 1);
    }

    #[test]
    fn zero_count_multiple_zeroes()
    {
        assert_eq!(zero_counts([-50, 60, -60].iter()), 2);
    }

    #[test]
    fn zero_count_loop_right()
    {
        assert_eq!(zero_counts([49, 1].iter()), 1);
    }

    #[test]
    fn zero_count_loop_left()
    {
        assert_eq!(zero_counts([-49, -1].iter()), 1);
    }

    #[test]
    fn zero_count_two_over_once()
    {
        assert_eq!(zero_counts_two([100].iter()), 1);
    }

    #[test]
    fn zero_count_two_under_once()
    {
        assert_eq!(zero_counts_two([-100].iter()), 1);
    }

    #[test]
    fn zero_count_two_over_multiple()
    {
        assert_eq!(zero_counts_two([1000].iter()), 10);
    }

    #[test]
    fn zero_count_two_under_multiple()
    {
        assert_eq!(zero_counts_two([-1000].iter()), 10);
    }

    #[test]
    fn zero_count_two_exactly_one()
    {
        assert_eq!(zero_counts_two([-50].iter()), 1);
    }

    #[test]
    fn zero_count_two_under_multiple_split()
    {
        assert_eq!(zero_counts_two([-50, -100].iter()), 2);
    }

    #[test]
    fn zero_count_two_zig_zag()
    {
        assert_eq!(zero_counts_two([-100, -100, 200].iter()), 4)
    }
    
    #[test]
    fn zero_count_two_sample_test()
    {
        assert_eq!(zero_counts_two([-68, -30, 48, -5, 60, -55, -1, -99, 14, -82].iter()), 6);
    }
}
