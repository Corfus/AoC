use std::env;
use std::fmt;
use std::str::FromStr;
use reqwest::{header};
use reqwest::blocking::{Client, ClientBuilder};
use scraper::{Html, Selector};
use regex::{Captures};

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Level {
  One,
  Two,
}

impl fmt::Display for Level {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Level::One => write!(f, "1"),
      Level::Two => write!(f, "2"),
    }
  }
}

type Preperation<T> = fn(Vec<String>) -> Option<Vec<T>>;
type Task<T> = fn(Vec<T>) -> Option<String>;

#[derive(Clone)]
pub struct AoC<T = String> {
  year: u16,
  day: u8,
  client: Client,
  preperation: Option<Preperation<T>>,
  task1: Option<Task<T>>,
  task2: Option<Task<T>>,
}

impl<T> AoC<T> {
  pub fn new(year: u16, day: u8) -> Option<AoC<T>> {
    env::var("AOC_SESSION")
      .map_err(|_| {
        panic!("No Environment Variable \"AOC_SESSION\" set");
      })
      .ok()
      .and_then(|session| {
        header::HeaderValue::from_str(&*&format!("session={}", session)).ok()
          .and_then(|header_value| {
            let mut request_headers = header::HeaderMap::new();
            request_headers.insert(
              header::COOKIE,
              header_value,
            );
            ClientBuilder::new()
              .default_headers(request_headers)
              .cookie_store(true)
              .build()
              .ok()
          })
      })
      .map(|client| {
        AoC {
          year,
          day,
          client,
          preperation: None,
          task1: None,
          task2: None,
        }
      })
  }

  pub fn register_tasks(mut self, preperation: Preperation<T>, task1: Task<T>, task2: Task<T>) -> Self {
    self.preperation = Some(preperation);
    self.task1 = Some(task1);
    self.task2 = Some(task2);
    self
  }

  #[deprecated(since = "0.2.0", note = "please use the new interface")]
  pub fn resolve_task<F>(&self, level: Level, handling: F) -> Option<()>
    where F: FnOnce(Vec<String>) -> Option<String> {
    self.get_input_data()
      .and_then(handling)
      .and_then(|result|
        self.send_answer(level, result)
          .and_then(|res| extract_answer_text(res))
          .map(|text| println!("{}", text))
      )
  }

  pub fn resolve(&self, level: Level) -> Option<()> {
    let preperation = self.preperation.as_ref().cloned()?;
    let handling = match level {
      Level::One => self.task1.as_ref().cloned(),
      Level::Two => self.task2.as_ref().cloned(),
    }?;
    self.get_input_data()
      .and_then(preperation)
      .and_then(handling)
      .and_then(|result| {
        println!("Result: {}", result);
        self.send_answer(level, result)
          .and_then(|res| extract_answer_text(res))
          .map(|text| println!("{}", text))
      })
  }

  fn get_input_data(&self) -> Option<Vec<String>> {
    self.client.get(format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day))
      .send()
      .ok()
      .and_then(|res| res.text().ok())
      .map(|mut res| {
        res.remove(res.len() - 1);
        res
      })
      .map(|body| {
        body
          .split("\n")
          .collect::<Vec<&str>>()
          .iter()
          .map(|line| line.to_string())
          .collect::<Vec<String>>()
      })
  }

  fn send_answer(&self, level: Level, answer: String) -> Option<String> {
    let params = [("level", format!("{}", level)), ("answer", answer)];
    self.client.post(format!("https://adventofcode.com/{}/day/{}/answer", self.year, self.day))
      .form(&params)
      .send()
      .ok()
      .and_then(|res| res.text().ok())
  }
}

fn extract_answer_text(html: String) -> Option<String> {
  let document = Html::parse_document(&html);
  Selector::parse("article")
    .ok()
    .and_then(|selector| {
      document.select(&selector).next()
    })
    .map(|article| article.text().map(|s| s.to_string()).collect::<Vec<_>>())
    .map(|v| {
      v.join(" ")
    })
}

pub fn parse_capture<T: FromStr>(capture: &Captures, index: usize) -> Option<T> {
  capture.get(index)
    .and_then(|m| m.as_str().parse::<T>().ok())
}

pub fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
  (0..v[0].len())
    .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
    .collect()
}

pub fn create_2d_vec<T: Clone>(rows: usize, columns: usize, default: T) -> Vec<Vec<T>> {
  vec![vec![default; columns]; rows]
}
