use tsplib::{EdgeWeight, EdgeWeightType, NodeCoord, Type};

#[derive(Clone, Debug)]
pub struct Instance {
    pub dimension: usize,
    pub name: String,
    matrix: Vec<u32>,
}

impl Instance {
    #[inline]
    pub fn distance(&self, i: usize, j: usize) -> u32 {
        self.matrix[(i * self.dimension) + j]
    }

    pub fn matrix_slice(&self) -> &[u32] {
        &self.matrix
    }
}

fn upper_to_full(vec: &[usize], n: usize) -> Vec<u32> {
    let mut matrix = vec![0; n * n];

    let mut index = 0;
    for row in 0..n {
        for col in row + 1..n {
            matrix[(row * n) + col] = vec[index] as u32;
            matrix[(col * n) + row] = vec[index] as u32;
            index += 1;
        }
    }

    matrix
}

fn upperdiag_to_full(vec: &[usize], n: usize) -> Vec<u32> {
    let mut matrix = vec![0; n * n];

    let mut index = 0;
    for row in 0..n {
        for col in row..n {
            matrix[(row * n) + col] = vec[index] as u32;
            matrix[(col * n) + row] = vec[index] as u32;
            index += 1;
        }
    }

    matrix
}

fn lower_to_full(vec: &[usize], n: usize) -> Vec<u32> {
    let mut matrix = vec![0; n * n];

    let mut index = 0;
    for row in 1..n {
        for col in 0..row {
            matrix[(row * n) + col] = vec[index] as u32;
            matrix[(col * n) + row] = vec[index] as u32;
            index += 1;
        }
    }

    matrix
}

fn lowerdiag_to_full(vec: &[usize], n: usize) -> Vec<u32> {
    let mut matrix = vec![0; n * n];

    let mut index = 0;
    for row in 0..n {
        for col in 0..=row {
            matrix[(row * n) + col] = vec[index] as u32;
            matrix[(col * n) + row] = vec[index] as u32;
            index += 1;
        }
    }

    matrix
}

fn euc_2d(coords: &[(usize, f32, f32)]) -> Vec<u32> {
    let n = coords.len();
    let mut matrix = vec![0; n * n];

    let calc_dist_euc = |i: usize, j: usize| {
        (((coords[i].1 - coords[j].1).powf(2.0) + (coords[i].2 - coords[j].2).powf(2.0)).sqrt()
            + 0.5)
            .floor() as u32
    };

    for (index, value) in matrix.iter_mut().enumerate() {
        let i = index / n;
        let j = index % n;
        *value = calc_dist_euc(i, j);
    }

    matrix
}

fn ceil_2d(coords: &[(usize, f32, f32)]) -> Vec<u32> {
    let n = coords.len();
    let mut matrix = vec![0; n * n];

    let calc_dist_ceil = |i: usize, j: usize| {
        ((coords[i].1 - coords[j].1).powf(2.0) + (coords[i].2 - coords[j].2).powf(2.0))
            .sqrt()
            .ceil() as u32
    };

    for (index, value) in matrix.iter_mut().enumerate() {
        let i = index / n;
        let j = index % n;
        *value = calc_dist_ceil(i, j);
    }

    matrix
}

fn att(coords: &[(usize, f32, f32)]) -> Vec<u32> {
    let n = coords.len();
    let mut matrix = vec![0; n * n];

    let calc_dist_att = |i: usize, j: usize| {
        let euc = (((coords[i].1 - coords[j].1).powf(2.0) + (coords[i].2 - coords[j].2).powf(2.0))
            / 10.0)
            .sqrt();

        let rounded = (euc + 0.5).floor() as u32;

        if (rounded as f32) < euc {
            rounded + 1
        } else {
            rounded
        }
    };

    for (index, value) in matrix.iter_mut().enumerate() {
        let i = index / n;
        let j = index % n;
        *value = calc_dist_att(i, j);
    }

    matrix
}

fn geo(coords: &[(usize, f32, f32)]) -> Vec<u32> {
    use std::f32::consts::PI;
    let n = coords.len();
    let mut matrix = vec![0; n * n];

    let latit: Vec<f32> = coords
        .iter()
        .map(|x| PI * (x.1.floor() + 5.0 * x.1.fract() / 3.0) / 180.0)
        .collect();
    let longit: Vec<f32> = coords
        .iter()
        .map(|x| PI * (x.2.floor() + 5.0 * x.2.fract() / 3.0) / 180.0)
        .collect();

    let calc_dist_geo = |i: usize, j: usize| {
        const RRR: f32 = 6378.388; // Earth radius approximation

        let q1 = (longit[i] - longit[j]).cos();
        let q2 = (latit[i] - latit[j]).cos();
        let q3 = (latit[i] + latit[j]).cos();

        let distance = RRR * ((0.5 * ((1.0 + q1) * q2 - (1.0 - q1) * q3)).acos());

        (distance + 1.0) as u32
    };

    for (index, value) in matrix.iter_mut().enumerate() {
        let i = index / n;
        let j = index % n;
        if i == j {
            *value = 0;
            continue;
        }
        *value = calc_dist_geo(i, j);
    }

    matrix
}

pub fn read_data(file_path: &str) -> Instance {
    let instance = tsplib::read(file_path).expect("Something went wrong while parsing the file");

    match instance.type_.expect("Instance type was not provided") {
        Type::Tsp => (),
        _ => panic!("This is not an tsp instance"),
    }

    let dimension = instance.dimension;
    let name = instance.name;

    let weight_type = instance
        .edge_weight_type
        .expect("Instance does't suppy a edge weight type");

    use EdgeWeight::*;
    use EdgeWeightType::*;
    let matrix = if let Explicit = weight_type {
        let matrix_type = instance
            .edge_weight
            .expect("Instance does't supply edge weight format in explicit type");
        match matrix_type {
            FullMatrix(vec) => vec.into_iter().map(|x| x as u32).collect(),
            UpperRow(vec) => upper_to_full(&vec, dimension),
            LowerRow(vec) => lower_to_full(&vec, dimension),
            UpperDiagRow(vec) => upperdiag_to_full(&vec, dimension),
            LowerDiagRow(vec) => lowerdiag_to_full(&vec, dimension),
            _ => panic!("Edge weight not supported"),
        }
    } else {
        let coords = if let Some(NodeCoord::Two(coord)) = instance.node_coord {
            coord
        } else {
            panic!("Something went wrong with the node coordinates type");
        };
        match weight_type {
            Euc2d => euc_2d(&coords),
            Ceil2d => ceil_2d(&coords),
            Geo => geo(&coords),
            Att => att(&coords),
            _ => panic!("Edge weight type not supported"),
        }
    };

    Instance {
        dimension,
        name,
        matrix,
    }
}
