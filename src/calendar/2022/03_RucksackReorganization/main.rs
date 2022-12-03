use itertools::Itertools;
use std::{ collections::HashSet, fs::File, io::{ BufRead, BufReader } };

enum ItemError { NotAnItem }

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Item(char);

impl TryFrom<char> for Item {
    type Error = ItemError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A'..='Z' | 'a'..='z' => Ok(Item(value)),
            _ => Err(ItemError::NotAnItem)
        }
    }
}

impl Item {
    const UPPERCASE_FIRST_PRIORITY : i32 = 27;
    const LOWERCASE_FIRST_PRIORITY : i32 = 1;

    fn get_priority(&self) -> Option<i32> {
        match self.0 {
            'A'..='Z' => Some(self.0 as i32 - 'A' as i32 + Item::UPPERCASE_FIRST_PRIORITY),
            'a'..='z'=> Some(self.0 as i32 - 'a' as i32 + Item::LOWERCASE_FIRST_PRIORITY),
            _ => None
        }
    }
}

#[derive(Debug)]
enum RucksackError { Empty, Unbalanced(usize), InvalidItems }

struct Rucksack {
    left_compartment: HashSet<Item>,
    right_compartment: HashSet<Item>,
}

impl Rucksack {
    fn parse_compartment<Iter>(chars: Iter) -> Result<HashSet<Item>, RucksackError> 
    where Iter : Iterator<Item = char>
    {
        chars.map(Item::try_from)
            .collect::<Result<HashSet<Item>, ItemError>>()
            .map_err(|_| RucksackError::InvalidItems)
    }

    fn get_duplicate_items(&self) -> HashSet<Item> {
        self.left_compartment.intersection(&self.right_compartment).copied().collect()
    }

    fn get_all_items(&self) -> HashSet<Item> {
        self.left_compartment.union(&self.right_compartment).copied().collect()
    }
}

impl TryFrom<&str> for Rucksack {
    type Error = RucksackError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let len = value.len();
        if len % 2 == 1 {
            return Err(RucksackError::Unbalanced(value.len()));
        }

        let half_size = value.len() / 2;
        if half_size == 0 {
            return Err(RucksackError::Empty);
        }

        let left_compartment = Rucksack::parse_compartment(value[0..half_size].chars())?;
        let right_compartment = Rucksack::parse_compartment(value[half_size..len].chars())?;

        Ok(Rucksack { left_compartment, right_compartment })
    }
}

#[derive(Debug)]
enum ParsingError { NoInputFile(String), IoError(std::io::Error), RucksackParsing(RucksackError) }

fn parse_input(input_path: &str) -> Result<Vec<Rucksack>, ParsingError> {
    let input_file = File::open(input_path)
        .map_err(|_| ParsingError::NoInputFile(String::from(input_path)))?;

    let input_lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .try_collect()
        .map_err(|e| ParsingError::IoError(e))?;

    let rucksacks: Vec<Rucksack> = input_lines.iter()
        .map(String::as_str)
        .map(Rucksack::try_from)
        .try_collect()
        .map_err(|e| ParsingError::RucksackParsing(e))?;

    Ok(rucksacks)
}

fn sum_priorities_of_duplicates<'a, Iter>(rucksacks: Iter) -> i32
where Iter : Iterator<Item = &'a Rucksack>
{
    rucksacks.map(Rucksack::get_duplicate_items)
        .map(|duplicates| duplicates.iter().filter_map(Item::get_priority).sum::<i32>())
        .sum()
}

fn get_common_item<'a, Iter>(mut item_sets: Iter) -> Option<Item>
where Iter: Iterator<Item = HashSet<Item>>
{
    let mut intersection = item_sets.next()?;
    intersection = item_sets.fold(intersection, |mut current, next| { current.retain(|item| next.contains(item)); current });
    let only_item = intersection.iter().next()?;
    Some(*only_item)
}

fn find_badges_and_sum_priorities<'a, Iter>(rucksacks: Iter) -> i32
where Iter : Iterator<Item = &'a Rucksack>
{
    rucksacks
        .map(Rucksack::get_all_items).into_iter()
        .chunks(3).into_iter()
        .filter_map(|chunk| get_common_item(chunk.into_iter()))
        .filter_map(|item| item.get_priority())
        .sum()
}

fn main() {
    let rucksacks = parse_input("inputs/2022/03/RucksackReorganization.txt");
    match rucksacks {
        Ok(rucksacks) => {
            println!("Summed priorities of duplicates: {}", sum_priorities_of_duplicates(rucksacks.iter()));
            println!("Summed priorities of found badges: {}", find_badges_and_sum_priorities(rucksacks.iter()));
        },
        Err(error) => {
            println!("{error:?}");
        }
    }
}