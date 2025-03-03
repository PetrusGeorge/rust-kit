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

    let iter = s.sequence.windows(3).enumerate();
    for (i, window) in iter {
        let vi_prev = window[0];
        let vi = window[1];
        let vi_next = window[2];

        let removal_delta = -(c(vi_prev, vi) + c(vi, vi_next));

        let inner_iter = s.sequence[i + 2..].windows(3).enumerate();
        for (j_offset, window_j) in inner_iter {
            let j = i + 3 + j_offset;
            let vj_prev = window_j[0];
            let vj = window_j[1];
            let vj_next = window_j[2];

            let delta = c(vi_prev, vj) + c(vj, vi_next) - c(vj_prev, vj) - c(vj, vj_next)
                + c(vj_prev, vi)
                + c(vi, vj_next)
                + removal_delta;

            if delta < best_delta {
                best_delta = delta;
                best_i = i + 1;
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

    let iter = s.sequence.windows(2).enumerate();
    for (i, window) in iter {
        let vi_prev = window[0];
        let vi = window[1];

        let inner_iter = s.sequence[i + 2..].windows(2).enumerate();
        for (j_offset, window_j) in inner_iter {
            let j = i + 2 + j_offset;

            let vj = window_j[0];
            let vj_next = window_j[1];

            let delta = c(vi, vj_next) + c(vj, vi_prev) - c(vi_prev, vi) - c(vj, vj_next);

            if delta < best_delta {
                best_delta = delta;
                best_i = i + 1;
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

    let iter = s.sequence.windows(block_size + 2).enumerate();
    for (i, window) in iter {
        let vi_prev = window[0];
        let vi = window[1];
        let vi_next = window[block_size + 1];
        let block_end = window[block_size];

        let removal_delta = c(vi_prev, vi_next) - c(vi_prev, vi) - c(block_end, vi_next);
        let mut check_delta = |j: usize, window_j: &[usize]| {
            let vj = window_j[0];
            let vj_next = window_j[1];

            let delta = c(vj, vi) + c(block_end, vj_next) - c(vj, vj_next) + removal_delta;

            if delta < best_delta {
                best_delta = delta;
                best_i = i + 1;
                best_j = j;
            }
        };

        let inner_iter_before = s.sequence.windows(2).take(i).enumerate();
        for (j_offset, window_j) in inner_iter_before {
            let j = j_offset;
            check_delta(j, window_j);
        }

        let inner_iter_after = s.sequence[i + block_size + 1..].windows(2).enumerate();
        for (j_offset, window_j) in inner_iter_after {
            let j = i + block_size + j_offset + 1;
            check_delta(j, window_j);
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
