mod construction;
mod local_search;
mod perturbation;
mod subsequence;

use crate::solution::Solution;
use construction::construction;
use instance_reader::Instance;
use local_search::local_search;
use perturbation::perturbation;
use subsequence::Subsequence;

pub fn ils(max_iter: usize, max_iter_ils: usize, instance: &Instance) -> Solution {
    let mut best = Solution {
        value: usize::MAX,
        ..Default::default()
    };
    let mut subseq_matrix =
        vec![vec![Subsequence::default(); instance.dimension + 1]; instance.dimension + 1];

    for _ in 0..max_iter {
        let mut s = construction(&mut subseq_matrix, instance);

        let mut inner_best = s.clone();

        let mut iter_ils = 1;
        while iter_ils <= max_iter_ils {
            local_search(&mut s, &mut subseq_matrix, instance);

            if s.value < inner_best.value {
                inner_best = s.clone();
                iter_ils = 0;
            }

            s = perturbation(inner_best.clone(), &mut subseq_matrix, instance);
            iter_ils += 1;
        }

        if inner_best.value < best.value {
            best = inner_best;
        }
    }

    best
}
