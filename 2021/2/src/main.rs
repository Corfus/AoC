use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use aoc_framework::{AoC, Level, parse_capture};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
enum Command {
    Forward,
    Down,
    Up,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Command::Forward),
            "down" => Ok(Command::Down),
            "up" => Ok(Command::Up),
            &_ => Err(())
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32
}

#[derive(Copy, Clone, Debug, Default)]
struct Position2 {
    horizontal: i32,
    depth: i32,
    aim: i32
}

trait PositionTrait {
    fn new(command: Command, step: i32) -> Self;

    fn multiply(&self) -> i32;
}

impl PositionTrait for Position {
    fn new(command: Command, step: i32) -> Position {
        match command {
            Command::Forward => Position {horizontal: step, depth: 0},
            Command::Down => Position {horizontal: 0, depth: step},
            Command::Up => Position {horizontal: 0, depth: -step},
        }
    }

    fn multiply(&self) -> i32 {
        self.horizontal * self.depth
    }
}

impl PositionTrait for Position2 {
    fn new(command: Command, step: i32) -> Position2 {
        match command {
            Command::Forward => Position2 {horizontal: step, depth: 0, aim: 0},
            Command::Down => Position2 {horizontal: 0, depth: 0, aim: step},
            Command::Up => Position2 {horizontal: 0, depth: 0, aim: -step},
        }
    }

    fn multiply(&self) -> i32 {
        self.horizontal * self.depth
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            horizontal: self.horizontal + rhs.horizontal,
            depth: self.depth + rhs.depth,
        }
    }
}

impl Add for Position2 {
    type Output = Position2;

    fn add(self, rhs: Self) -> Self::Output {
        Position2 {
            horizontal: self.horizontal + rhs.horizontal,
            depth: self.depth + rhs.horizontal * self.aim,
            aim: self.aim + rhs.aim
        }
    }
}

impl Sum for Position {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Position::default(), |a, b| a + b)
    }
}

impl Sum for Position2 {
    fn sum<I: Iterator<Item=Self>>(iter: I) -> Self {
        iter.fold(Position2::default(), |a, b| a + b)
    }
}

fn prepare_data<T: PositionTrait>(input: Vec<String>) -> Option<Vec<T>> {
    input
        .iter()
        .cloned()
        .map(|s| {
            Regex::new("(.*) (.*)")
                .ok()
                .and_then(|re| {
                    re
                        .captures(&s)
                        .and_then(|cap| {
                            parse_capture::<Command>(&cap, 1)
                                .and_then(|command| parse_capture::<i32>(&cap, 2)
                                    .map(|step| T::new(command, step)))
                        })
                })
        })
        .collect::<Option<Vec<_>>>()
}

fn task1(input: Vec<Position>) -> Option<String> {
    Some(
        input
            .iter()
            .cloned()
            .sum::<Position>()
            .multiply()
            .to_string()
    )
}

fn task2(input: Vec<Position2>) -> Option<String> {
    Some(
        input
            .iter()
            .cloned()
            .sum::<Position2>()
            .multiply()
            .to_string()
    )
}

fn main() {
    AoC::new(2021, 2).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
