use crate::solution::*;
use instance_reader::Instance;
use rand::{Rng, rng};

use super::subsequence::{SubsequenceMatrix, update_solution};

// Auxiliary data structure for best insertion
struct InsertionInfo {
    cl_index: usize,
    value: u32,
}

// Calculate every possible insertion from cl into the solution
fn calculate_insertion_cost(s: &Solution, cl: &[usize], instance: &Instance) -> Vec<InsertionInfo> {
    let mut insertion_cost: Vec<InsertionInfo> = Vec::with_capacity(cl.len());
    let last_sequence = s.sequence.last().unwrap();

    for (i, node) in cl.iter().enumerate() {
        insertion_cost.push(InsertionInfo {
            cl_index: i,
            value: instance.distance(*last_sequence, *node),
        });
    }

    insertion_cost
}

// Constructs a solution with a grasp algorithm using best insertion
pub fn construction(subseq_matrix: &mut SubsequenceMatrix, instance: &Instance) -> Solution {
    // cl is the candidate list to insert into the solution
    let mut cl: Vec<usize> = (1..instance.dimension).collect();
    let mut s = Solution {
        sequence: vec![0],
        value: u32::MAX,
    };

    while !cl.is_empty() {
        let mut insertion_cost = calculate_insertion_cost(&s, &cl, instance);
        insertion_cost.sort_unstable_by_key(|x| x.value);

        // Choose a random index from insertion cost but the first values have more priority
        let alpha: f64 = rng().random_range(1e-10..1.0);
        let chosen =
            (rng().random::<u32>() % (alpha * insertion_cost.len() as f64).ceil() as u32) as usize;

        let chosen_insertion = &insertion_cost[chosen];

        s.sequence.push(cl[chosen_insertion.cl_index]);

        cl.swap_remove(chosen_insertion.cl_index);
    }

    s.sequence.push(0);
    update_solution(&mut s, subseq_matrix, instance, None);

    s
}
