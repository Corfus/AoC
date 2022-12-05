use aoc_framework::{AoC, Level};

#[derive(Clone, Debug)]
struct SignalSystem {
    output: Vec<String>,
    translation: Vec<String>,
}

impl SignalSystem {
    pub fn new(digits: Vec<String>, output: Vec<String>) -> Option<Self> {
        Self::get_translation_table(&mut digits.clone())
            .map(|translation| SignalSystem {
                output,
                translation,
            })
    }

    pub fn translated_digits(&self) -> Option<Vec<usize>> {
        self.output.iter()
            .map(|digit| self.translation.iter().position(|d| d == digit))
            .filter(|opt| opt.is_some())
            .collect::<Option<Vec<usize>>>()
    }

    pub fn translate(&self) -> Option<usize> {
        self.translated_digits()
            .and_then(|list| list.iter().fold(String::new(), |str, d| str + &*d.to_string()).parse::<usize>().ok())
    }

    fn get_translation_table(digits: &mut Vec<String>) -> Option<Vec<String>> {
        let mut translation = vec!["".to_string(); 10];
        Self::get_number_through_length(digits, &mut translation, 2, 1)
            .and_then(|_| Self::get_number_through_length(digits, &mut translation, 3, 7))
            .and_then(|_| Self::get_number_through_length(digits, &mut translation, 4, 4))
            .and_then(|_| Self::get_number_through_length(digits, &mut translation, 7, 8))
            .and_then(|_| {
                let patterns = [translation.clone()[4].clone() + &*translation[7], translation[7].clone()];
                Self::get_number_through_matching(digits, &mut translation, 6, [9, 0, 6], patterns)
            })
            .and_then(|_| {
                let patterns = [translation.clone()[7].clone() + &*translation[7], Self::remove_string_from(&translation[8], &translation[9])];
                Self::get_number_through_matching(digits, &mut translation, 5, [3, 2, 5], patterns)
            })
            .map(|_| translation)
    }

    fn get_number_through_length(digits: &mut Vec<String>, translation: &mut Vec<String>, length: usize, digit: usize) -> Option<()> {
        digits.iter().cloned().position(|d| d.len() == length)
            .map(|index| translation[digit] = digits.remove(index))
    }

    fn get_number_through_matching(digits: &mut Vec<String>, translation: &mut Vec<String>, length: usize, digit_resolution: [usize; 3], patterns: [String; 2]) -> Option<()> {
        let mut p = patterns.iter().cloned().collect::<Vec<String>>();
        p.push(String::new());

        digit_resolution
            .iter()
            .enumerate()
            .map(|(i, digit)| {
                Self::find_overlapped_string(&Self::get_digits_with_length(digits, length), &Self::string_to_unique_char_list(&p[i]))
                    .map(|(index, _)| {
                        translation[digit_resolution[i]] = digits.remove(index);
                    })
            })
            .collect::<Option<Vec<()>>>()
            .map(|_| ())
    }

    fn get_digits_with_length(digits: &Vec<String>, length: usize) -> Vec<(usize, String)> {
        digits.iter().cloned().enumerate().filter(|(_, d)| d.len() == length).collect()
    }

    fn string_to_unique_char_list(s: &String) -> Vec<char> {
        let mut almost = s.chars().collect::<Vec<char>>();
        almost.sort();
        almost.dedup();
        almost
    }

    fn find_overlapped_string(list: &Vec<(usize, String)>, chars: &Vec<char>) -> Option<(usize, String)> {
        list.iter()
            .cloned()
            .find(|(_, s)| chars.iter().fold(true, |state, c| state && s.contains(&c.to_string())))
    }
    fn remove_string_from(origin: &String, diff: &String) -> String {
        String::from_iter(origin.chars().filter(|o| diff.chars().find(|d| o == d).is_none()).collect::<Vec<char>>())
    }
}

type PreparedInput = SignalSystem;

fn sort_string(s: &String) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();
    String::from_iter(chars)
}

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| {
            let res = s.split(" | ").map(|digits| digits.split(" ").map(|d| sort_string(&d.to_string())).collect()).collect::<Vec<Vec<String>>>();
            res.get(0).and_then(|digits| res.get(1).and_then(|output| SignalSystem::new(digits.clone(), output.clone())))
        })
        .collect::<Option<Vec<PreparedInput>>>()
}

fn count_digits(haystack: &Vec<SignalSystem>, needles: &Vec<usize>) -> Option<usize> {
    haystack
        .iter()
        .map(|system| system
            .translated_digits()
            .map(|output| output
                .iter()
                .cloned()
                .filter(|d| needles.contains(d))
                .collect::<Vec<usize>>().len()
            )
        ).collect::<Option<Vec<usize>>>()
        .map(|list| list.iter().fold(0, |sum, c| sum + c))
}

fn count_numbers(haystack: &Vec<SignalSystem>) -> Option<usize> {
    haystack
        .iter()
        .map(|system| system.translate())
        .collect::<Option<Vec<usize>>>()
        .and_then(|list| list.iter().cloned().reduce(|a, b| a + b))
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    count_digits(&input, &vec![1, 4, 7, 8])
        .map(|n| n.to_string())
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    count_numbers(&input)
        .map(|result| result.to_string())
}

fn main() {
    AoC::<PreparedInput>::new(2021, 8).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
