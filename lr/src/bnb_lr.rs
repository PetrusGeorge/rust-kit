use std::collections::{HashSet, VecDeque};

use instance_reader::Instance;

use crate::lr::lr;
use crate::solution::Solution;

fn convert_solution(s_in: &[Vec<usize>], cost: f64) -> Solution {
    let n = s_in.len();

    let mut sequence = Vec::with_capacity(n + 1);
    sequence.push(0);

    let mut current = s_in[0][0];
    sequence.push(current);

    let mut not_visited: HashSet<usize> = (1..n).collect();
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
        value: cost as usize,
    }
}

pub fn bnb_lr(instance: &Instance, upperbound: usize) -> Solution {
    let mut tree = VecDeque::new();
    let mut upperbound = upperbound as f64;
    let mut best_node = Default::default();

    tree.push_back(lr(Default::default(), instance, upperbound));

    while !tree.is_empty() {
        let node = tree.pop_back().unwrap();
        println!("{:?}", node);

        if node.solution.is_some() {
            if node.value < upperbound {
                println!("{upperbound}");
                upperbound = node.value;
                best_node = node;
            }
            continue;
        }

        let (index_first, indeces) = node.ban_from_child.as_ref().unwrap();
        for i in indeces.iter() {
            let mut new_node = node.clone();
            println!("Meu pai {:?}", new_node);
            new_node.forbidden_arcs.insert((*index_first, *i));
            new_node = lr(new_node, instance, upperbound);
            println!("Depois {:?}\n\n", new_node);
            if new_node.value < upperbound {
                tree.push_back(new_node);
            }
        }
    }

    println!("{:?}", best_node);

    convert_solution(&best_node.solution.unwrap(), best_node.value)
}
