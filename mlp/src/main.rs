mod ils;
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

    let num_threads = std::thread::available_parallelism().unwrap().get();

    println!("{}", instance.name);
    let s = ils(
        10,
        std::cmp::min(100, instance.dimension) as u32,
        num_threads,
        &instance,
    );
    for v in s.sequence.iter().take(s.sequence.len() - 1) {
        print!("{v} -> ");
    }
    println!("{}", s.sequence.last().unwrap());

    println!("Cost: {}", s.value);
}
