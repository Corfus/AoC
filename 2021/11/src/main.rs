use std::collections::HashMap;
use aoc_framework::{AoC, Level};

type PreparedInput = Vec<u8>;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| s.chars().map(|c| c.to_string().parse().ok()).collect::<Option<Vec<u8>>>())
        .collect::<_>()
}

fn enlight_octopuses(octopuses: &mut Vec<PreparedInput>) -> usize {
    let mut enlightened_octopuses: HashMap<(usize, usize), bool> = HashMap::new();
    let f = octopuses.clone().iter().enumerate().fold(0, |s, (row, o_rows)| s + o_rows
        .iter()
        .enumerate()
        .fold(0, |sum, (col, _)| sum + flashes(octopuses, (row, col), &mut enlightened_octopuses)),
    );
    enlightened_octopuses.keys().cloned().for_each(|(row, col)| octopuses[row][col] = 0);
    f
}

fn increase_energy_level(octopuses: &mut Vec<PreparedInput>) {
    *octopuses = octopuses.iter().map(|row| row.iter().map(|o| o + 1).collect::<Vec<_>>()).collect::<Vec<_>>();
}

fn flashes(octopuses: &mut Vec<PreparedInput>, (row, col): (usize, usize), enlightened_octopuses: &mut HashMap<(usize, usize), bool>) -> usize {
    let mut f = 0;
    if octopuses[row][col] > 9 && !enlightened_octopuses.contains_key(&(row, col)) {
        f += 1;
        enlightened_octopuses.insert((row, col), true);
        let min_col = col == 0;
        let min_row = row == 0;
        let max_col = col == octopuses[0].len() - 1;
        let max_row = row == octopuses.len() - 1;
        let mut increase_energy = |r: usize, c: usize| {
            octopuses[r][c] += 1;
            f += flashes(octopuses, (r, c), enlightened_octopuses);
        };

        increase_energy(row, col);

        if !min_col && !min_row {
            increase_energy(row - 1, col - 1);
        }
        if !min_col {
            increase_energy(row, col - 1);
        }
        if !min_col && !max_row {
            increase_energy(row + 1, col - 1);
        }
        if !max_row {
            increase_energy(row + 1, col);
        }
        if !max_col && !max_row {
            increase_energy(row + 1, col + 1);
        }
        if !max_col {
            increase_energy(row, col + 1);
        }
        if !max_col && !min_row {
            increase_energy(row - 1, col + 1);
        }
        if !min_row {
            increase_energy(row - 1, col);
        }
    }
    f
}

pub fn check_full_flash(octopuses: &Vec<PreparedInput>) -> bool {
    octopuses.iter().all(|row| row.iter().all(|e| e == &0))
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    let mut octopuses = input;
    let mut flashes = 0;

    (0..100).for_each(|_| {
        increase_energy_level(&mut octopuses);
        flashes += enlight_octopuses(&mut octopuses);
    });
    Some(flashes.to_string())
}

fn task2(mut octopuses: Vec<PreparedInput>) -> Option<String> {
    (1..1000).find(|_| {
        increase_energy_level(&mut octopuses);
        enlight_octopuses(&mut octopuses);
        check_full_flash(&octopuses)
    })
        .map(|step| step.to_string())
}

fn main() {
    AoC::<PreparedInput>::new(2021, 11).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
