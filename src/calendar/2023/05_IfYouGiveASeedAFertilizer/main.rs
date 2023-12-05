struct Range {
    source: u32,
    destination: u32,
    length: u32
}

enum AlmanacData {
    InitSet { label: String, numbers: Vec<u32> },
    Mapping { label: String, ranges: Vec<Range> }
}

fn parse_input_line(line: &str) -> Option<AlmanacData> {
    Some(AlmanacData::InitSet { label: "".to_string(), numbers: vec![] })
}

fn main() {
    let lines_result = advent_of_code::read_file("inputs/2023/05/input.txt").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_init_set() {
        let result = parse_input_line("something: 1 2 3").unwrap();
        assert!(matches!(result, AlmanacData::InitSet { label: ("something".to_string()), numbers: vec![] }))
    }
}