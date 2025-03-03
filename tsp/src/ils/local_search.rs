use rand::{Rng, rng};

use crate::instance_reader::Instance;
use crate::solution::Solution;

#[derive(Clone)]
enum Searches {
    Swap,
    TwoOpt,
    OrOpt(usize),
}

fn best_swap(s: &mut Solution, instance: &Instance) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let c = |i: usize, j: usize| instance.matrix[i][j] as isize;

    for i in 1..(s.sequence.len() - 2) {
        let vi = s.sequence[i];
        let vi_next = s.sequence[i + 1];
        let vi_prev = s.sequence[i - 1];

        let removal_delta = -(c(vi_prev, vi) + c(vi, vi_next));
        for j in (i + 2)..(s.sequence.len() - 1) {
            let vj = s.sequence[j];
            let vj_next = s.sequence[j + 1];
            let vj_prev = s.sequence[j - 1];

            let delta = c(vi_prev, vj) + c(vj, vi_next) - c(vj_prev, vj) - c(vj, vj_next)
                + c(vj_prev, vi)
                + c(vi, vj_next)
                + removal_delta;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence.swap(best_i, best_j);
        s.value = (s.value as isize + best_delta) as usize;

        return true;
    }

    false
}

fn best_2opt(s: &mut Solution, instance: &Instance) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let c = |i: usize, j: usize| instance.matrix[i][j] as isize;

    for i in 1..(s.sequence.len() - 2) {
        let vi = s.sequence[i];
        let vi_prev = s.sequence[i - 1];

        for j in (i + 1)..(s.sequence.len() - 1) {
            let vj = s.sequence[j];
            let vj_next = s.sequence[j + 1];

            let delta = c(vi, vj_next) + c(vj, vi_prev) - c(vi_prev, vi) - c(vj, vj_next);

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence[best_i..=best_j].reverse();
        s.value = (s.value as isize + best_delta) as usize;

        return true;
    }

    false
}

fn best_oropt(s: &mut Solution, block_size: usize, instance: &Instance) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let c = |i: usize, j: usize| instance.matrix[i][j] as isize;

    for i in 1..(s.sequence.len() - block_size) {
        let vi = s.sequence[i];
        let vi_next = s.sequence[i + block_size];
        let vi_prev = s.sequence[i - 1];
        let block_end = s.sequence[i + block_size - 1];

        let removal_delta = c(vi_prev, vi_next) - c(vi_prev, vi) - c(block_end, vi_next);

        let mut check_delta = |j: usize| {
            let vj = s.sequence[j];
            let vj_next = s.sequence[j + 1];
            let delta = c(vj, vi) + c(block_end, vj_next) - c(vj, vj_next) + removal_delta;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        };

        // Insert block before i
        for j in 0..(i - 1) {
            check_delta(j);
        }
        // Insert block after i
        for j in (i + block_size)..(s.sequence.len() - block_size) {
            check_delta(j);
        }
    }

    if best_delta < 0 {
        if best_i < best_j {
            s.sequence[best_i..=best_j].rotate_left(block_size);
        } else {
            s.sequence[(best_j + 1)..(best_i + block_size)].rotate_right(block_size);
        }

        s.value = (s.value as isize + best_delta) as usize;

        return true;
    }

    false
}

pub fn local_search(s: &mut Solution, instance: &Instance) {
    use Searches::*;
    const SEARCHES: [Searches; 5] = [Swap, TwoOpt, OrOpt(1), OrOpt(2), OrOpt(3)];

    let mut nl = SEARCHES.to_vec();

    while !nl.is_empty() {
        let chosen = rng().random_range(0..nl.len());
        let search_type = &nl[chosen];

        let improved = match search_type {
            Swap => best_swap(s, instance),
            TwoOpt => best_2opt(s, instance),
            OrOpt(block_size) => best_oropt(s, *block_size, instance),
        };

        if improved {
            nl = SEARCHES.to_vec();
        } else {
            nl.swap_remove(chosen);
        }
    }
}
