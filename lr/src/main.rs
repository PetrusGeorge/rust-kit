mod bnb_lr;
mod lr;
mod solution;

use std::env;

use bnb_lr::bnb_lr;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Not enough arguments, you need to pass the path to an instance");
        eprintln!(
            "Use cargo run /path/to/instance UPPERBOUND or ./path/to/bin /path/to/instance UPPERBOUND"
        );
        return;
    }

    let instance = instance_reader::read_data(&args[1]);
    let upperbound = args[2].parse().unwrap();

    let s = bnb_lr(&instance, upperbound);

    if s.is_none() {
        println!("No better solution was found for this upperbound");
        return;
    }

    let s = s.unwrap();

    println!("Solution:");
    for node in s.sequence.iter().take(instance.dimension) {
        print!("{node} -> ");
    }
    println!("{}", s.sequence.last().unwrap());

    println!("Cost: {}", s.value);
}
