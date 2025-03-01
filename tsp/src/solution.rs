#[derive(Debug)]
pub struct Solution {
    pub sequence: Vec<usize>,
    pub value: usize,
}

pub fn recalculate_solution(s: &mut Solution, instance: &Instance) {
    let mut sum = 0;
    for v in s.sequence.windows(2) {
        sum += instance.matrix[v[0]][v[1]];
    }
    s.value = sum;
}
