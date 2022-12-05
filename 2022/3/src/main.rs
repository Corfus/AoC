use aoc_framework::{AoC, Level};

type PreparedInput = String;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    Some(input)
}

fn char_to_value(char: char) -> u32 {
  let v = char as u32;
  if char.is_uppercase() {
    v - 38
  } else {
    v - 96
  }
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    input
      .iter()
      .cloned()
      .map(|s| {
        let l = s.len();
        let (a, b) = s.split_at(l / 2).into();
        (String::from(a), String::from(b))
      })
      .map(|(a, b)| {
          let chars = a.chars().collect::<Vec<char>>();
          b.chars().find(|c| chars.contains(c))
            .map(|c| char_to_value(c))
      })
      .collect::<Option<Vec<u32>>>()
      .map(|list| list.iter().sum::<u32>().to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
  input
    .chunks(3)
    .map(|chunks| {
      let char_chunks = chunks.iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<_>>();
      let a = char_chunks[0].clone();
      let b = chunks[1].clone();
      let c = chunks[2].clone();
      c.chars().find(|ch| a.contains(ch) && b.contains(*ch))
        .map(|c| char_to_value(c))
    })
    .collect::<Option<Vec<u32>>>()
    .map(|list| list.iter().sum::<u32>().to_string())
}

fn main() {
    AoC::<PreparedInput>::new(2022, 3).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
