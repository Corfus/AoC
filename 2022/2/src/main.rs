use std::cmp::Ordering;
use std::ops::Add;
use aoc_framework::{AoC, Level};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Sign {
  Rock,
  Paper,
  Scissor,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Ending {
  Lose,
  Draw,
  Win,
}

impl From<&Sign> for i32 {
  fn from(s: &Sign) -> Self {
    match s {
      Sign::Rock => 1,
      Sign::Paper => 2,
      Sign::Scissor => 3
    }
  }
}

impl From<i32> for Sign {
  fn from(value: i32) -> Self {
    let clean_value = ((value - 1) % 3) + 1;
    match clean_value {
      1 => Sign::Rock,
      2 => Sign::Paper,
      _ => Sign::Scissor,
    }
  }
}

impl Add<i32> for Sign {
  type Output = Sign;

  fn add(self, rhs: i32) -> Self::Output {
    let shift = ((rhs - 1) % 3) + 1;
    (i32::from(&self) + shift).into()
  }
}

impl TryFrom<char> for Sign {
  type Error = ();

  fn try_from(value: char) -> Result<Self, Self::Error> {
    if value == 'A' || value == 'X' {
      Ok(Sign::Rock)
    } else if value == 'B' || value == 'Y' {
      Ok(Sign::Paper)
    } else if value == 'C' || value == 'Z' {
      Ok(Sign::Scissor)
    } else {
      Err(())
    }
  }
}

impl TryFrom<char> for Ending {
  type Error = ();

  fn try_from(value: char) -> Result<Self, Self::Error> {
    if value == 'X' {
      Ok(Ending::Lose)
    } else if value == 'Y' {
      Ok(Ending::Draw)
    } else if value == 'Z' {
      Ok(Ending::Win)
    } else {
      Err(())
    }
  }
}

impl PartialOrd for Sign {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other {
      Some(Ordering::Equal)
    } else if self == &Sign::Paper && other == &Sign::Rock {
      Some(Ordering::Greater)
    } else if self == &Sign::Paper && other == &Sign::Scissor {
      Some(Ordering::Less)
    } else if self == &Sign::Rock && other == &Sign::Scissor {
      Some(Ordering::Greater)
    } else {
      other.partial_cmp(self).map(|result| {
        if result == Ordering::Greater {
          Ordering::Less
        } else {
          Ordering::Greater
        }
      })
    }
  }
}

impl Sign {
  fn fight(&self, competitor: &Sign) -> i32 {
    if self == competitor {
      3 + i32::from(self)
    } else if self > competitor {
      6 + i32::from(self)
    } else {
      i32::from(self)
    }
  }

  fn get_sign_for_ending(&self, ending: Ending) -> Self {
    match ending {
      Ending::Lose => *self + 2,
      Ending::Draw => self.clone(),
      Ending::Win => *self + 1,
    }
  }

  fn convert_to_signs(s: &str) -> Option<(Sign, Sign)> {
    let chars = s.chars().collect::<Vec<char>>();
    let c = chars.get(0).cloned().and_then(|char| Sign::try_from(char).ok())?;
    let m = chars.get(2).cloned().and_then(|char| Sign::try_from(char).ok())?;
    Some((c, m))
  }

  fn convert_to_sign_and_ending(s: &str) -> Option<(Sign, Ending)> {
    let chars = s.chars().collect::<Vec<char>>();
    let c = chars.get(0).cloned().and_then(|char| Sign::try_from(char).ok())?;
    let e = chars.get(2).cloned().and_then(|char| Ending::try_from(char).ok())?;
    Some((c, e))
  }
}

type PreparedInput = String;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
  Some(input)
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
  input
    .iter()
    .cloned()
    .map(|s| Sign::convert_to_signs(&s))
    .collect::<Option<Vec<(Sign, Sign)>>>()
    .map(|signs| {
      signs
        .iter()
        .fold(0, |score, (c, m)| {
          score + m.fight(c)
        }).to_string()
    })
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
  input
    .iter()
    .cloned()
    .map(|s| Sign::convert_to_sign_and_ending(&s))
    .collect::<Option<Vec<(Sign, Ending)>>>()
    .map(|signs| {
      signs
        .iter()
        .fold(0, |score, (c, e)| {
          let m = c.get_sign_for_ending(*e);
          score + m.fight(c)
        }).to_string()
    })
}

fn main() {
  AoC::new(2022, 2).map(|aoc| {
    aoc.resolve_task(Level::Two, |input| {
      prepare_data(input)
        .and_then(task2)
    })
  });
}
