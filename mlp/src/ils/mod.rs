mod construction;
mod local_search;
mod perturbation;
pub mod subsequence;

use crate::solution::Solution;
use construction::construction;
use instance_reader::Instance;
use local_search::local_search;
use perturbation::perturbation;

use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

pub fn ils(max_iter: u32, max_iter_ils: u32, num_threads: usize, instance: &Instance) -> Solution {
    let best = Mutex::new(Solution::new(Vec::new(), u32::MAX, instance));

    let iter = AtomicU32::new(0); // Atomic counter of finished iterations
    let num_threads = num_threads.min(max_iter as usize); // Don't create more threads than iterations

    let ils_lambda = || {
        loop {
            let i = iter.fetch_add(1, Ordering::Relaxed);
            if i >= max_iter {
                break;
            }

            let mut s = construction(instance);
            let mut inner_best = s.clone();
            let mut iter_ils = 1;

            while iter_ils <= max_iter_ils {
                local_search(&mut s);

                if s.value < inner_best.value {
                    inner_best = s.clone();
                    iter_ils = 0;
                }

                s = perturbation(inner_best.clone());
                iter_ils += 1;
            }

            let mut global_best = best.lock().unwrap();
            if inner_best.value < global_best.value {
                *global_best = inner_best;
            }
        }
    };

    thread::scope(|scope| {
        for _ in 0..num_threads {
            scope.spawn(ils_lambda);
        }
    });

    best.into_inner().unwrap()
}
