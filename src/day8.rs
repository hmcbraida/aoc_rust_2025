use core::panic;
use std::fs;

type ThreeVector = (f64, f64, f64);

type InputData = Vec<ThreeVector>;

fn parse_input(input_path: &str) -> InputData {
    fs::read_to_string(&input_path)
        .unwrap()
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| {
            let nums: Vec<f64> = line
                .split(",")
                .map(|atom| atom.parse::<f64>().unwrap())
                .collect();
            if let &[x, y, z, ..] = nums.as_slice() {
                return (x, y, z);
            }

            panic!("Could not parse: {}", line);
        })
        .collect()
}

fn vec_length(my_vec: ThreeVector) -> f64 {
    (my_vec.0.powf(2.0) + my_vec.1.powf(2.0) + my_vec.2.powf(2.0)).sqrt()
}

fn vec_diff(vec1: ThreeVector, vec2: ThreeVector) -> ThreeVector {
    (vec1.0 - vec2.0, vec1.1 - vec2.1, vec1.2 - vec2.2)
}

fn calc_dist_matrix(input_data: &InputData) -> Vec<Vec<f64>> {
    input_data
        .iter()
        .map(|pos_vec| {
            input_data
                .iter()
                .map(|other_pos_vec| vec_diff(*pos_vec, *other_pos_vec))
                .map(|v| vec_length(v))
                .collect()
        })
        .collect()
}

fn find_min_dist(dist_matrix: &Vec<Vec<f64>>, components: &Vec<Vec<usize>>) -> (usize, usize) {
    let flat_dist: Vec<_> = dist_matrix
        .iter()
        .map(|row| row.iter().enumerate())
        .enumerate()
        .map(|(row_idx, row)| row.map(move |(col_idx, d)| (row_idx, col_idx, *d)))
        .flatten()
        // .filter(|(row_idx, col_idx, _)| *row_idx != *col_idx)
        .filter(|(row_idx, col_idx, _)| !components[*row_idx].contains(&col_idx))
        .collect();

    // println!("{:?}", flat_dist);

    flat_dist
        .iter()
        .min_by(|(_, _, v1), (_, _, v2)| v1.partial_cmp(v2).unwrap())
        .map(|(x, y, _)| (*x, *y))
        .unwrap()
}

fn connect_values(components: &mut Vec<Vec<usize>>, a: usize, b: usize) {
    let mut first_set = components[a].clone();
    let mut second_set = components[b].clone();

    // println!("first (idx {}): {:?}", a, first_set);
    // println!("second (idx {}): {:?}", b, second_set);

    first_set.append(&mut second_set);
    first_set.sort();
    first_set.dedup();

    // println!("{:?}", first_set);

    let new_set = first_set;

    for idx in new_set.iter() {
        components[*idx] = new_set.clone();
    }
}

fn calc_three_biggest_product(components: &Vec<Vec<usize>>) -> u64 {
    let mut biggest: Vec<_> = components.iter().map(|list| list.len() as u64).collect();

    biggest.sort();
    biggest.dedup();
    biggest.reverse();

    // println!("{:?}", biggest);

    if let &[x, y, z, ..] = biggest.as_slice() {
        return x * y * z;
    }

    return 0;
}

pub fn go(input_path: &str) -> String {
    let input_data = parse_input(&input_path);
    let dist_matrix = calc_dist_matrix(&input_data);

    let mut components: Vec<_> = (0..input_data.len()).map(|idx| vec![idx]).collect();

    let mut iter_c: usize = 1;
    loop {
        let (v1_idx, v2_idx) = find_min_dist(&dist_matrix, &components);

        println!("connecting {} and {}", v1_idx, v2_idx);
        // println!("{}", calc_three_biggest_product(&components));
        iter_c += 1;
        if iter_c > 1000 {
            break;
        }

        connect_values(&mut components, v1_idx, v2_idx);
    }

    // println!("{:?}", components);

    format!(
        "product of three biggest: {}",
        calc_three_biggest_product(&components)
    )
}
