use rand::{Rng, rng};

use crate::solution::Solution;

use super::subsequence::Subsequence;

#[derive(Clone)]
enum Searches {
    Swap,
    TwoOpt,
    OrOpt(usize),
}

fn best_swap(s: &mut Solution) -> bool {
    let mut best_delta = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let last_idx = s.sequence.len() - 1;

    for i in 1..s.sequence.len() - 2 {
        for j in i + 2..s.sequence.len() - 1 {
            let sigma = s
                .subseq_matrix
                .concatenator_for(0, i - 1)
                .concatenate(j, j)
                .concatenate(i + 1, j - 1)
                .concatenate(i, i)
                .concatenate(j + 1, last_idx)
                .into_subsequence();

            let delta = sigma.c as i32 - s.value as i32;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence.swap(best_i, best_j);
        s.update(Some((best_i, best_j)));

        return true;
    }

    false
}

fn best_2opt(s: &mut Solution) -> bool {
    let mut best_delta = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let last_idx = s.sequence.len() - 1;

    for i in 1..s.sequence.len() - 2 {
        for j in i + 1..s.sequence.len() - 1 {
            let sigma = s
                .subseq_matrix
                .concatenator_for(0, i - 1)
                .concatenate(j, i)
                .concatenate(j + 1, last_idx)
                .into_subsequence();

            let delta = sigma.c as i32 - s.value as i32;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence[best_i..=best_j].reverse();
        s.update(Some((best_i, best_j)));

        return true;
    }

    false
}

fn best_oropt(s: &mut Solution, block_size: usize) -> bool {
    let mut best_delta = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    let last_idx = s.sequence.len() - 1;

    for i in 1..s.sequence.len() - block_size {
        let mut check_delta = |sigma: Subsequence, j: usize| {
            let delta = sigma.c as i32 - s.value as i32;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        };

        // Insert block before i
        for j in 1..i.saturating_sub(2) {
            let sigma = s
                .subseq_matrix
                .concatenator_for(0, j)
                .concatenate(i, i + block_size - 1)
                .concatenate(j + 1, i - 1)
                .concatenate(i + block_size, last_idx)
                .into_subsequence();

            check_delta(sigma, j);
        }

        // Insert block after i
        for j in i + block_size..s.sequence.len() - 1 {
            let sigma = s
                .subseq_matrix
                .concatenator_for(0, i - 1)
                .concatenate(i + block_size, j)
                .concatenate(i, i + block_size - 1)
                .concatenate(j + 1, last_idx)
                .into_subsequence();

            check_delta(sigma, j);
        }
    }

    if best_delta < 0 {
        if best_i < best_j {
            s.sequence[best_i..=best_j].rotate_left(block_size);
            s.update(Some((best_i, best_j)));
        } else {
            s.sequence[(best_j + 1)..(best_i + block_size)].rotate_right(block_size);
            s.update(Some((best_j, best_i + block_size)));
        }

        return true;
    }

    false
}

pub fn local_search(s: &mut Solution) {
    use Searches::*;
    const SEARCHES: [Searches; 5] = [Swap, TwoOpt, OrOpt(1), OrOpt(2), OrOpt(3)];

    let mut nl = SEARCHES.to_vec();

    while !nl.is_empty() {
        let chosen = rng().random_range(0..nl.len());
        let search_type = &nl[chosen];

        let improved = match search_type {
            Swap => best_swap(s),
            TwoOpt => best_2opt(s),
            OrOpt(block_size) => best_oropt(s, *block_size),
        };

        if improved {
            nl = SEARCHES.to_vec();
        } else {
            nl.swap_remove(chosen);
        }
    }
}
