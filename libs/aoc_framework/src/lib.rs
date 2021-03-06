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
    Two
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Level::One => write!(f, "1"),
            Level::Two => write!(f, "2"),
        }
    }
}

#[derive(Clone)]
pub struct AoC {
    year: u16,
    day: u8,
    client: Client,
}

impl AoC {
    pub fn new(year: u16, day: u8) -> Option<Self> {
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
                }
            })
    }

    pub fn get_input_data(&self) -> Option<Vec<String>> {
        self.client.get(format!("https://adventofcode.com/{}/day/{}/input", self.year, self.day))
            .send()
            .ok()
            .and_then(|res| res.text().ok())
            .map(|body| {
                body
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|line| line.to_string())
                    .filter(|line| !line.is_empty())
                    .collect::<Vec<String>>()
            })
    }

    pub fn send_answer(&self, level: Level, answer: String) -> Option<String> {
        let params = [("level", format!("{}", level)), ("answer", answer)];
        self.client.post(format!("https://adventofcode.com/{}/day/{}/answer", self.year, self.day))
            .form(&params)
            .send()
            .ok()
            .and_then(|res| res.text().ok())
    }

    pub fn resolve_task<F>(&self, level: Level, handling: F) -> Option<()>
        where F: FnOnce(Vec<String>) -> Option<String>  {
        self.get_input_data()
            .and_then(handling)
            .and_then(|result|
                self.send_answer(level, result)
                    .and_then(|res| extract_answer_text(res))
                    .map(|text| println!("{}", text))
            )
    }
}

pub fn extract_answer_text(html: String) -> Option<String> {
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
    (0..rows)
        .fold(Vec::new() as Vec<Vec<T>>, |mut v, _| {
            v.push(Vec::new());
            v
        })
        .iter()
        .cloned()
        .map(|mut vec| {
            (0..columns).for_each(|_| vec.push(default.clone()));
            vec
        })
        .collect::<Vec<Vec<T>>>()
}
