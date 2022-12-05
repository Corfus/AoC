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
    None
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    None
}

fn main() {
    AoC::<PreparedInput>::new({{ year }}, {{ day }})
      .and_then(|aoc| aoc.register_tasks(prepare_data, task1, task2).resolve(Level::One));
}
