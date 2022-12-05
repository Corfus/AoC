use aoc_framework::{AoC, Level};

fn prepare_data(input: Vec<String>) -> Option<Vec<u32>> {
    input
        .iter()
        .cloned()
        .map(|s| s.parse().ok())
        .collect::<_>()
}

fn task1(input: Vec<u32>) -> Option<String> {
    let iter_a = input.iter().cloned();
    let iter_b = iter_a.clone();
    iter_a.clone().find_map(|a| {
        iter_b
            .clone()
            .find(|b| a + b == 2020 )
            .map(|b| {
                let res = a * b;
                println!("a({}) * b({}) = {}", a, b , res);
                format!("{}", res)
            })
    })
}

fn task2(input: Vec<u32>) -> Option<String> {
    let iter_a = input.iter().cloned();
    let iter_b = iter_a.clone();
    let iter_c = iter_a.clone();
    iter_a.clone().find_map(|a| {
        iter_b
            .clone()
            .find_map(|b| {
                iter_c
                    .clone()
                    .find(|c| a + b + c == 2020 )
                    .map(|c| {
                        let res = a * b * c;
                        println!("a({}) * b({}) * c({}) = {}", a, b , c, res);
                        format!("{}", res)
                    })
            })

    })
}

fn main() {
    AoC::<PreparedInput>::new(2020, 1).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
