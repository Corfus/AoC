use std::ops::{AddAssign};
use std::str::FromStr;
use aoc_framework::{AoC, Level};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Symbol {
    Zero,
    One,
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Symbol::Zero),
            "1" => Ok(Symbol::One),
            &_ => Err(())
        }
    }
}

impl AddAssign<Symbol> for String {
    fn add_assign(&mut self, rhs: Symbol) {
        self.push(match rhs {
            Symbol::Zero => '0',
            Symbol::One => '1'
        });
    }
}

impl Symbol {
    pub fn invert(&self) -> Symbol {
        match self {
            Symbol::Zero => Symbol::One,
            Symbol::One => Symbol::Zero
        }
    }
}

type PreparedInput = Vec<Symbol>;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| s.clone().chars().map(|c| c.to_string().parse::<Symbol>().ok()).collect::<_>())
        .collect::<_>()
}

fn get_inner_len<T>(v: &Vec<Vec<T>>) -> usize {
    v.first().unwrap_or(&(Vec::new() as Vec<T>)).len()
}

fn get_parent_vec(v: &Vec<PreparedInput>) -> Vec<Vec<Symbol>> {
    (0..get_inner_len(&v)).map(|_| Vec::new()).collect::<Vec<Vec<Symbol>>>()
}

fn invert_vec_logic(input: &Vec<PreparedInput>) -> Vec<PreparedInput> {
    input
        .iter()
        .fold(get_parent_vec(&input),
              |mut a, b| {
                  b.iter().enumerate().for_each(|(index, c)| a[index].push(c.clone()));
                  a
              })
}

fn get_most_common_symbol(row: &PreparedInput) -> Symbol {
    match row.iter().cloned().filter(|s| *s == Symbol::Zero).count() > row.len() / 2 {
        true => Symbol::Zero,
        false => Symbol::One
    }
}

fn calc_check_sum(numbers: &Vec<String>) -> Option<String> {
    numbers
        .iter()
        .map(|list| u32::from_str_radix(list, 2).ok())
        .collect::<Option<Vec<u32>>>()
        .map(|list| list.iter().fold(1, |a, b| a * b))
        .map(|v| v.to_string())
}

fn criteria_filter(list: &Vec<PreparedInput>, pos: usize, invert: bool) -> Option<Vec<PreparedInput>> {
    let symbol_list = invert_vec_logic(list);

    let raw_symbol = get_most_common_symbol(&symbol_list[pos]);
    let symbol = if invert { raw_symbol.invert() } else { raw_symbol };

    let filtered_list = list
        .iter()
        .cloned()
        .filter(|number| number[pos] == symbol)
        .collect::<Vec<_>>();
    if filtered_list.len() <= 1 {
        if filtered_list.len() == 1 {
            Some(filtered_list)
        } else {
            None
        }
    } else {
        criteria_filter(&filtered_list, pos + 1, invert)
    }
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    calc_check_sum(
        &invert_vec_logic(&input)
            .iter()
            .map(get_most_common_symbol)
            .fold(vec!["".to_string(), "".to_string()], |mut list, symbol| {
                list[0] += symbol;
                list[1] += symbol.invert();
                list
            })
    )
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let oxygen_list = criteria_filter(&input, 0, false);
    let co2 = criteria_filter(&input, 0, true);

    oxygen_list.and_then(|o| co2.map(|c| vec![o[0].clone(), c[0].clone()]))
        .map(|sensors| sensors
            .iter()
            .cloned()
            .map(|number| number.iter().fold("".to_string(), |mut a, b| {
                a += *b;
                a
            }))
            .collect::<Vec<String>>()
            )
        .and_then(|a| calc_check_sum(&a))
}

fn main() {
    AoC::new(2021, 3).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
