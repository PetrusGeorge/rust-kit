use core::panic;

use tsplib::{EdgeWeight, EdgeWeightType, NodeCoord};

#[derive(Default, Debug)]
pub struct Instance {
    dimension: usize,
    matrix: Vec<Vec<usize>>,
}

fn full_to_full(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut index = 0;
    for row in 0..n {
        for col in 0..n {
            matrix[row][col] = vec[index];
            matrix[col][row] = vec[index];
            index += 1;
        }
    }

    matrix
}

fn upper_to_full(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut index = 0;
    for row in 0..n {
        for col in row + 1..n {
            matrix[row][col] = vec[index];
            matrix[col][row] = vec[index];
            index += 1;
        }
    }

    matrix
}

fn upperdiag_to_full(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut index = 0;
    for row in 0..n {
        for col in row + 0..n {
            matrix[row][col] = vec[index];
            matrix[col][row] = vec[index];
            index += 1;
        }
    }

    matrix
}

fn lower_to_full(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut index = 0;
    for row in 1..n {
        for col in 0..row {
            matrix[row][col] = vec[index];
            matrix[col][row] = vec[index];
            index += 1;
        }
    }

    matrix
}

fn lowerdiag_to_full(vec: Vec<usize>, n: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let mut index = 0;
    for row in 0..n {
        for col in 0..=row {
            matrix[row][col] = vec[index];
            matrix[col][row] = vec[index];
            index += 1;
        }
    }

    matrix
}

fn euc_2d(coords: &Vec<(usize, f32, f32)>) -> Vec<Vec<usize>> {
    let n = coords.len();
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let calc_dist_euc = |i: usize, j: usize| {
        (((coords[i].1 - coords[j].1).powf(2.0) + (coords[i].2 - coords[j].2).powf(2.0)).sqrt()
            + 0.5)
            .floor() as usize
    };

    for row in 0..n {
        for col in 0..n {
            matrix[row][col] = calc_dist_euc(row, col);
        }
    }

    matrix
}

fn att(coords: &Vec<(usize, f32, f32)>) -> Vec<Vec<usize>> {
    let n = coords.len();
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];

    let calc_dist_att = |i: usize, j: usize| {
        let euc = (((coords[i].1 - coords[j].1).powf(2.0) + (coords[i].2 - coords[j].2).powf(2.0))
            / 10.0)
            .sqrt();

        let rounded = (euc + 0.5).floor() as usize;

        if (rounded as f32) < euc {
            rounded + 1
        } else {
            rounded
        }
    };

    for row in 0..n {
        for col in 0..n {
            matrix[row][col] = calc_dist_att(row, col);
        }
    }

    matrix
}

fn geo(coords: &Vec<(usize, f32, f32)>) -> Vec<Vec<usize>> {
    use std::f32::consts::PI;
    let n = coords.len();
    let mut matrix: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let latit: Vec<f32> = coords
        .iter()
        .map(|x| PI * (x.1.floor() + 5.0 * x.1.fract() / 3.0) / 180.0)
        .collect();
    let longit: Vec<f32> = coords
        .iter()
        .map(|x| PI * (x.2.floor() + 5.0 * x.2.fract() / 3.0) / 180.0)
        .collect();

    println!("{} {:?}", coords[0].2.floor(), coords[0].2.fract());

    let calc_dist_geo = |i: usize, j: usize| {
        const RRR: f32 = 6378.388; // Earth radius approximation

        let q1 = (longit[i] - longit[j]).cos();
        let q2 = (latit[i] - latit[j]).cos();
        let q3 = (latit[i] + latit[j]).cos();

        let distance = RRR * ((0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).acos());

        (distance + 1.0) as usize
    };

    for row in 0..n {
        for col in 0..n {
            if row == col {
                matrix[row][col] = 0;
                continue;
            }
            matrix[row][col] = calc_dist_geo(row, col);
        }
    }

    matrix
}

pub fn read_data(file_path: &str) -> Instance {
    let instance = tsplib::read(file_path).unwrap();

    let dimension = instance.dimension;
    println!("{:?}", instance);

    let weight_type = instance.edge_weight_type.unwrap();

    use EdgeWeight::*;
    use EdgeWeightType::*;
    let matrix = if let Explicit = weight_type {
        let matrix_type = instance.edge_weight.unwrap();
        match matrix_type {
            FullMatrix(vec) => full_to_full(vec, dimension),
            UpperRow(vec) => upper_to_full(vec, dimension),
            LowerRow(vec) => lower_to_full(vec, dimension),
            UpperDiagRow(vec) => upperdiag_to_full(vec, dimension),
            LowerDiagRow(vec) => lowerdiag_to_full(vec, dimension),
            _ => panic!("Edge weight not supported"),
        }
    } else {
        let coords = if let Some(NodeCoord::Two(coord)) = instance.node_coord {
            coord
        } else {
            panic!("Somethin went wrong with the node coordinates type");
        };
        match weight_type {
            Euc2d => euc_2d(&coords),
            Geo => geo(&coords),
            Att => att(&coords),
            _ => panic!("Edge weight type not supported"),
        }
    };

    println!("{:?}", matrix);

    Instance { dimension, matrix }
}
