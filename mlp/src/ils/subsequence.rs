use crate::solution::Solution;
use instance_reader::Instance;

#[derive(Default, Clone)]
pub struct Subsequence {
    t: usize,
    pub c: usize,
    w: usize,
    first: usize,
    last: usize,
}

impl Subsequence {
    pub fn concatenate(&self, other: &Subsequence, instance: &Instance) -> Subsequence {
        let distance = instance.matrix[self.last][other.first];
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

// Ignore this clippy lint that triggers on the first loop
// rewriting it using iter is needless complicated
#[allow(clippy::needless_range_loop)]
pub fn update_subsequences(
    s: &Solution,
    subseq_matrix: &mut [Vec<Subsequence>],
    instance: &Instance,
    bounds: Option<(usize, usize)>,
) {
    let (begin, end) = bounds.unwrap_or((0, s.sequence.len() - 1));

    // Single node subsequences
    for i in begin..=end {
        subseq_matrix[i][i] = Subsequence::create_single_node(i, s);
    }

    // Direct subsequences
    for i in 0..=end {
        let first_col = std::cmp::max(begin, i + 1);
        for j in first_col..s.sequence.len() {
            subseq_matrix[i][j] =
                subseq_matrix[i][j - 1].concatenate(&subseq_matrix[j][j], instance);
        }
    }

    // Reverse subsequences
    for i in (begin..s.sequence.len()).rev() {
        let last_col = std::cmp::min(end as isize, i as isize - 1);
        for j in (0..=last_col).rev() {
            let j = j as usize;
            subseq_matrix[i][j] =
                subseq_matrix[i][j + 1].concatenate(&subseq_matrix[j][j], instance);
        }
    }
}

pub fn extract_solution_cost(subseq_matrix: &[Vec<Subsequence>]) -> usize {
    subseq_matrix[0].last().unwrap().c
}

pub fn update_solution(
    s: &mut Solution,
    subseq_matrix: &mut [Vec<Subsequence>],
    instance: &Instance,
    bounds: Option<(usize, usize)>,
) {
    update_subsequences(s, subseq_matrix, instance, bounds);
    s.value = extract_solution_cost(subseq_matrix);
}
