use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{ fs::File, io::{ BufRead, BufReader } };

#[derive(Debug)]
enum ParsingError { NoStackLabels, InvalidCargoLabel(String), InvalidInstruction(String), OutOfBoundsStack(usize) }

#[derive(Debug)]
enum InstructionError { OutOfBoundsStack(usize), StackUnderflow(usize) }

#[derive(Debug)]
enum Error { InstructionErrors(InstructionError), IoError(std::io::Error), ParsingErrors(ParsingError), RegexError(regex::Error) }

impl From<InstructionError> for Error {
    fn from(error: InstructionError) -> Self {
        Error::InstructionErrors(error)
    }
}

impl From<std::io::Error> for Error { 
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl From<ParsingError> for Error {
    fn from(error: ParsingError) -> Self {
        Error::ParsingErrors(error)
    }
}

impl From<&regex::Error> for Error {
    fn from(error: &regex::Error) -> Self {
        Error::RegexError(error.clone())
    }
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize
}

type Cargo = Vec<Vec<char>>;
type Instructions = Vec<Instruction>;

fn parse_cargo_label_entry(cargo_label_entry: &str) -> Result<Option<char>, Error> {
    lazy_static! {
        static ref CRATE_REGEX: Result<Regex, regex::Error> = Regex::new(r"\[(\w)\]");
    }

    let captured_label = CRATE_REGEX.as_ref()?.captures(cargo_label_entry);
    match captured_label {
        Some(capture) => {
            match capture.get(1).and_then(|label| label.as_str().chars().next()) {
                Some(label) => Ok(Some(label)),
                None => Err(Error::from(ParsingError::InvalidCargoLabel(String::from(cargo_label_entry))))
            }
        },
        None => Ok(None)
    }
}

fn parse_cargo(cargo_lines: Vec<String>) -> Result<Cargo, Error> {
    lazy_static! {
        static ref LABEL_REGEX: Result<Regex, regex::Error> = Regex::new(r"\d+");
        static ref CRATE_OR_NULL_REGEX: Result<Regex, regex::Error> = Regex::new(r"(\[\w\]|\s{3})\s?");
    }

    let unwrapped_label_regex = LABEL_REGEX.as_ref()?;
    let unwrapped_crate_or_null_regex = CRATE_OR_NULL_REGEX.as_ref()?;
    
    let mut cargo = Cargo::new();

    let mut cargo_lines_iter = cargo_lines.iter().rev();

    let stack_line = cargo_lines_iter.by_ref().next().ok_or(ParsingError::NoStackLabels)?;
    let stack_labels_count = unwrapped_label_regex.find_iter(stack_line.as_str()).count();
    cargo.reserve(stack_labels_count);
    for _ in 0..stack_labels_count {
        cargo.push(Vec::<char>::new());
    }

    for cargo_line in cargo_lines_iter {
        let crates_iter = unwrapped_crate_or_null_regex
            .find_iter(cargo_line.as_str())
            .enumerate()
            .filter_map(|(index, regex_match)| match parse_cargo_label_entry(regex_match.as_str()) {
                Ok(Some(label)) => Some(Ok((index, label))),
                Ok(None) => None,
                Err(error) => Some(Err(error))
            });

        for parsed_crate_line in crates_iter {
            let (index, crate_label) = parsed_crate_line?;
            let stack = cargo.get_mut(index).ok_or(ParsingError::OutOfBoundsStack(index))?;
            stack.push(crate_label);
        }
    }

    Ok(cargo)
}

fn parse_instruction<'a>(instruction_line: &'a str) -> Result<Instruction, Error> { 
    lazy_static! {
        static ref INSTRUCTION_REGEX: Result<Regex, regex::Error> = Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)");
    }

    let unwrapped_instruction_regex = INSTRUCTION_REGEX.as_ref()?;
    let captures = unwrapped_instruction_regex
        .captures(instruction_line)
        .ok_or(ParsingError::InvalidInstruction(String::from(instruction_line)))?;

    let capture_to_usize = |capture: regex::Match| -> Option<usize> { capture.as_str().parse::<usize>().ok() };
    let amount = captures.name("amount").and_then(capture_to_usize);
    let from = captures.name("from").and_then(capture_to_usize);
    let to = captures.name("to").and_then(capture_to_usize);

    match (amount, from, to) {
        (Some(amount), Some(from), Some(to)) => Ok(Instruction { amount, from, to }),
        _ => Err(Error::from(ParsingError::InvalidInstruction(String::from(instruction_line))))
    }
}

