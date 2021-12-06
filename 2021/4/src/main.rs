use aoc_framework::{AoC, create_2d_vec, Level, transpose};

#[derive(Clone, Debug)]
struct Board {
    board: Vec<Vec<u32>>,
    marked_fields: Vec<Vec<bool>>,
}

fn create_vec<T: Clone>(size: usize, default: T) -> Vec<Vec<T>> {
    create_2d_vec(size, size, default.clone())
}

impl Default for Board {
    fn default() -> Self {
        Board {
            board: create_vec(5, 0),
            marked_fields: create_vec(5, false),
        }
    }
}

impl Board {
    pub fn new(input: &[String]) -> Option<Self> {
        input
            .iter()
            .map(|row| row
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(|n| n.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()
            )
            .collect::<Option<Vec<Vec<u32>>>>()
            .map(|board| {
                Board {
                    board,
                    ..Board::default()
                }
            })
    }

    pub fn check_board(&self) -> bool {
        let check_rows = |v: &Vec<Vec<bool>>| v
            .iter()
            .find(|row| row.iter().cloned().fold(true, |a, b| a && b))
            .is_some();
        check_rows(&self.marked_fields) || check_rows(&transpose(&self.marked_fields))
    }

    pub fn mark(&mut self, value: u32) -> Option<()> {
        self.board
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| row
                .iter()
                .position(|v| *v == value)
                .map(|column_index| (row_index, column_index))
            )
            .map(|(row, column)| {
                self.marked_fields[row][column] = true;
            })
    }

    pub fn sum_of_unmarked(&self) -> u32 {
        self.board
            .iter()
            .enumerate()
            .fold(0, |sum, (row_index, row)| sum + row.iter().enumerate().fold(0, |mut row_sum, (col_index, value)| {
                if !self.marked_fields[row_index][col_index] {
                    row_sum += value;
                }
                row_sum
            }))
    }
}

type PreparedInput = (Vec<u32>, Vec<Board>);

fn prepare_data(input: Vec<String>) -> Option<PreparedInput> {
    let results = input[0].split(",").map(|n| n.parse::<u32>().ok()).collect::<Option<Vec<u32>>>();
    input[1..]
        .chunks(5)
        .map(|chunk| Board::new(chunk))
        .collect::<Option<Vec<Board>>>()
        .and_then(|board| results.map(|r| (r, board)))
}

fn task1(input: PreparedInput) -> Option<String> {
    let (inp, mut boards) = input;

    inp
        .iter()
        .cloned()
        .find_map(|number| {
            boards = boards.iter().cloned().map(|mut b| {
                b.mark(number);
                b
            }).collect::<Vec<_>>();
            boards.iter().cloned().find(|b| b.check_board()).map(|board| (board, number))
        })
        .map(|(board, number)| board.sum_of_unmarked() * number)
        .map(|value| value.to_string())
}

fn task2(input: PreparedInput) -> Option<String> {
    let (inp, boards) = input;

    boards
        .iter()
        .cloned()
        .map(|mut board| inp.iter().cloned().enumerate().find_map(|(index, number)| {
            board.mark(number);
            if board.check_board() { Some((index, number, board.clone())) } else { None }
        }))
        .collect::<Option<Vec<(usize, u32, Board)>>>()
        .and_then(|list| list.iter().reduce(|a, b| {
            if a.clone().0 > b.clone().0 {
                a
            } else {
                b
            }
        })
            .map(|(_, number, board)| board.sum_of_unmarked() * number)
            .map(|value| value.to_string())
        )
}

fn main() {
    AoC::new(2021, 4).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
