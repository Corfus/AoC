use aoc_framework::{AoC, Level, transpose};
use regex::Regex;

type PreparedInput = (Vec<Vec<char>>, Vec<Command>);

#[derive(Copy, Clone, Debug)]
struct Command {
  amount: u32,
  origin: u8,
  target: u8,
}

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
  let pos = input.iter().position(|c| c.len() == 0)?;
  let (a, b) = input.split_at(pos);
  let mut raw_commands = b.to_vec();
  raw_commands.remove(0);
  Some(vec![(get_stacks(&a.to_vec()), get_commands(&raw_commands)?)])
}

fn get_stacks(input: &Vec<String>) -> Vec<Vec<char>> {
  let mut chars = input
    .iter()
    .map(|line| line.chars().collect::<Vec<char>>())
    .map(|line| {
      let mut stcks: Vec<char> = vec![];
      for i in 0..(line.len() + 1) / 4 {
        stcks.push(line[i * 4 + 1])
      }
      stcks
    })
    .collect::<Vec<Vec<char>>>();
  chars.remove(chars.len() - 1);
  transpose(&chars)
    .iter()
    .map(|stack| stack.iter().fold(String::new(), |mut s, c| {
      s.push(*c);
      s
    }))
    .map(|stack| {
      stack.trim().chars().collect::<Vec<char>>()
    })
    .collect::<Vec<Vec<_>>>()
}

fn get_commands(input: &Vec<String>) -> Option<Vec<Command>> {
  let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").ok()?;
  input
    .iter()
    .map(|s| {
      re.captures(s)
        .and_then(|cap| {
          Some(Command {
            amount: cap.get(1)?.as_str().parse().ok()?,
            origin: cap.get(2)?.as_str().parse().ok().map(|v: u8| v - 1)?,
            target: cap.get(3)?.as_str().parse().ok().map(|v: u8| v - 1)?,
          })
        })
    })
    .collect::<Option<Vec<Command>>>()
}

fn extract(input: &Vec<PreparedInput>) -> PreparedInput {
  input[0].clone()
}

fn manipulate(stack: &mut Vec<Vec<char>>, command: &Command, reverse: bool) {
  let origin_idx = command.origin as usize;
  let target_idx = command.target as usize;
  let amount = command.amount as usize;

  let origin = stack[origin_idx][amount..].to_vec();
  let mut diff = stack[origin_idx][0..amount].to_vec();
  if reverse {
    diff.reverse();
  }
  let mut target = stack[target_idx].clone();
  target.splice(0..0,diff);

  stack[origin_idx] = origin;
  stack[target_idx] = target;
}

fn get_result(stack: &Vec<Vec<char>>) -> String {
  stack
    .iter()
    .map(|s| s[0].clone())
    .fold(String::new(), |mut s, c| {
      s.push(c);
      s
    })
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
  let (mut stack, commands) = extract(&input);
  commands
    .iter()
    .for_each(|command| manipulate(&mut stack, command, true));
  Some(get_result(&stack))
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
  let (mut stack, commands) = extract(&input);
  commands
    .iter()
    .for_each(|command| manipulate(&mut stack, command, false));
  Some(get_result(&stack))
}

fn main() {
  AoC::<PreparedInput>::new(2022, 5)
    .and_then(|aoc| aoc.register_tasks(prepare_data, task1, task2).resolve(Level::Two));
}
