use aoc_framework::{AoC, Level, parse_capture};
use regex::{Regex};

type PreparedInput = (usize, usize, String, String);

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| {
            Regex::new("(.*)-(.*) (.*): (.*)")
                .ok()
                .and_then(|re| {
                    re
                        .captures(&s)
                        .and_then(|cap| {
                            parse_capture::<usize>(&cap, 1)
                                .and_then(|first| parse_capture::<usize>(&cap, 2)
                                    .and_then(|second| parse_capture::<String>(&cap, 3)
                                        .and_then(|forth| parse_capture::<String>(&cap, 4)
                                            .map(|fifth| (first, second, forth, fifth)
                                            ))))
                        })
                })
        })
        .collect::<Option<Vec<_>>>()
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    let valid_passwords = input.iter().filter(|(min, max, char, word)| {
        let count = word.matches(char).count();
        count >= min.clone() && count <= max.clone()
    })
        .cloned()
        .collect::<Vec<PreparedInput>>();
    Some(format!("{}", valid_passwords.len()))
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let valid_passwords = input.iter().filter(|(pos1, pos2, char, word)| {
        let c = char.as_bytes()[0];
        match word.as_bytes().get(pos1 - 1).and_then(|a| word.as_bytes().get(pos2 - 1).map(|b| a.clone() == c && b.clone() != c || a.clone() != c && b.clone() == c)) {
            None => false,
            Some(b) => b
        }
    })
        .cloned()
        .collect::<Vec<PreparedInput>>();
    Some(format!("{}", valid_passwords.len()))
}

fn main() {
    AoC::<PreparedInput>::new(2020, 2).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
