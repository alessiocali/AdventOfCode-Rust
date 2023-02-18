use crate::error;
use std::{ clone::Clone, hash::Hash, cmp::Eq };

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    pub fn value(&self) -> Point {
        match &self {
            Direction::Left => Point { x: -1, y: 0 },
            Direction::Right => Point { x: 1, y: 0 },
            Direction::Up => Point { x: 0, y: 1 },
            Direction::Down => Point { x: 0, y: -1 },
        }
    }
}

impl<'a> TryFrom<&'a str> for Direction {
    type Error = error::Error;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        value
            .chars()
            .next()
            .and_then(|char| match char {
                'L' => Some(Direction::Left),
                'R' => Some(Direction::Right),
                'U' => Some(Direction::Up),
                'D' => Some(Direction::Down),
                _ => None
            })
            .ok_or(error::Error::DirectionParsingError(value.to_string()))
    }
}

pub type Path = Vec<Direction>;