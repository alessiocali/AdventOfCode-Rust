use std::collections::{ HashMap, HashSet };
use std::fs::File;
use std::io::{ BufReader, BufRead };
use regex::Regex;

#[derive(thiserror::Error, Clone, Debug)]
enum Error {
    #[error("Error parsing line: {0}.\nLine was: {1}")]
    ParsingError(String, String)
}

#[derive(Clone, Copy)]
struct AlmanacRange {
    start: u64,
    length: u64
}

struct AlmanacRangeMapping {
    from_start: u64,
    to_start: u64,
    length: u64
}

struct AlmanacMap {
    to: String,
    range_mappings: Vec<AlmanacRangeMapping>
}

#[derive(Default)]
struct Almanac {
    seeds: HashSet<u64>,
    seeds_as_ranges: Vec<AlmanacRange>,
    maps_by_source: HashMap<String, AlmanacMap>
}

fn parse_input<T: AsRef<str>>(lines: impl Iterator<Item = T>) -> Result<Almanac, Error> {
    lazy_static::lazy_static! {
        static ref SEEDS_REGEX: Regex = Regex::new(r"^seeds:(.*)$").unwrap();
        static ref MAP_REGEX: Regex = Regex::new(r"^(?<from>\w+)\-to\-(?<to>\w+) map:$").unwrap();
        static ref MAP_RANGE_REGEX: Regex = Regex::new(r"^(?<to_start>\d+) (?<from_start>\d+) (?<length>\d+)$").unwrap();
    }
    
    let mut result = Almanac::default();
    let mut current_map_from: Option<String> = None;
    
    for line in lines {
        if let Some(capture) = SEEDS_REGEX.captures(line.as_ref()) {
            let seeds_string = capture.get(0).unwrap().as_str();
            let seed_numbers: Vec<_> = seeds_string
                .split(" ")
                .filter_map(|number_string| number_string.parse::<u64>().ok())
                .collect();

            result.seeds_as_ranges = seed_numbers
                .windows(2)
                .step_by(2)
                .map(|window| AlmanacRange { start: window[0], length: window[1] })
                .collect();

            result.seeds = seed_numbers.into_iter().collect();
        }
        else if let Some(capture) = MAP_REGEX.captures(&line.as_ref()) {
            let from = capture.name("from").unwrap().as_str().to_string();
            let map_key = from.clone();
            current_map_from = Some(map_key.clone());

            let to = capture.name("to").unwrap().as_str().to_string();
            let new_map = AlmanacMap { to, range_mappings: vec![] };
            result.maps_by_source.insert(map_key, new_map);
        }
        else if let Some(capture) = MAP_RANGE_REGEX.captures(line.as_ref()) {
            let current_map_from = current_map_from.as_ref().ok_or(Error::ParsingError("Found range without map.".to_string(), line.as_ref().to_string()))?;
            let current_map = result.maps_by_source.get_mut(current_map_from).ok_or(Error::ParsingError(format!("Found range but map {current_map_from} was not found."), line.as_ref().to_string()))?;

            let from_start = capture.name("from_start").unwrap().as_str().parse::<u64>().unwrap();
            let to_start = capture.name("to_start").unwrap().as_str().parse::<u64>().unwrap();
            let length = capture.name("length").unwrap().as_str().parse::<u64>().unwrap();
            current_map.range_mappings.push(AlmanacRangeMapping { from_start, to_start, length });
        }
    }

    Ok(result)
}

fn apply_map_to_elements(source_elements: impl Iterator<Item = u64>, map: &AlmanacMap) -> HashSet<u64> {
    let mut result = HashSet::<u64>::new();

    for element in source_elements {
        let matching_range = map.range_mappings.iter().find(|range| range.from_start <= element && element <= range.from_start + range.length);
        if let Some(matching_range) = matching_range {
            result.insert(element - matching_range.from_start + matching_range.to_start);
        }
        else {
            result.insert(element);
        }
    }

    result
}

