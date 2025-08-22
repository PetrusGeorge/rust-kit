use hungarian::*;
use instance_reader::Instance;
use std::collections::{HashSet, VecDeque};

use crate::solution::Solution;

pub enum SearchMode {
    Bfs,
    Dfs,
}

#[derive(Clone, Debug)]
struct Node {
    forbidden_arcs: Vec<(usize, usize)>,
    smallest_subtour: Vec<usize>,
    value: u32,
}

fn find_smallest_subtour(solution: &HungarianResult) -> Vec<usize> {
    let n = solution.assigment.len();
    let mut not_included: HashSet<usize> = (0..n).collect();
    let mut smaller = Vec::new();
    while !not_included.is_empty() {
        let mut current_node = not_included.iter().next().cloned().unwrap();
        let first_node = current_node;
        let mut current_subtour = vec![first_node];
        not_included.remove(&current_node);

        loop {
            let a = solution.assigment[current_node]
                .iter()
                .position(|&x| x == 1)
                .unwrap();

            if a == first_node {
                break;
            }
            not_included.remove(&a);
            current_subtour.push(a);
            current_node = a;
        }

        current_subtour.push(first_node);

        if current_subtour.len() < smaller.len() || smaller.is_empty() {
            smaller = current_subtour;
        }
    }
    smaller
}

fn set_matrix(forbidden_arcs: &[(usize, usize)], instance: &Instance) -> Vec<Vec<i32>> {
    let n = instance.dimension;
    let mut matrix = vec![vec![0; n]; n];

    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, value) in row.iter_mut().enumerate() {
            let distance = if i == j {
                99999999
            } else {
                instance.distance(i, j) as i32
            };
            *value = distance;
        }
    }
    for (i, j) in forbidden_arcs {
        // Setting this to i32::MAX causes problems with the hungarian solver
        matrix[*i][*j] = 99999999;
    }

    matrix
}

pub fn bnb(instance: &Instance, mode: SearchMode) -> Solution {
    let mut tree = VecDeque::new();
    let mut upperbound = u32::MAX;
    let mut best_node = Node {
        forbidden_arcs: Vec::new(),
        smallest_subtour: Vec::new(),
        value: u32::MAX,
    };

    // Solve root node
    let mut h = Hungarian::new(&set_matrix(&[], instance), HungarianMode::MinimizeCost);
    let h_result = h.solve();
    let smallest_subtour = find_smallest_subtour(&h_result);
    let forbidden_arcs = Vec::new();
    let value = h_result.cost as u32;
    tree.push_back(Node {
        forbidden_arcs,
        smallest_subtour,
        value,
    });

    while !tree.is_empty() {
        let node = match mode {
            SearchMode::Bfs => tree.pop_front().unwrap(),
            SearchMode::Dfs => tree.pop_back().unwrap(),
        };

        if node.smallest_subtour.len() == instance.dimension + 1 {
            if node.value < upperbound {
                upperbound = node.value;
                best_node = node.clone();
            }
            continue;
        }

        for arc in node.smallest_subtour.windows(2) {
            let mut forbidden_arcs = node.forbidden_arcs.clone();
            forbidden_arcs.push((arc[0], arc[1]));

            let matrix = set_matrix(&forbidden_arcs, instance);

            let mut h = Hungarian::new(&matrix, HungarianMode::MinimizeCost);
            let h_result = h.solve();
            let smallest_subtour = find_smallest_subtour(&h_result);
            let value = h_result.cost as u32;
            if value < upperbound {
                tree.push_back(Node {
                    forbidden_arcs,
                    smallest_subtour,
                    value,
                });
            }
        }
    }

    Solution {
        sequence: best_node.smallest_subtour,
        value: best_node.value,
    }
}
