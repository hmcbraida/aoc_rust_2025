use std::fs;

struct InputData {
    width: usize,
    height: usize,
    roll_matrix: Vec<Vec<bool>>,
}

impl InputData {
    fn within_bnd(&self, c: (i32, i32)) -> bool {
        let (x, y) = c;
        if x >= self.width as i32 || x < 0 {
            return false;
        }

        if y >= self.height as i32 || y < 0 {
            return false;
        }

        return true;
    }

    fn get_surrounds(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;

        let deltas = vec![
            (-1, 0),
            (1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        deltas
            .iter()
            .map(|delta| {
                let (dx, dy) = delta;
                ((x + *dx), (y + *dy))
            })
            .filter(|c| self.within_bnd(*c))
            .map(|c| {
                let (x, y) = c;
                (x as usize, y as usize)
            })
            .collect()
    }
}

fn parse_input(input_path: &str) -> InputData {
    let roll_matrix: Vec<Vec<bool>> = fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .filter(|x| return x.len() > 0)
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '@' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    let width = roll_matrix[0].len();
    let height = roll_matrix.len();

    InputData {
        width,
        height,
        roll_matrix,
    }
}

fn construct_adjacency(data: &InputData) -> Vec<Vec<u8>> {
    let mut adjacency: Vec<Vec<u8>> = data
        .roll_matrix
        .iter()
        .map(|row| row.iter().map(|_| 0).collect())
        .collect();

    for (x, row) in data.roll_matrix.iter().enumerate() {
        for (y, maybe_roll) in row.iter().enumerate() {
            if *maybe_roll {
                for adj in data.get_surrounds(x, y) {
                    let (other_x, other_y) = adj;

                    adjacency[other_x][other_y] += 1;
                }
            }
        }
    }

    adjacency
}

fn check_can_remove(input_data: &InputData, adjacency: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let can_remove = adjacency
        .iter()
        .zip(input_data.roll_matrix.iter())
        .map(|row| {
            let (adj_row, input_row) = row;
            adj_row
                .iter()
                .zip(input_row)
                .map(|x| {
                    let (n_adj, is_roll) = x;
                    if *is_roll && *n_adj < 4 { true } else { false }
                })
                .collect()
            // let (adj_count, is_roll) = x;
            // (**adj_count < 4) && **is_roll
        })
        .collect();

    can_remove
}

pub fn go(input_path: &str) -> String {
    let mut input_data = parse_input(input_path);
    let mut adjacency = construct_adjacency(&input_data);

    let mut removable: Vec<Vec<bool>>;
    let mut n_removable_initially = 0;
    let mut total_n_removable = 0;

    loop {
        removable = check_can_remove(&input_data, &adjacency);
        let n_removable = removable.iter().flatten().filter(|x| **x).count();
        if n_removable == 0 {
            break;
        }
        if n_removable_initially == 0 {
            n_removable_initially = n_removable;
        }
        total_n_removable += n_removable;
        input_data.roll_matrix = input_data
            .roll_matrix
            .iter()
            .zip(removable.iter())
            .map(|(input_row, remove_row)| {
                input_row
                    .iter()
                    .zip(remove_row.iter())
                    .map(|(is_roll, is_removable)| *is_roll && !*is_removable)
                    .collect()
            })
            .collect();
        adjacency = construct_adjacency(&input_data);
    }

    format!(
        "Initial number removable: {}.\nAfter iterated removals, {}",
        n_removable_initially, total_n_removable
    )
}
