use aoc_framework::{AoC, Level};

type PreparedInput = String;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| s.parse().ok())
        .collect::<_>()
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    Some("".to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    Some("".to_string())
}

fn main() {
    AoC::<PreparedInput>::new({{ year }}, {{ day }}).map(|aoc| {
        aoc.resolve_task(Level::One, |input| {
            prepare_data(input)
                .and_then(task1)
        })
    });
}
