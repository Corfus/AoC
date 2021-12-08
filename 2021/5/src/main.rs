use aoc_framework::{AoC, create_2d_vec, Level, parse_capture};
use regex::Regex;

struct Map {
    map: Vec<Vec<isize>>,
}

impl Map {
    pub fn new(max: (isize, isize)) -> Self {
        let map = create_2d_vec((max.1 + 1) as usize, (max.0 + 1) as usize, 0);
        Map {
            map
        }
    }

    pub fn count(&self) -> isize {
        self.map.iter().fold(0, |sum, row| sum + row.iter().fold(0, |a, b| if *b >= 2 { a + 1 } else { a }))
    }

    pub fn set_line(&mut self, line: &PreparedInput) {
        let x_diff = line.2 - line.0;
        let y_diff = line.3 - line.1;
        let diff = (if x_diff.abs() > y_diff.abs() { x_diff } else { y_diff }).abs() + 1;

        let mut set_point = |x: isize, y: isize| {
            self.map[(line.1 + y) as usize][(line.0 + x) as usize] += 1;
        };

        let x_sign = x_diff.signum();
        let y_sign = y_diff.signum();

        (0..diff).for_each(|d| {
            set_point(x_sign * d, y_sign * d);
        });
    }
}


type PreparedInput = (isize, isize, isize, isize);

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| Regex::new("(.*),(.*) -> (.*),(.*)")
            .ok()
            .and_then(|re|
                re
                    .captures(&s)
                    .and_then(|cap|
                        parse_capture::<isize>(&cap, 1)
                            .and_then(|x1| parse_capture::<isize>(&cap, 2)
                                .and_then(|x2| parse_capture::<isize>(&cap, 3)
                                    .and_then(|y1| parse_capture::<isize>(&cap, 4)
                                        .map(|y2| (x1, x2, y1, y2))
                                    )))
                    )
            ))
        .collect::<_>()
}

fn get_max(input: &Vec<PreparedInput>) -> (isize, isize) {
    input
        .iter()
        .cloned()
        .fold((0, 0), |(old_x, old_y), b| {
            let x = if b.0 > old_x { b.0 } else if b.2 > old_x { b.2 } else { old_x };
            let y = if b.1 > old_y { b.1 } else if b.3 > old_y { b.3 } else { old_y };
            (x, y)
        })
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    let inp = input
        .iter()
        .cloned()
        .filter(|line| {
            let l = line.clone();
            l.0 == l.2 || l.1 == l.3
        })
        .collect::<Vec<_>>();
    let mut map = Map::new(get_max(&inp));

    inp
        .iter()
        .for_each(|line| map.set_line(line));

    Some(map.count().to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let mut map = Map::new(get_max(&input));

    input
        .iter()
        .for_each(|line| map.set_line(line));
    Some(map.count().to_string())
}

fn main() {
    AoC::new(2021, 5).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
