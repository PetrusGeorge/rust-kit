use crate::ils::subsequence::SubsequenceMatrix;
use instance_reader::Instance;

#[derive(Debug, Clone)]
pub struct Solution<'a> {
    pub sequence: Vec<usize>,
    pub value: u32,
    pub subseq_matrix: SubsequenceMatrix<'a>,
}

impl<'a> Solution<'a> {
    pub fn new(sequence: Vec<usize>, value: u32, instance: &'a Instance) -> Self {
        Solution {
            sequence,
            value,
            subseq_matrix: SubsequenceMatrix::from(instance),
        }
    }

    pub fn update(&mut self, bounds: Option<(usize, usize)>) {
        self.subseq_matrix.update(&self.sequence, bounds);
        self.value = self.subseq_matrix.extract_solution_cost();
    }

    pub fn apply_double_bridge(
        &mut self,
        i: usize,
        j: usize,
        block_size_i: usize,
        block_size_j: usize,
    ) {
        let i_end = i + block_size_i;
        let j_end = j + block_size_j;
        self.sequence[i..j_end].reverse();
        self.sequence[i..i_end].reverse();
        self.sequence[i_end..j].reverse();
        self.sequence[j..j_end].reverse();

        self.update(Some((i, j_end)));
    }
}
