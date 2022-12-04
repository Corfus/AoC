use aoc_framework::{AoC, Level};

type PreparedInput = ((u32, u32), (u32, u32));

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
      .iter()
      .map(|s| split_at_char(s, ','))
      .map(|(a, b)| Some((get_edges(&a)?, get_edges(&b)?)))
      .collect::<Option<Vec<_>>>()
}

fn get_edges(span: &String) -> Option<(u32, u32)> {
    let (a, b) = split_at_char(span, '-');
    let start = a.parse().ok()?;
    let end = b.parse().ok()?;
    Some((start, end))
}

fn is_inside(a: (u32, u32), b: (u32, u32)) -> bool {
    let a_end = a.1 + 1;
    let b_end = b.1 + 1;
    (a.0..a_end).contains(&b.0) && (a.0..a_end).contains(&b.1) || (b.0..b_end).contains(&a.0) && (b.0..b_end).contains(&a.1)
}

fn is_inside_or_at_edge(a: (u32, u32), b: (u32, u32)) -> bool {
    !(a.1 < b.0 || b.1 < a.0)
}

fn split_at_char(haystack: &String, needle: char) -> (String, String) {
    let pos = haystack.chars().position(|c| c == needle).unwrap_or(0);
    let (a, b) = haystack.split_at(pos);
    (String::from(a), String::from(b)[1..].to_string())
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    Some(input
      .iter()
      .map(|(a, b)| is_inside(a.clone(), b.clone()))
      .fold(0, |sum, v| sum + (v as u32))
      .to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    Some(input
      .iter()
      .map(|(a, b)| is_inside_or_at_edge(a.clone(), b.clone()))
      .fold(0, |sum, v| sum + (v as u32))
      .to_string())
}

fn main() {
    AoC::new(2022, 4).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
