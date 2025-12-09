use std::fs;

struct InputData {
    width: usize,
    height: usize,
    roll_matrix: Vec<Vec<bool>>,
}

impl InputData {
    /// Checks whether a point c is within bounds.
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

    /// Get the squares surrounding coords x and y
    fn get_surrounds(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        // Allow them to be negative temporarily
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
                // move in each possible direction
                let (dx, dy) = delta;
                ((x + *dx), (y + *dy))
            })
            .filter(|c| self.within_bnd(*c)) // remove OOB
            .map(|c| {
                let (x, y) = c;
                (x as usize, y as usize) // back to usize
            })
            .collect()
    }
}

fn parse_input(input_path: &str) -> InputData {
    let roll_matrix: Vec<Vec<bool>> = fs::read_to_string(input_path)
        .unwrap()
        .split("\n")
        .filter(|x| return x.len() > 0) // ignore blank line
        .map(|x| {
            x.chars()
                .map(|x| match x {
                    '@' => true, // @ => roll, otherwise blank space.
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

/// Construct the adjacency count matrix for each point.
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

/// Check which squares are removable.
///
/// A square is removable iff it has fewer than 4 neighbours.
fn check_can_remove(input_data: &InputData, adjacency: &Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    adjacency
        .iter()
        .zip(input_data.roll_matrix.iter())
        .map(|row| {
            let (adj_row, input_row) = row;
            adj_row
                .iter()
                .zip(input_row)
                .map(|x| {
                    let (n_adj, is_roll) = x;
                    // key condition:
                    if *is_roll && *n_adj < 4 { true } else { false }
                })
                .collect()
        })
        .collect()
}

pub fn go(input_path: &str) -> String {
    // initial input and adjacency matrix.
    // mutable because they will change iteratively in pt 2!
    let mut input_data = parse_input(input_path);
    let mut adjacency = construct_adjacency(&input_data);

    let mut removable: Vec<Vec<bool>>;
    // this tracks what the initially removable count was (pt 1)
    let mut n_removable_initially = 0;
    let mut total_n_removable = 0;

    loop {
        removable = check_can_remove(&input_data, &adjacency);
        let n_removable = removable.iter().flatten().filter(|x| **x).count();
        if n_removable == 0 {
            // we are done
            break;
        }
        if n_removable_initially == 0 {
            // hacky but oh well
            n_removable_initially = n_removable;
        }
        // increment the total number that we will be able to remove.
        total_n_removable += n_removable;
        // remove all removable rolls.
        // removable <=> there exists a roll already, and it is marked as
        // removable.
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
        // recompute the adjacency matrix.
        adjacency = construct_adjacency(&input_data);
    }

    format!(
        "Initial number removable: {}.\nAfter iterated removals, {}",
        n_removable_initially, total_n_removable
    )
}
