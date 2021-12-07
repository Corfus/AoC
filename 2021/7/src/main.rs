use aoc_framework::{AoC, Level};

type PreparedInput = i32;

fn prepare_data(input: Vec<String>) -> Option<Vec<i32>> {
    input
        .first()
        .and_then(|s| s.split(",").map(|n| n.parse::<i32>().ok()).collect::<Option<Vec<i32>>>())
}

fn get_median(input: &Vec<PreparedInput>) -> i32 {
    let mut list = input.clone();
    list.sort();
    let center = list.len() / 2;
    list[center]
}

fn get_mean(input: &Vec<PreparedInput>) -> i32 {
    input.iter().sum::<i32>() / input.len() as i32
}


fn task1(input: Vec<PreparedInput>) -> Option<String> {
    let mean = get_median(&input);
    Some(input.iter().fold(0, |sum, pos| sum + (mean - pos).abs())
        .to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let mean = get_mean(&input);
    (mean - 10 .. mean + 10).map(|m| {
        input.iter().fold(0, |sum, pos| {
            let distance =  ((m - pos) as i32).abs();
            let fuel: i32 = (1..distance + 1).sum();
            sum + fuel
        })
    })
        .reduce(|a, b| if a < b { a } else {b})
        .map(|fuel| fuel.to_string())
}

fn main() {
    AoC::new(2021, 7).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
