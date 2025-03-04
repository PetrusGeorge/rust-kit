mod bnb;
mod solution;

use bnb::{SearchMode, bnb};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Not enough arguments, you need to pass the path to an instance");
        eprintln!(
            "Use cargo run /path/to/instance SEARCH_TYPE or ./path/to/bin /path/to/instance SEARCH_TYPE"
        );
        eprintln!("Search type must be BFS or DFS and is optional");

        return;
    }

    let instance = instance_reader::read_data(&args[1]);
    let search_type_str: String = args.get(2).cloned().unwrap_or(String::from("DFS"));
    let search_mode = if search_type_str.eq("BFS") {
        SearchMode::Bfs
    } else {
        SearchMode::Dfs
    };

    let s = bnb(&instance, search_mode);
    println!("Solution: ");
    for v in s.sequence.iter().take(s.sequence.len() - 1) {
        print!("{v} -> ");
    }
    println!("{}", s.sequence.last().unwrap());

    println!("Cost: {}", s.value);
}
