use std::env;

use std::process;

use minigrep::Config;
fn main() {
    // get env::args (a collection) and collect into an iterable of strings
    let args: Vec<String> = env::args().collect();

    // cla's start at 0 with program name
    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if Let syntax - if run is ann Err of some e, thhen do {}
    // nothing to unwrap so why use unwrap_or_else
    if let Err(e) = minigrep::run(config){
        println!("Application error: {}", e);

        process::exit(1);
    }
}
