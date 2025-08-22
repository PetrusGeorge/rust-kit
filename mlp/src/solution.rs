#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub sequence: Vec<usize>,
    pub value: u32,
}

impl Solution {
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
