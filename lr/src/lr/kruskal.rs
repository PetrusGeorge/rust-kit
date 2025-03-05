use ordered_float::OrderedFloat;
use std::{cmp::Ordering, collections::BinaryHeap};

struct Pset {
    pset: Vec<usize>,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Edge {
    cost: OrderedFloat<f64>,
    i: usize,
    j: usize,
}

impl Edge {
    pub fn from(cost: f64, i: usize, j: usize) -> Self {
        Edge {
            cost: OrderedFloat(cost),
            i,
            j,
        }
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.i.cmp(&other.i))
            .then_with(|| self.j.cmp(&other.j))
    }
}

impl Pset {
    fn new(n: usize) -> Self {
        Pset {
            pset: (0..n).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.pset[i] == i {
            i
        } else {
            self.pset[i] = self.find(self.pset[i]);
            self.pset[i]
        }
    }

    fn is_same(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    fn union(&mut self, i: usize, j: usize) {
        let set_i = self.find(i);
        self.pset[set_i] = self.find(j);
    }
}

pub fn mst(mut pq: BinaryHeap<Edge>, n: usize) -> (f64, Vec<Vec<usize>>) {
    let mut cost = 0.0;
    let mut pset = Pset::new(n);
    let mut edges = vec![vec![]; n];

    while !pq.is_empty() {
        let edge = pq.pop().unwrap();
        let (i, j) = (edge.i, edge.j);

        if !pset.is_same(i, j) {
            edges[i].push(j);
            edges[j].push(i);
            pset.union(i, j);
            cost += edge.cost.0;
        }
    }

    (cost, edges)
}