fn apply_map_to_ranges(source_ranges: impl Iterator<Item = AlmanacRange>, map: &AlmanacMap) -> Vec<AlmanacRange> {
    let mut result = vec![];
    let mut unmapped_ranges: Vec<_> = source_ranges.collect();

    for range_mapping in &map.range_mappings {
        let mut unmapped_for_this_mapping: Vec<AlmanacRange> = vec![];
        for range in &unmapped_ranges {
            if let Some(mapped) = apply_range_mapping(&range, &range_mapping) {
                let mapped_portion = AlmanacRange { start: range_mapping.from_start, length: range_mapping.length };
                let (left_remainder, right_remainder) = subtract_range(&range, &mapped_portion);
                
                if let Some(left_remainder) = left_remainder {
                    unmapped_for_this_mapping.push(left_remainder);
                }

                if let Some(right_remainder) = right_remainder {
                    unmapped_for_this_mapping.push(right_remainder);
                }

                result.push(mapped);
            }
            else {
                unmapped_for_this_mapping.push(range.clone());
            }
        }
        unmapped_ranges = unmapped_for_this_mapping;
    }

    result.extend(unmapped_ranges);
    result
}

/// Maps `source_range` using `mapping`, returning the mapped portion of `source_range` that overlaps
/// with the mapping. Returns None if the `source_range` is not mapped by `mapping`.
fn apply_range_mapping(source_range: &AlmanacRange, mapping: &AlmanacRangeMapping) -> Option<AlmanacRange> {
    let is_disjoint = source_range.start >= mapping.from_start + mapping.length
                    ||source_range.start + source_range.length <= mapping.from_start;
    
    if is_disjoint {
        return None;
    }
    
    let overlap_start = std::cmp::max(source_range.start, mapping.from_start);
    let overlap_end = std::cmp::min(source_range.start + source_range.length, mapping.from_start + mapping.length);
    let new_start = overlap_start - mapping.from_start + mapping.to_start;
    let new_length = overlap_end - overlap_start;
    Some(AlmanacRange { start: new_start, length: new_length })
}

/// Returns two new `AlmanacRange` obtained by subtracting `subtracting_range` from `source_range`.
/// The two ranges are to the left and the right of the subtracting area, respectively.
/// Either can be None if the is no remaining range to either the left or the right.
fn subtract_range(source_range: &AlmanacRange, subtracting_range: &AlmanacRange) -> (Option<AlmanacRange>, Option<AlmanacRange>) {
    let overlap_start = std::cmp::max(source_range.start, subtracting_range.start);
    let overlap_end = std::cmp::min(source_range.start + source_range.length, subtracting_range.start + subtracting_range.length);
    
    let left_side = if overlap_start > source_range.start {
        Some(AlmanacRange { start: source_range.start, length: overlap_start - source_range.start })
    }
    else {
        None
    };

    let right_side = if overlap_end < source_range.start + source_range.length {
        Some(AlmanacRange { start: overlap_end, length: source_range.start + source_range.length - overlap_end })
    }
    else {
        None
    };

    (left_side, right_side)
}

fn solve_problem_1(almanac: &Almanac) -> Option<u64> {
    let mut items = almanac.seeds.clone();
    let mut label = "seed".to_string();
    
    while let Some(map) = almanac.maps_by_source.get(&label) {
        items = apply_map_to_elements(items.into_iter(), &map);
        label = map.to.clone();
    };
    
    items.iter().min().copied()
}

fn solve_problem_2(almanac: &Almanac) -> Option<u64> {
    let mut item_ranges = almanac.seeds_as_ranges.clone();
    let mut label = "seed".to_string();

    while let Some(map) = almanac.maps_by_source.get(&label) {
        item_ranges = apply_map_to_ranges(item_ranges.into_iter(), &map);
        label = map.to.clone();
    }

    item_ranges.iter().map(|range| range.start).min()
}

fn main() {
    let file = File::open("inputs/2023/05/input.txt").unwrap();
    let lines = BufReader::new(file).lines().filter_map(|line| line.ok());
    let almanac = parse_input(lines).unwrap();

    let solution_1 = solve_problem_1(&almanac).unwrap();
    let solution_2 = solve_problem_2(&almanac).unwrap();

    println!("Solution 1: {solution_1}");
    println!("Solution 2: {solution_2}");
}

