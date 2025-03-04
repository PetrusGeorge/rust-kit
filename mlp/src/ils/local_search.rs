use rand::{Rng, rng};

use crate::instance_reader::Instance;
use crate::solution::Solution;

use super::subsequence::{Subsequence, update_solution};

#[derive(Clone)]
enum Searches {
    Swap,
    TwoOpt,
    OrOpt(usize),
}

fn best_swap(
    s: &mut Solution,
    subseq_matrix: &mut [Vec<Subsequence>],
    instance: &Instance,
) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    for i in 1..s.sequence.len() - 2 {
        for j in i + 2..s.sequence.len() - 1 {
            let sigma = subseq_matrix[0][i - 1]
                .concatenate(&subseq_matrix[j][j], instance)
                .concatenate(&subseq_matrix[i + 1][j - 1], instance)
                .concatenate(&subseq_matrix[i][i], instance)
                .concatenate(subseq_matrix[j + 1].last().unwrap(), instance);

            let delta = sigma.c as isize - s.value as isize;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence.swap(best_i, best_j);
        update_solution(s, subseq_matrix, instance, Some((best_i, best_j)));

        return true;
    }

    false
}

fn best_2opt(
    s: &mut Solution,
    subseq_matrix: &mut [Vec<Subsequence>],
    instance: &Instance,
) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    for i in 1..s.sequence.len() - 2 {
        for j in i + 1..s.sequence.len() - 1 {
            let sigma = subseq_matrix[0][i - 1]
                .concatenate(&subseq_matrix[j][i], instance)
                .concatenate(subseq_matrix[j + 1].last().unwrap(), instance);

            let delta = sigma.c as isize - s.value as isize;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        }
    }

    if best_delta < 0 {
        s.sequence[best_i..=best_j].reverse();
        update_solution(s, subseq_matrix, instance, Some((best_i, best_j)));

        return true;
    }

    false
}

fn best_oropt(
    s: &mut Solution,
    subseq_matrix: &mut [Vec<Subsequence>],
    block_size: usize,
    instance: &Instance,
) -> bool {
    let mut best_delta: isize = 0;
    let mut best_i = usize::MAX;
    let mut best_j = usize::MAX;

    for i in 1..s.sequence.len() - block_size {
        let mut check_delta = |sigma: Subsequence, j: usize| {
            let delta = sigma.c as isize - s.value as isize;

            if delta < best_delta {
                best_delta = delta;
                best_i = i;
                best_j = j;
            }
        };

        // Insert block before i
        for j in 1..i.saturating_sub(2) {
            let sigma = subseq_matrix[0][j]
                .concatenate(&subseq_matrix[i][i + block_size - 1], instance)
                .concatenate(&subseq_matrix[j + 1][i - 1], instance)
                .concatenate(subseq_matrix[i + block_size].last().unwrap(), instance);

            check_delta(sigma, j);
        }

        // Insert block after i
        for j in i + block_size..s.sequence.len() - 1 {
            let sigma = subseq_matrix[0][i - 1]
                .concatenate(&subseq_matrix[i + block_size][j], instance)
                .concatenate(&subseq_matrix[i][i + block_size - 1], instance)
                .concatenate(subseq_matrix[j + 1].last().unwrap(), instance);

            check_delta(sigma, j);
        }
    }

    if best_delta < 0 {
        if best_i < best_j {
            s.sequence[best_i..=best_j].rotate_left(block_size);
            update_solution(s, subseq_matrix, instance, Some((best_i, best_j)));
        } else {
            s.sequence[(best_j + 1)..(best_i + block_size)].rotate_right(block_size);
            update_solution(
                s,
                subseq_matrix,
                instance,
                Some((best_j, best_i + block_size)),
            );
        }

        return true;
    }

    false
}

pub fn local_search(s: &mut Solution, subseq_matrix: &mut [Vec<Subsequence>], instance: &Instance) {
    use Searches::*;
    const SEARCHES: [Searches; 5] = [Swap, TwoOpt, OrOpt(1), OrOpt(2), OrOpt(3)];

    let mut nl = SEARCHES.to_vec();

    while !nl.is_empty() {
        let chosen = rng().random_range(0..nl.len());
        let search_type = &nl[chosen];

        let improved = match search_type {
            Swap => best_swap(s, subseq_matrix, instance),
            TwoOpt => best_2opt(s, subseq_matrix, instance),
            OrOpt(block_size) => best_oropt(s, subseq_matrix, *block_size, instance),
            // _ => false,
        };

        if improved {
            nl = SEARCHES.to_vec();
        } else {
            nl.swap_remove(chosen);
        }
    }
}
