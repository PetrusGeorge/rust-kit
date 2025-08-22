use crate::solution::Solution;
use instance_reader::Instance;

#[derive(Default, Clone)]
pub struct Subsequence {
    t: u32,
    pub c: u32,
    w: u32,
    first: usize,
    last: usize,
}

impl Subsequence {
    pub fn concatenate(&self, other: &Subsequence, instance: &Instance) -> Subsequence {
        let distance = instance.distance(self.last, other.first);
        let t = self.t + distance + other.t;
        let c = self.c + other.w * (self.t + distance) + other.c;
        let w = self.w + other.w;
        let first = self.first;
        let last = other.last;

        Subsequence {
            t,
            c,
            w,
            first,
            last,
        }
    }

    pub fn create_single_node(i: usize, s: &Solution) -> Self {
        Self {
            t: 0,
            c: 0,
            w: if i == 0 { 0 } else { 1 },
            first: s.sequence[i],
            last: s.sequence[i],
        }
    }
}

pub struct SubsequenceMatrix {
    matrix: Vec<Subsequence>,
    dimension: usize,
}

impl SubsequenceMatrix {
    pub fn from(dimension: usize) -> Self {
        SubsequenceMatrix {
            matrix: vec![Default::default(); dimension * dimension],
            dimension,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> &Subsequence {
        &self.matrix[(i * self.dimension) + j]
    }
    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut Subsequence {
        &mut self.matrix[(i * self.dimension) + j]
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }
}

// Ignore this clippy lint that triggers on the first loop
// rewriting it using iter is needless complicated
#[allow(clippy::needless_range_loop)]
pub fn update_subsequences(
    s: &Solution,
    subseq_matrix: &mut SubsequenceMatrix,
    instance: &Instance,
    bounds: Option<(usize, usize)>,
) {
    let (begin, end) = bounds.unwrap_or((0, s.sequence.len() - 1));

    // Single node subsequences
    for i in begin..=end {
        *subseq_matrix.get_mut(i, i) = Subsequence::create_single_node(i, s);
    }

    // Direct subsequences
    for i in 0..=end {
        let first_col = std::cmp::max(begin, i + 1);
        for j in first_col..s.sequence.len() {
            *subseq_matrix.get_mut(i, j) = subseq_matrix
                .get(i, j - 1)
                .concatenate(subseq_matrix.get(j, j), instance);
        }
    }

    // Reverse subsequences
    for i in (begin..s.sequence.len()).rev() {
        let last_col = std::cmp::min(end as isize, i.saturating_sub(1) as isize) as usize;
        for j in (0..=last_col).rev() {
            *subseq_matrix.get_mut(i, j) = subseq_matrix
                .get(i, j + 1)
                .concatenate(subseq_matrix.get(j, j), instance);
        }
    }
}

pub fn extract_solution_cost(subseq_matrix: &SubsequenceMatrix) -> u32 {
    subseq_matrix.get(0, subseq_matrix.dimension - 1).c
}

pub fn update_solution(
    s: &mut Solution,
    subseq_matrix: &mut SubsequenceMatrix,
    instance: &Instance,
    bounds: Option<(usize, usize)>,
) {
    update_subsequences(s, subseq_matrix, instance, bounds);
    s.value = extract_solution_cost(subseq_matrix);
}
