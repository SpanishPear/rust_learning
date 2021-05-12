use std::env;
use std::fs;

fn main() {
    // get env::args (a collection) and collect into an iterable of strings
    let args: Vec<String> = env::args().collect();

    // cla's start at 0 with program name
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file!");

    println!("With text:\n{}", contents);
}
