mod kruskal;

use instance_reader::Instance;
use kruskal::{Edge, mst};
use std::collections::BinaryHeap;

#[derive(Debug, Default, Clone)]
pub struct Node {
    pub forbidden_arcs: Vec<(usize, usize)>,
    pub lambdas: Vec<f64>,
    pub solution: Option<Vec<Vec<usize>>>,
    pub ban_from_child: Option<(usize, Vec<usize>)>,
    pub value: f64,
}

fn build_priority_queue(
    instance: &Instance,
    lambdas: &[f64],
    forbidden_arcs: &[(usize, usize)],
) -> BinaryHeap<Edge> {
    let n = instance.dimension;
    // The first node will not be part of kruskal
    let final_size = (n - 1) * (n - 2) / 2 - forbidden_arcs.len();
    let mut edges = Vec::with_capacity(final_size);

    let mut forbidden_arcs_it = forbidden_arcs
        .iter()
        .skip_while(|(a, _)| *a == 0)
        .peekable();

    for i in 1..n {
        for j in i + 1..n {
            if let Some(&&(a, b)) = forbidden_arcs_it.peek() {
                if (a, b) == (i, j) {
                    forbidden_arcs_it.next();
                    continue;
                }
            }

            let cost = instance.distance(i, j) as f64 - lambdas[i] - lambdas[j];

            edges.push(Edge::from(cost, i, j));
        }
    }

    edges.into()
}

fn closest_to_first_node(
    instance: &Instance,
    lambdas: &[f64],
    forbidden_arcs: &[(usize, usize)],
) -> (usize, usize, f64) {
    let mut lowest_index_1 = 0;
    let mut lowest_index_2 = 0;
    let mut lowest1 = f64::INFINITY;
    let mut lowest2 = f64::INFINITY;

    let mut forbidden_arcs_it = forbidden_arcs
        .iter()
        .take_while(|(a, _)| *a == 0)
        .peekable();

    for (i, lambda) in lambdas.iter().enumerate().skip(1) {
        if let Some(&&(_, b)) = forbidden_arcs_it.peek() {
            if b == i {
                forbidden_arcs_it.next();
                continue;
            }
        }

        // Lambda[0] is always 0 so we ignore it
        let cost = instance.distance(0, i) as f64 - lambda;

        if cost < lowest1 {
            // Shift the value to second
            lowest2 = lowest1;
            lowest_index_2 = lowest_index_1;

            lowest1 = cost;
            lowest_index_1 = i;
        } else if cost < lowest2 {
            lowest2 = cost;
            lowest_index_2 = i;
        }
    }

    (lowest_index_1, lowest_index_2, lowest1 + lowest2)
}

pub fn lr(node: Node, instance: &Instance, upperbound: f64) -> Node {
    const MAX_ITER: usize = 30;
    const MIN_EPS: f64 = 1e-5;

    let mut best_node = node;
    let mut lambdas = best_node.lambdas.clone();

    // Root node case
    if lambdas.is_empty() {
        lambdas = vec![0.0; instance.dimension];
    }

    let mut best_edges = Vec::new();

    let mut iter_not_improved = 0;
    let mut eps = 1.0;
    while eps > MIN_EPS {
        // Solve MST without the first node
        let (mut cost, mut edges) = mst(
            build_priority_queue(instance, &lambdas, &best_node.forbidden_arcs),
            instance.dimension,
        );
        // Adjust cost
        cost += 2.0 * lambdas.iter().sum::<f64>();

        // Reinsert first node into solution
        let (first, second, added_cost) =
            closest_to_first_node(instance, &lambdas, &best_node.forbidden_arcs);
        edges[0].push(first);
        edges[first].push(0);
        edges[0].push(second);
        edges[second].push(0);
        cost += added_cost;

        let subgradients = edges
            .iter()
            .map(|x| 2 - x.len() as isize)
            .collect::<Vec<isize>>();
        let sum_subgradient = subgradients.iter().map(|x| (x * x) as usize).sum::<usize>();
        // Assign feasible solution and end algorithm
        if sum_subgradient == 0 {
            best_node.value = cost;
            best_node.solution = Some(edges);
            break;
        }

        // Check if there is an improvement
        if cost > best_node.value {
            best_node.value = cost;
            best_node.lambdas = lambdas.clone();
            best_edges = edges;
            iter_not_improved = 0;
        } else {
            iter_not_improved += 1;
            if iter_not_improved >= MAX_ITER {
                iter_not_improved = 0;
                eps /= 2.0;
            }
        }

        let mi = (eps * (upperbound - cost)) / sum_subgradient as f64;

        for (index, lambda) in lambdas.iter_mut().enumerate() {
            *lambda += mi * subgradients[index] as f64;
        }

        // Assuming that the feasible solution cost is integer
        // When this cost is higher than upperbound - 1 (with some slack)
        // there will be no better solution than the current upperbound
        if upperbound - cost < 1.0 - 1e-2 {
            // this will also cause a problem when the upperbound in the user input is the optimal value
            // this optimization will make so that the optimal value can't be proven
            best_node.value = f64::INFINITY;
            return best_node;
        }
    }

    best_node.ban_from_child = if best_node.solution.is_none() {
        // if best edges is empty at this point something went wrong
        // anyways if the code panics at this point it's mostly likely that the upperbound is way
        // higher than the actual optimal value, mostly you would want to treat this instead of unwraping
        Some(
            best_edges
                .into_iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
                .unwrap(),
        )
    } else {
        None
    };

    best_node
}
