mod instance_reader;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    instance_reader::read_data(&args[1]);
}
