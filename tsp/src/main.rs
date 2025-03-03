mod ils;
mod instance_reader;
mod solution;

use ils::ils;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Not enough arguments, you need to pass the path to an instance");
        eprintln!("Use cargo run /path/to/instance or ./path/to/bin /path/to/instance");
        return;
    }

    let instance = instance_reader::read_data(&args[1]);

    let max_iter_ils = if instance.dimension < 150 {
        instance.dimension
    } else {
        instance.dimension / 2
    };

    println!("{}", instance.name);
    let s = ils(50, max_iter_ils, &instance);
    println!("Solution: ");
    for v in s.sequence.iter().take(s.sequence.len() - 1) {
        print!("{v} -> ");
    }
    println!("{}", s.sequence.last().unwrap());

    println!("Cost: {}", s.value);
}
