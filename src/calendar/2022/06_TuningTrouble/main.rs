use std::{ 
    fs::File, 
    io::{BufRead, BufReader}
};

#[derive(Debug)]
enum Error { EmptyFile, MarkerNotFound, IoError(std::io::Error) }

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

fn read_input_file(file_path: &str) -> Result<String, Error> {
    let file = File::open(file_path)?;
    match BufReader::new(file).lines().next() {
        Some(result) => result.map_err(Error::from),
        None => Err(Error::EmptyFile),
    }
}

// Trivial cheesy implementation. If we wanted to not be cheesy we could use
// a HashSet or even better a size-27 bitset, but who cares :^)
fn are_chars_unique_trivial(window: &str) -> bool {
    let mut iter = window.chars();
    let mut unique = true;

    while let (Some(current), true) = (iter.next(), unique) {
        unique &= iter.clone().all(|next| next != current);
    }

    unique
}

fn find_marker_index(input_string: &String, window_size: usize) -> Result<usize, Error> {
    let (mut min, mut max) = (0, window_size);
    
    while max < input_string.len() {
        let window = &input_string[min..max];
        if are_chars_unique_trivial(window) {
            return Ok(max);
        }

        min += 1;
        max += 1;
    }

    Err(Error::MarkerNotFound)
}

fn solve_problem(file_path: &str) -> Result<(usize, usize), Error> { 
    let input = read_input_file(file_path)?;
    let marker_size_4 = find_marker_index(&input, 4)?;
    let marker_size_14 = find_marker_index(&input, 14)?;
    Ok((marker_size_4, marker_size_14))
}

fn main() {
    match solve_problem("inputs/2022/06/TuningTrouble.txt") {
        Ok((solution1, solution2)) => {
            println!("Marker at size 4: {solution1}");
            println!("Marker at size 14: {solution2}");
        },
        Err(err) => println!("{err:?}")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unique_chars() {
        assert!(are_chars_unique_trivial("abcd"));
        assert!(!are_chars_unique_trivial("aabb"));
        assert!(!are_chars_unique_trivial("abbc"));
        assert!(!are_chars_unique_trivial("abcc"));
    }
}