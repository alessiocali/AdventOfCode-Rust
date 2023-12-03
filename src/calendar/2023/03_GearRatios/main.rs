use std::collections::{ HashMap, HashSet };
use std::fs::File;
use std::io::{ BufReader, BufRead };

enum SchematicGlyph {
    Digit(u8),
    Period,
    Gear,
    Symbol
}

struct Schematic {
    rows: Vec<Vec<SchematicGlyph>>
}

struct GearJunction<'a> {
    part_1: &'a [SchematicGlyph],
    part_2: &'a [SchematicGlyph]
}

struct SchematicPart<'a> {
    glyphs: &'a [SchematicGlyph],
    symbols_around: HashSet<(usize, usize)>
}

fn string_to_glyps(string: &str) -> Vec<SchematicGlyph> {
    string.chars().map(|ch| {
        if ch == (b'.' as char) {
            SchematicGlyph::Period
        }
        else if ch == (b'*' as char) {
            SchematicGlyph::Gear
        }
        else if let Some(digit) = ch.to_digit(10).and_then(|d| u8::try_from(d).ok()) {
            SchematicGlyph::Digit(digit)
        }
        else {
            SchematicGlyph::Symbol
        }
    })
    .collect::<Vec<_>>()
}

impl Schematic {
    fn new<T: AsRef<str>>(rows_slice: &[T]) -> Schematic {
        let rows = rows_slice
            .iter()
            .map(|row| string_to_glyps(row.as_ref()))
            .collect::<Vec<_>>();

        Schematic { rows }
    }

    fn get_at(&self, x: usize, y: usize) -> Option<&SchematicGlyph> {
        self.rows.get(y).and_then(|row| row.get(x))
    }

    fn get_parts(&self) -> Vec<SchematicPart> {
        let mut result = vec![];
        for (y, row) in self.rows.iter().enumerate() {
            let mut x_min : Option<usize> = None;
            let mut x_max : Option<usize> = None;
            let mut symbols_around : HashSet<(usize, usize)> = HashSet::new();

            let mut push_symbol = |symbols_around: HashSet<(usize, usize)>, x_min: &Option<usize>, x_max: &Option<usize>| {
                if !symbols_around.is_empty() {
                    let part = SchematicPart { glyphs: &row[x_min.unwrap()..=x_max.unwrap()], symbols_around };
                    result.push(part);
                };
            };

            for (x, glyph) in row.iter().enumerate() { 
                match glyph {
                    SchematicGlyph::Digit(_) => {
                        x_min = x_min.or(Some(x));
                        x_max = Some(x);
                        symbols_around.extend(self.get_symbols_around(x, y).iter());
                    },
                    _ => {
                        push_symbol(symbols_around, &x_min, &x_max);
                        x_min = None;
                        x_max = None;
                        symbols_around = HashSet::new();
                    }
                }
            }

            push_symbol(symbols_around, &x_min, &x_max)
        };

        result
    }

    fn get_symbols_around(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for x_offset in -1i8..=1 {
            let x_around = if let Ok(x_around) = usize::try_from(x as i64 + x_offset as i64) { x_around } else { continue };
            for y_offset in -1i8..=1 {
                let y_around = if let Ok(y_around) = usize::try_from(y as i64 + y_offset as i64) { y_around } else { continue };
                if let Some(SchematicGlyph::Symbol | SchematicGlyph::Gear) = self.get_at(x_around, y_around) {
                    result.push((x_around, y_around))
                }
            }
        }

        result
    }

    fn get_all_gears<'a>(&self, parts: &Vec<SchematicPart<'a>>) -> Vec<GearJunction<'a>> {
        let mut gears_symbols_to_adjacent_parts: HashMap<(usize, usize), Vec<&SchematicPart>> = HashMap::new();
        for part in parts {
            for symbol in &part.symbols_around {
                if let Some(SchematicGlyph::Gear) = self.get_at(symbol.0, symbol.1) {
                    gears_symbols_to_adjacent_parts.entry((symbol.0, symbol.1)).or_default().push(part);
                }
            };
        }
    
        let mut result = vec![];
        for (_, parts) in gears_symbols_to_adjacent_parts.iter().filter(|(_, value)| value.len() == 2) {
            result.push(GearJunction { part_1: parts[0].glyphs, part_2: parts[1].glyphs });
        }
    
        result
    }

}

fn get_glyph_number(part: &[SchematicGlyph]) -> u32 {
    let digits = part.iter().filter_map(|glyph| if let SchematicGlyph::Digit(digit) = glyph { Some(digit) } else { None });
    digits.rev().enumerate().fold(0u32, |acc, (idx, digit)| acc + *digit as u32 * 10u32.pow(idx as u32))
}

