use std::str::FromStr;
use aoc_framework::{AoC, Level};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Bracket {
    OpenRound,
    ClosedRound,
    OpenSquare,
    ClosedSquare,
    OpenCurved,
    ClosedCurved,
    LessThan,
    GreaterThan,
}

impl FromStr for Bracket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Bracket::OpenRound),
            ")" => Ok(Bracket::ClosedRound),
            "[" => Ok(Bracket::OpenSquare),
            "]" => Ok(Bracket::ClosedSquare),
            "{" => Ok(Bracket::OpenCurved),
            "}" => Ok(Bracket::ClosedCurved),
            "<" => Ok(Bracket::LessThan),
            ">" => Ok(Bracket::GreaterThan),
            &_ => Err(())
        }
    }
}

impl Bracket {
    pub fn get_score(&self) -> u32 {
        match self {
            Bracket::ClosedRound => 3,
            Bracket::ClosedSquare => 57,
            Bracket::ClosedCurved => 1197,
            _ => 25137
        }
    }

    pub fn get_missing_score(&self) -> usize {
        match self {
            Bracket::ClosedRound => 1,
            Bracket::ClosedSquare => 2,
            Bracket::ClosedCurved => 3,
            _ => 4
        }
    }

    pub fn get_counter(&self) -> Self {
        match self {
            Bracket::OpenRound => Bracket::ClosedRound,
            Bracket::ClosedRound => Bracket::OpenRound,
            Bracket::OpenSquare => Bracket::ClosedSquare,
            Bracket::ClosedSquare => Bracket::OpenSquare,
            Bracket::OpenCurved => Bracket::ClosedCurved,
            Bracket::ClosedCurved => Bracket::OpenCurved,
            Bracket::LessThan => Bracket::GreaterThan,
            Bracket::GreaterThan => Bracket::LessThan
        }
    }
}

const OPEN_BRACKETS: [Bracket; 4] = [
    Bracket::OpenRound,
    Bracket::OpenSquare,
    Bracket::OpenCurved,
    Bracket::LessThan,
];

const CLOSED_BRACKETS: [Bracket; 4] = [
    Bracket::ClosedRound,
    Bracket::ClosedSquare,
    Bracket::ClosedCurved,
    Bracket::GreaterThan,
];

type PreparedInput = Vec<Bracket>;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .map(|s| s.chars().map(|c| c.to_string().parse::<Bracket>().ok()).collect::<Option<Vec<Bracket>>>())
        .collect::<Option<Vec<Vec<_>>>>()
}

fn check_line(line: &Vec<Bracket>) -> (Option<Bracket>, Vec<Bracket>) {
    let mut br: Vec<Bracket> = vec![];
    let corrupted_bracket = line
        .iter()
        .cloned()
        .enumerate()
        .find_map(|(index, b)| {
            if OPEN_BRACKETS.contains(&b) {
                br.push(b.clone());
                None
            } else {
                br.clone().last()
                    .and_then(|last| {
                        if b.get_counter() == last.clone() {
                            br.pop();
                            None
                        } else {
                            Some(b)
                        }
                    })
            }
        });
    (corrupted_bracket, br)
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    input
        .iter()
        .map(|line| check_line(line))
        .map(|(br, _)| br)
        .filter(|opt| opt.is_some())
        .collect::<Option<Vec<Bracket>>>()
        .map(|chars| chars.iter().fold(0, |sum, b| sum * 5 + b.get_score()))
        .map(|value| value.to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let mut scores = input
        .iter()
        .map(|line| check_line(line))
        .filter(|(opt, _)| opt.is_none())
        .map(|(_, brackets)| brackets)
        .map(|mut b| {
            b.reverse();
            b.iter().fold(0 as usize, |s, v| s * 5 + v.get_counter().get_missing_score())
        })
        .collect::<Vec<usize>>();

    scores.sort();
    Some(scores[scores.len() / 2].to_string())
}

fn main() {
    AoC::<PreparedInput>::new(2021, 10).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
