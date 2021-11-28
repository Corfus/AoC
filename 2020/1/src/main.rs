use aoc_framework::{AoC, extract_answer_text};

fn main() {
    AoC::new(2020, 1).map(|aoc| {
        aoc.get_input_data()
            .map(|body| {
                let lines = body.split("\n");
                lines.collect::<Vec<&str>>()
                    .iter()
                    .map(|line| line.to_string())
                    .filter(|line| !line.is_empty())
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<u32>>()
            })
            .and_then(|list| {
                let iter_a = list.iter().cloned();
                let iter_b = iter_a.clone();
                iter_a.clone().find_map(|a| {
                    iter_b
                        .clone()
                        .find(|b| a + b == 2020 )
                        .map(|b| {
                            let res = a * b;
                            println!("a({}) * b({}) = {}", a, b , res);
                            res
                        })
                })
            })
            .and_then(|result|
                aoc.send_answer(1, format!("{}", result))
                    .and_then(|res| extract_answer_text(res))
                    .map(|text| println!("{}", text))
            )
    });
}
