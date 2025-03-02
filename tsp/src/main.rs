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
    let s = ils(50, 150, &instance);
    println!("{:?}", s);
}
