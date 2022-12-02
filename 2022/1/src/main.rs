use aoc_framework::{AoC, Level};

type PreparedInput = Vec<i32>;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
  input.iter()
    .fold(vec![vec![]] as Vec<Vec<Option<i32>>>, | mut parent, value| {
      if value == "" {
        parent.push(vec![]);
      } else {
        let int = value.clone().parse().ok()
          .and_then(|v: String| v.parse().ok());
        parent.last_mut().map(|vec| vec.push(int));
      }
      parent
    })
    .iter()
    .cloned()
    .map(|vec| vec.iter().cloned().collect::<Option<PreparedInput>>())
    .collect::<Option<Vec<PreparedInput>>>()
}

fn collect_calories(elves: &Vec<PreparedInput>) -> Vec<i32> {
  elves.iter().map(|cals| cals.iter().fold(0, | amount, cal| amount + cal)).collect::<_>()
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
  let max_calories = collect_calories(&input).iter().fold(0, | max, cal| {
    std::cmp::max(max, *cal)
  });
  Some(max_calories.to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
  let mut elves = collect_calories(&input);
  elves.sort();
  elves.reverse();
  Some((elves[0] + elves[1] + elves[2]).to_string())
}

fn main() {
  AoC::new(2022, 1).map(|aoc| {
    aoc.resolve_task(Level::Two, |input| {
      prepare_data(input)
        .and_then(task2)
    })
  });
}
