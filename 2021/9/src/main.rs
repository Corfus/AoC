use aoc_framework::{AoC, Level};

type PreparedInput = Vec<u32>;

fn prepare_data(input: Vec<String>) -> Option<Vec<PreparedInput>> {
    input
        .iter()
        .cloned()
        .map(|s| s.chars().map(|c| c.to_string().parse::<u32>().ok()).collect::<Option<Vec<u32>>>())
        .collect::<_>()
}

fn get_lowest_points(input: &Vec<PreparedInput>) -> Vec<(usize, usize)> {
    input.iter().cloned().enumerate().fold(Vec::new(),|mut vec, (row_index, row)| {
        let mut r = row
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(col_index, height)| {
                let r = row_index.clone();
                let c = col_index.clone();

                if (r == 0 || input[r - 1][c] > height) &&
                    (r == input.len() - 1 || input[r + 1][c] > height) &&
                    (c == 0 || input[r][c - 1] > height) &&
                    (c == input[0].len() - 1 || input[r][c + 1] > height) {
                    Some((row_index, col_index))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, usize)>>();
        vec.append(&mut r);
        vec
    })
}

fn task1(input: Vec<PreparedInput>) -> Option<String> {
    let points = get_lowest_points(&input)
        .iter()
        .fold(0, |sum: u32, (row, col)| input[row.clone()][col.clone()] + 1 + sum);
    Some(points.to_string())
}

fn grow_basin((row, col): (usize, usize), map: &Vec<PreparedInput>, basin: &mut Vec<(usize, usize)>) {
    if !basin.contains(&(row, col)) {
        basin.push((row, col));

        let mut grow = |r: usize, c: usize| {
            if  map[r][c] < 9 {
                grow_basin((r, c), map, basin);
            }
        };

        if row as u32 != 0 {
            grow(row - 1, col);
        }
        if row as usize != map.len() - 1 {
            grow(row + 1, col);
        }
        if col as u32 != 0 {
            grow(row, col - 1);
        }
        if col as usize != map[0].len() - 1 {
            grow(row, col + 1);
        }
    }
}

fn task2(input: Vec<PreparedInput>) -> Option<String> {
    let mut basin_sizes = get_lowest_points(&input)
        .iter()
        .cloned()
        .map(|(row, col)| {
            let mut basin = vec![];
            grow_basin((row, col), &input, &mut basin);
            basin
        })
        .map(|basin| basin.len())
        .collect::<Vec<usize>>();

    basin_sizes.sort();
    basin_sizes.reverse();
    Some((basin_sizes[0] * basin_sizes[1] * basin_sizes[2]).to_string())
}

fn main() {
    AoC::<PreparedInput>::new(2021, 9).map(|aoc| {
        aoc.resolve_task(Level::Two, |input| {
            prepare_data(input)
                .and_then(task2)
        })
    });
}