#[cfg(test)]
mod test_parsing {
    use super::*;

    #[test]
    fn parse_seeds() {
        let source = vec!["seeds: 1 2 3 4"];
        let almanac = parse_input(source.iter()).unwrap();
        
        assert_eq!(almanac.seeds.len(), 4);
        assert!(almanac.seeds.contains(&1));
        assert!(almanac.seeds.contains(&2));
        assert!(almanac.seeds.contains(&3));
        assert!(almanac.seeds.contains(&4));

        assert_eq!(almanac.seeds_as_ranges.len(), 2);
        assert_eq!(almanac.seeds_as_ranges[0].start, 1);
        assert_eq!(almanac.seeds_as_ranges[0].length, 2);
        assert_eq!(almanac.seeds_as_ranges[1].start, 3);
        assert_eq!(almanac.seeds_as_ranges[1].length, 4);
    } 

    #[test]
    fn parse_single_map() {
        let source = vec!["a-to-b map:", "1 2 3", "4 5 6"];
        let almanac = parse_input(source.iter()).unwrap();

        assert_eq!(almanac.maps_by_source.len(), 1);
        assert!(almanac.maps_by_source.contains_key("a"));
        
        let from_a = almanac.maps_by_source.get("a").unwrap();
        assert_eq!(from_a.to, "b");

        assert_eq!(from_a.range_mappings.len(), 2);
        let range_1 = &from_a.range_mappings[0];
        let range_2 = &from_a.range_mappings[1];

        assert_eq!(range_1.from_start, 2);
        assert_eq!(range_1.to_start, 1);
        assert_eq!(range_1.length, 3);

        assert_eq!(range_2.from_start, 5);
        assert_eq!(range_2.to_start, 4);
        assert_eq!(range_2.length, 6);
    }

    #[test]
    fn parse_multiple_maps() {
        let source = vec!["a-to-b map:", "1 2 3", "b-to-c map:", "4 5 6"];
        let almanac = parse_input(source.iter()).unwrap();

        assert_eq!(almanac.maps_by_source.len(), 2);
        assert!(almanac.maps_by_source.contains_key("a"));
        assert!(almanac.maps_by_source.contains_key("b"));

        let a_to_b = almanac.maps_by_source.get("a").unwrap();
        let b_to_c = almanac.maps_by_source.get("b").unwrap();

        assert_eq!(a_to_b.range_mappings.len(), 1);
        assert_eq!(b_to_c.range_mappings.len(), 1);
        let range_1 = &a_to_b.range_mappings[0];
        let range_2 = &b_to_c.range_mappings[0];

        assert_eq!(range_1.from_start, 2);
        assert_eq!(range_1.to_start, 1);
        assert_eq!(range_1.length, 3);

        assert_eq!(range_2.from_start, 5);
        assert_eq!(range_2.to_start, 4);
        assert_eq!(range_2.length, 6);
    }

}

#[cfg(test)]
mod test_mapping {
    use super::*;

    fn make_map(from_start: u64, to_start: u64, length: u64) -> AlmanacMap {
        let range = AlmanacRangeMapping { from_start, to_start, length };
        AlmanacMap { to: "".to_string(), range_mappings: vec![range] }
    }

