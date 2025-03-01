use crate::instance_reader::Instance;
use crate::solution::*;
use rand::seq::IteratorRandom;
use rand::{Rng, rng};

// Auxiliary data structure for best insertion
struct InsertionInfo {
    cl_index: usize,
    removed_edge: usize,
    value: usize,
}

// Calculate every possible insertion from cl into the solution
fn calculate_insertion_cost(s: &Solution, cl: &[usize], instance: &Instance) -> Vec<InsertionInfo> {
    let mut insertion_cost: Vec<InsertionInfo> =
        Vec::with_capacity(cl.len() * (s.sequence.len() - 1));

    for a in 0..(s.sequence.len() - 1) {
        let i = s.sequence[a];
        let j = s.sequence[a + 1];
        for (cl_index, inserted_node) in cl.iter().enumerate() {
            let value = instance.matrix[i][*inserted_node] + instance.matrix[*inserted_node][j]
                - instance.matrix[i][j];

            insertion_cost.push(InsertionInfo {
                cl_index,
                removed_edge: a + 1,
                value,
            })
        }
    }

    insertion_cost
}

fn choose_three_random(cl: &mut Vec<usize>, instance: &Instance) -> Solution {
    // Choose 3 random clients
    let mut sequence = Vec::new();
    for _ in 0..3 {
        let index = (0..cl.len()).choose(&mut rng()).unwrap();
        sequence.push(cl.swap_remove(index));
    }

    // Fix 0 as the first client
    sequence.insert(0, 0);
    sequence.push(0);

    let mut s = Solution {
        sequence,
        value: usize::MAX,
    };
    recalculate_solution(&mut s, instance);

    s
}

// Constructs a solution with a grasp algorithm using best insertion
pub fn construction(instance: &Instance) -> Solution {
    // cl is the candidate list to insert into the solution
    let mut cl: Vec<usize> = (1..instance.dimension).collect();
    let mut s = choose_three_random(&mut cl, instance);

    while !cl.is_empty() {
        let mut insertion_cost = calculate_insertion_cost(&s, &cl, instance);
        insertion_cost.sort_unstable_by_key(|x| x.value);

        // Choose a random index from insertion cost but the first values have more priority
        let alpha: f64 = rng().random_range(1e-10..1.0);
        let chosen =
            (rng().random::<u32>() % (alpha * insertion_cost.len() as f64).ceil() as u32) as usize;

        let chosen_insertion = &insertion_cost[chosen];

        s.sequence
            .insert(chosen_insertion.removed_edge, cl[chosen_insertion.cl_index]);

        cl.remove(chosen_insertion.cl_index);

        s.value += chosen_insertion.value;
    }

    s
}
