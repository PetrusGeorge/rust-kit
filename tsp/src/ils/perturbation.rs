use rand::{Rng, rng};
use std::cmp::max;

use crate::solution::Solution;
use instance_reader::Instance;

pub fn perturbation(mut s: Solution, instance: &Instance) -> Solution {
    // Choose block sizes beetwen 2 and ceil(V/10)
    let n = instance.dimension;
    let upper_bound = max((instance.dimension as f32 / 10.0).ceil() as usize, 2);
    let mut block_size_i = rng().random_range(2..=upper_bound);
    let mut block_size_j = rng().random_range(2..=upper_bound);

    // Choose where the i block is going to be put
    let mut i = rng().random_range(1..(n - block_size_i));

    // Calculate the possibilities and decide if the other block is going before or after
    let possibilities_before_i = i.saturating_sub(block_size_j);
    let possibilities_after_i = n.saturating_sub(i + block_size_i - 1 + block_size_j);
    let back = rng().random_range(1..=(possibilities_before_i + possibilities_after_i))
        <= possibilities_before_i;

    let mut j;

    if back {
        j = rng().random_range(1..=possibilities_before_i);

        // Ensures i < j
        std::mem::swap(&mut block_size_i, &mut block_size_j);
        std::mem::swap(&mut i, &mut j);
    } else {
        j = rng().random_range(1..=possibilities_after_i) + i + block_size_i - 1;
    }

    s.apply_double_bridge(i, j, block_size_i, block_size_j);
    s.recalculate(instance);

    s
}
