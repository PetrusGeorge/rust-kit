use core::panic;
use std::collections::{BTreeSet, VecDeque};

use instance_reader::Instance;

use crate::lr::lr;
use crate::solution::Solution;

fn convert_solution(s_in: &[Vec<usize>], cost: f64) -> Solution {
    let n = s_in.len();

    let mut sequence = Vec::with_capacity(n + 1);
    sequence.push(0);

    let mut current = s_in[0][0];
    sequence.push(current);

    let mut not_visited: BTreeSet<usize> = (1..n).collect();
    not_visited.remove(&current);

    loop {
        let edges = &s_in[current];
        current = if not_visited.contains(&edges[0]) {
            edges[0]
        } else if not_visited.contains(&edges[1]) {
            edges[1]
        } else {
            break;
        };
        not_visited.remove(&current);
        sequence.push(current);
    }
    sequence.push(0);

    Solution {
        sequence,
        value: cost.round() as usize,
    }
}

pub fn bnb_lr(instance: &Instance, upperbound: usize) -> Option<Solution> {
    let mut tree = VecDeque::new();
    let mut upperbound = upperbound as f64;
    let mut best_node = None;

    // Solve root node
    tree.push_back(lr(Default::default(), instance, upperbound));

    while let Some(node) = tree.pop_back() {
        // If this is NONE than the upperbound is either wrong or is the optimal value
        let (index_first, indeces) = node.ban_from_child.as_ref()?;
        for i in indeces {
            let mut new_node = node.clone();
            let edge = if *index_first < *i {
                (*index_first, *i)
            } else {
                (*i, *index_first)
            };

            match new_node.forbidden_arcs.binary_search(&edge) {
                Ok(_) => panic!("Duplicated edge on forbidden arcs"),
                Err(idx) => new_node.forbidden_arcs.insert(idx, edge),
            }

            new_node = lr(new_node, instance, upperbound);

            if new_node.value < upperbound {
                // Is node feasible?
                if new_node.solution.is_some() {
                    upperbound = node.value;
                    best_node = Some(new_node);
                } else {
                    tree.push_back(new_node);
                }
            }
        }
    }

    let best_node = best_node?;
    Some(convert_solution(
        &best_node.solution.unwrap(),
        best_node.value,
    ))
}
