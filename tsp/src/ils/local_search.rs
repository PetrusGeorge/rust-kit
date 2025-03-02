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
    false
}
fn best_2opt(s: &mut Solution, instance: &Instance) -> bool {
    false
}
fn best_oropt(s: &mut Solution, block_size: usize, instance: &Instance) -> bool {
    false
}

pub fn local_search(s: &mut Solution, instance: &Instance) {
    use Searches::*;
    const SEARCHES: [Searches; 5] = [Swap, TwoOpt, OrOpt(1), OrOpt(2), OrOpt(3)];

    let mut nl = SEARCHES.to_vec();

    while !nl.is_empty() {
        let chose = rng().random_range(0..nl.len());
        let search_type = &nl[chose];

        let improved = match search_type {
            Swap => best_swap(s, instance),
            TwoOpt => best_2opt(s, instance),
            OrOpt(block_size) => best_oropt(s, *block_size, instance),
        };

        if improved {
            nl = SEARCHES.to_vec();
        } else {
            nl.swap_remove(chose);
        }
    }
}