fn parse_instructions<Iter>(instruction_iter: Iter) -> Result<Instructions, Error>
where Iter: Iterator<Item = Result<String, Error>> 
{
    let instructions: Instructions = instruction_iter
        .map(|line| match line {
            Ok(line) => parse_instruction(line.as_str()),
            Err(err) => Err(err),
        })
        .try_collect()?;

    Ok(instructions)
}

fn parse_input_file(path: &str) -> Result<(Cargo, Instructions), Error> {
    let input_file = File::open(path)?;
    let mut reader_it = BufReader::new(input_file).lines();

    let cargo_lines: Vec<_> = reader_it
        .by_ref()
        .take_while(|line_result| line_result.is_ok() && !line_result.as_ref().unwrap().is_empty())
        .try_collect()
        .map_err(|e| Error::IoError(e))?;

    let cargo = parse_cargo(cargo_lines)?;
    let instructions = parse_instructions(
        reader_it.map(|line| match line {
            Ok(line) => Ok(line),
            Err(err) => Err(Error::from(err))
        })
    )?;

    Ok((cargo, instructions))
}

fn apply_instructions_with_stacks(cargo: &Cargo, instructions: &Instructions) -> Result<Cargo, Error> {
    let mut result = cargo.clone();

    for instruction in instructions {
        let from_index = instruction.from - 1;
        let to_index = instruction.to - 1;

        for _ in 0..instruction.amount {
            let to_move = result.get_mut(from_index)
                .ok_or(InstructionError::OutOfBoundsStack(from_index))?
                .pop()
                .ok_or(InstructionError::StackUnderflow(from_index))?;

            result.get_mut(to_index)
                .ok_or(InstructionError::OutOfBoundsStack(to_index))?
                .push(to_move);
        }
    }

    Ok(result)
}

fn apply_instructions_with_slices(cargo: &Cargo, instructions: &Instructions) -> Result<Cargo, Error> {
    let mut result = cargo.clone();

    for instruction in instructions {
        let from_index = instruction.from - 1;
        let to_index = instruction.to - 1;

        let from_size = result.get(from_index).ok_or(InstructionError::OutOfBoundsStack(from_index))?.len();
        if from_size < instruction.amount {
            return Err(Error::from(InstructionError::StackUnderflow(from_index)));
        }
        
        let new_size = from_size - instruction.amount;
        let to_move = result.get_mut(from_index)
            .ok_or(InstructionError::OutOfBoundsStack(from_index))?
            .drain(new_size..)
            .collect_vec();

        result.get_mut(to_index)
            .ok_or(InstructionError::OutOfBoundsStack(to_index))?
            .extend(to_move);
    }

    Ok(result)
}

fn get_topmost_crates(cargo: &Cargo) -> String {
    cargo.iter()
        .map(|stack| stack.last().copied().unwrap_or(' '))
        .join("")
}

fn main() {
    let (cargo, instructions) = match parse_input_file("inputs/2022/05/SupplyStacks.txt") {
        Ok((cargo, instructions)) => (cargo, instructions),
        Err(err) => {
            println!("{err:?}");
            std::process::exit(1);
        }
    };
    
    let topmost_9000 = apply_instructions_with_stacks(&cargo, &instructions).and_then(|cargo| Ok(get_topmost_crates(&cargo)));
    let topmost_9001 = apply_instructions_with_slices(&cargo, &instructions).and_then(|cargo| Ok(get_topmost_crates(&cargo)));
    
    match (topmost_9000, topmost_9001) {
        (Ok(topmost_9000), Ok(topmost_9001)) => {
            println!("Topmost crates: {topmost_9000}");
            println!("Topmost crates: {topmost_9001}");
        },
        (Err(err), _) => println!("{err:?}"),
        (_, Err(err)) => println!("{err:?}")
    }
}