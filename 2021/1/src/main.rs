use aoc_framework::{AoC, Level};

type PreparedInput = u32;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| s.parse().ok())
        .collect::<_>()
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    Some(input.windows(2).filter(|window| window[0] < window[1]).count().to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    input
        .windows(3)
        .map(|window| window.iter().cloned().reduce(|a, b| a + b))
        .collect::<Option<Vec<u32>>>()
        .and_then(|avg| task1(avg))
}

fn main() {
    AoC::<PreparedInput>::new(2021, 1).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
