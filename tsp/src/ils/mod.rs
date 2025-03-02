mod construction;
mod local_search;
mod perturbation;

use crate::instance_reader::Instance;
use crate::solution::Solution;
use construction::construction;
use local_search::local_search;
use perturbation::perturbation;

pub fn ils(max_iter: usize, max_iter_ils: usize, instance: &Instance) -> Solution {
    let mut best = Solution {
        value: usize::MAX,
        ..Default::default()
    };

    for _ in 0..max_iter {
        let mut s = construction(instance);
        let mut inner_best = s.clone();

        let mut iter_ils = 1;
        while iter_ils <= max_iter_ils {
            local_search(&mut s, instance);

            if s.value < inner_best.value {
                inner_best = s.clone();
                iter_ils = 0;
            }

            s = perturbation(inner_best.clone(), instance);
            iter_ils += 1;
        }

        if inner_best.value < best.value {
            best = inner_best;
        }
    }

    best
}