fn main() {
    let file = File::open("inputs/2023/03/input.txt").unwrap();
    let lines = BufReader::new(file).lines().filter_map(|line_result| line_result.ok()).collect::<Vec<_>>();
    let schematic = Schematic::new(&lines[..]);
    let parts = schematic.get_parts();
    let solution_1 = 
        parts
        .iter()
        .map(|part| get_glyph_number(&part.glyphs) as u64)
        .sum::<u64>();

    let solution_2 = 
        schematic
        .get_all_gears(&parts)
        .iter()
        .map(|gear_junction| (get_glyph_number(gear_junction.part_1) * get_glyph_number(gear_junction.part_2)) as u64)
        .sum::<u64>();

    println!("Solution 1: {solution_1}");
    println!("Solution 2: {solution_2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_single_part() {
        let schematic = Schematic::new(&vec![
            "..123..",
            "...#..."
        ]);

        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].glyphs.len(), 3);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));
        assert!(matches!(parts[0].glyphs[2], SchematicGlyph::Digit(3)));
    }

    #[test]
    fn test_single_digit() {
        let schematic = Schematic::new(&vec!["*1.2"]);
        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].glyphs.len(), 1);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
    }

    #[test]
    fn test_multiple_parts() {
        let schematic = Schematic::new(&vec![
            ".12.34.",
            "...#..."
        ]);

        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 2);

        assert_eq!(parts[0].glyphs.len(), 2);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));

        assert_eq!(parts[1].glyphs.len(), 2);
        assert!(matches!(parts[1].glyphs[0], SchematicGlyph::Digit(3)));
        assert!(matches!(parts[1].glyphs[1], SchematicGlyph::Digit(4)));
    }

    #[test]
    fn test_multiple_symbols() {
        let schematic = Schematic::new(&vec![
            ".12.34.",
            ".#....$"
        ]);

        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 2);

        assert_eq!(parts[0].glyphs.len(), 2);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));

        assert_eq!(parts[1].glyphs.len(), 2);
        assert!(matches!(parts[1].glyphs[0], SchematicGlyph::Digit(3)));
        assert!(matches!(parts[1].glyphs[1], SchematicGlyph::Digit(4)));
    }

    #[test]
    fn test_symbol_in_between() {
        let schematic = Schematic::new(&vec![
            ".12$34."
        ]);

        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 2);

        assert_eq!(parts[0].glyphs.len(), 2);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));

        assert_eq!(parts[1].glyphs.len(), 2);
        assert!(matches!(parts[1].glyphs[0], SchematicGlyph::Digit(3)));
        assert!(matches!(parts[1].glyphs[1], SchematicGlyph::Digit(4)));
    }

    #[test]
    fn test_near_symbols_and_digits() {
        let schematic = Schematic::new(&vec![
            "..12..",
            ".34#.."
        ]);

        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 2);

        assert_eq!(parts[0].glyphs.len(), 2);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));

        assert_eq!(parts[1].glyphs.len(), 2);
        assert!(matches!(parts[1].glyphs[0], SchematicGlyph::Digit(3)));
        assert!(matches!(parts[1].glyphs[1], SchematicGlyph::Digit(4)));
    }

    #[test]
    fn test_close_but_separated() {
        let schematic = Schematic::new(&vec!["*123.456"]);
        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);

        assert_eq!(parts[0].glyphs.len(), 3);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));
        assert!(matches!(parts[0].glyphs[2], SchematicGlyph::Digit(3)));
    }

    #[test]
    fn test_end_of_line() {
        let schematic = Schematic::new(&vec!["*123"]);
        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);

        assert_eq!(parts[0].glyphs.len(), 3);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
        assert!(matches!(parts[0].glyphs[1], SchematicGlyph::Digit(2)));
        assert!(matches!(parts[0].glyphs[2], SchematicGlyph::Digit(3)));
    }

    #[test]
    fn test_gear_is_symbol() {
        let schematic = Schematic::new(&vec!["1*"]);
        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);

        assert_eq!(parts[0].glyphs.len(), 1);
        assert!(matches!(parts[0].glyphs[0], SchematicGlyph::Digit(1)));
    }

    #[test]
    fn test_part_near_symbols() {
        let schematic = Schematic::new(&vec![
            "*.#",
            ".1.",
            "%.$"
        ]);
        let parts = schematic.get_parts();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].symbols_around.len(), 4);
        assert!(parts[0].symbols_around.contains(&(0,0)));
        assert!(parts[0].symbols_around.contains(&(2,0)));
        assert!(parts[0].symbols_around.contains(&(0,2)));
        assert!(parts[0].symbols_around.contains(&(2,2)));
    }

    #[test]
    fn test_part_number() {
        let glyphs = vec![SchematicGlyph::Digit(1), SchematicGlyph::Digit(2), SchematicGlyph::Digit(3)];
        assert_eq!(get_glyph_number(&glyphs), 123);
    }
}