    #[test]
    fn map_in_range_elements() {
        let map = make_map(10, 20, 5);
        let source = vec![13, 15];
        let result = apply_map_to_elements(source.into_iter(), &map);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&23));
        assert!(result.contains(&25));
    }

    #[test]
    fn map_before_range_elements() {
        let map = make_map(10, 20, 5);
        let source = vec![5, 8];
        let result = apply_map_to_elements(source.into_iter(), &map);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&5));
        assert!(result.contains(&8));
    }

    #[test]
    fn map_after_range_elements() {
        let map = make_map(10, 20, 5);
        let source = vec![17, 19];
        let result = apply_map_to_elements(source.into_iter(), &map);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&17));
        assert!(result.contains(&19));
    }

    #[test]
    fn map_multiple_range_elements() {
        let mut map = make_map(10, 20, 5);
        map.range_mappings.push(AlmanacRangeMapping { from_start: 30, to_start: 40, length: 5});

        let source = vec![11, 33];
        let result = apply_map_to_elements(source.into_iter(), &map);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&21));
        assert!(result.contains(&43));
    }

    #[test]
    fn map_range_whole() {
        let mapping = AlmanacRangeMapping { from_start: 10, to_start: 20, length: 5 };
        let source_range = AlmanacRange { start: 12, length: 2 };
        let mapped_range = apply_range_mapping(&source_range, &mapping).unwrap();

        assert_eq!(mapped_range.start, 22);
        assert_eq!(mapped_range.length, 2);
    }

    #[test]
    fn map_range_partial_before() {
        let mapping = AlmanacRangeMapping { from_start: 10, to_start: 20, length: 5 };
        let source_range = AlmanacRange { start: 8, length: 4 };
        let mapped_range = apply_range_mapping(&source_range, &mapping).unwrap();

        assert_eq!(mapped_range.start, 20);
        assert_eq!(mapped_range.length, 2);
    }

    #[test]
    fn map_range_partial_after() {
        let mapping = AlmanacRangeMapping { from_start: 10, to_start: 20, length: 5 };
        let source_range = AlmanacRange { start: 13, length: 4 };
        let mapped_range = apply_range_mapping(&source_range, &mapping).unwrap();

        assert_eq!(mapped_range.start, 23);
        assert_eq!(mapped_range.length, 2);
    }

    #[test]
    fn map_range_encompassing() {
        let mapping = AlmanacRangeMapping { from_start: 10, to_start: 20, length: 5 };
        let source_range = AlmanacRange { start: 8, length: 10 };
        let mapped_range = apply_range_mapping(&source_range, &mapping).unwrap();

        assert_eq!(mapped_range.start, 20);
        assert_eq!(mapped_range.length, 5);        
    }

    #[test]
    fn map_range_disjoint() {
        let mapping = AlmanacRangeMapping { from_start: 10, to_start: 20, length: 5 };
        let source_range = AlmanacRange { start: 5, length: 5 };
        let mapped_range_optional = apply_range_mapping(&source_range, &mapping);
        assert!(mapped_range_optional.is_none());

        let source_range = AlmanacRange { start: 15, length: 5 };
        let mapped_range_optional = apply_range_mapping(&source_range, &mapping);
        assert!(mapped_range_optional.is_none());
    }

}

#[cfg(test)]
mod test_subtraction {
    use super::*;

    #[test]
    fn test_subtract_subset_right() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 8, length: 2 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);

        assert!(right_side.is_none());

        let left_side = left_side.unwrap();
        assert_eq!(left_side.start, 5);
        assert_eq!(left_side.length, 3);
    }

    #[test]
    fn test_subtract_subset_left() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 5, length: 2 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);

        assert!(left_side.is_none());

        let right_side = right_side.unwrap();
        assert_eq!(right_side.start, 7);
        assert_eq!(right_side.length, 3);
    }

    #[test]
    fn test_subtract_inner() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 6, length: 2 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);

        let left_side = left_side.unwrap();
        assert_eq!(left_side.start, 5);
        assert_eq!(left_side.length, 1);

        let right_side = right_side.unwrap();
        assert_eq!(right_side.start, 8);
        assert_eq!(right_side.length, 2);
    }

    #[test]
    fn test_subtract_outer() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 4, length: 8 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);
        assert!(left_side.is_none());
        assert!(right_side.is_none());
    }

    #[test]
    fn test_subtract_disjoint_left() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 3, length: 2 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);
        assert!(left_side.is_none());
        
        let right_side = right_side.unwrap();
        assert_eq!(right_side.start, 5);
        assert_eq!(right_side.length, 5);
    }

    #[test]
    fn test_subtract_disjoint_right() {
        let source_range = AlmanacRange { start: 5, length: 5 };
        let subtracting_range = AlmanacRange { start: 10, length: 5 };
        let (left_side, right_side) = subtract_range(&source_range, &subtracting_range);
        assert!(right_side.is_none());
        
        let left_side = left_side.unwrap();
        assert_eq!(left_side.start, 5);
        assert_eq!(left_side.length, 5);
    }
}