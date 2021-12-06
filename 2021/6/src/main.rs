use aoc_framework::{AoC, Level};

type PreparedInput = Shoal;

#[derive(Copy, Clone, Debug)]
struct Shoal {
    timer: u8,
    quantity: usize,
}

impl Shoal {
    pub fn new(timer: u8, quantity: usize) -> Self {
        Shoal {
            timer,
            quantity,
        }
    }

    pub fn grow(&mut self) -> usize {
        if self.timer == 0 {
            self.timer = 6;
            self.quantity
        } else {
            self.timer -= 1;
            0
        }
    }

    pub fn add_fishes(&mut self, q: usize) {
        self.quantity += q;
    }
}

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .first()
        .and_then(|s| s.split(",").map(|n| n.parse::<usize>().ok()).collect::<Option<Vec<usize>>>())
        .map(|mut list| {
            list.sort();
            let mut shouls: Vec<Shoal> = (0..9).map(|timer| Shoal::new(timer, 0)).collect();
            list.iter().for_each(|timer| {
                shouls[*timer].add_fishes(1);
            });
            shouls
        })
}

fn get_fishes_quantity(shoals: &Vec<Shoal>) -> usize {
    shoals.iter().fold(0, |sum, shoal| sum + shoal.quantity)
}

fn fish_growth(mut shoals: Vec<PreparedInput>, days: usize) -> Option<String> {
    (0..days).fold(Some("".to_string()), |_, __| {
        let mut children = 0;
        shoals = shoals
            .iter()
            .cloned()
            .map(|mut shoal| {
                children += shoal.grow();
                shoal
            })
            .collect();

        (0..8)
            .map(|timer| {
                let shs = shoals.clone().iter().cloned().filter(|sh| sh.timer == timer).collect::<Vec<_>>();
                if shs.len() > 1 {
                    shs.iter().cloned().reduce(|a, b| Shoal::new(timer, a.quantity + b.quantity))
                } else {
                    Some(shs.get(0).cloned().unwrap_or(Shoal::new(timer, 0)))
                }
            })
            .collect::<Option<Vec<_>>>()
            .map(|list| {
                shoals = list.clone();
                if children > 0 {
                    shoals.push(Shoal::new(8, children));
                }
                get_fishes_quantity(&shoals).to_string()
            })
    })
}

fn task1(mut shoals: Vec<PreparedInput>) -> Option<String> {
    fish_growth(shoals, 80)
}

fn task2(mut shoals: Vec<PreparedInput>) -> Option<String> {
    fish_growth(shoals, 256)
}

fn main() {
    AoC::new(2021, 6).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
