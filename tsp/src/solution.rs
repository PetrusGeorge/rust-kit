use instance_reader::Instance;

#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub sequence: Vec<usize>,
    pub value: usize,
}

impl Solution {
    pub fn recalculate(&mut self, instance: &Instance) {
        let mut sum = 0;
        for v in self.sequence.windows(2) {
            sum += instance.matrix[v[0]][v[1]];
        }
        self.value = sum;
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
    }
}
