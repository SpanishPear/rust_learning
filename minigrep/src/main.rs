use std::env;
use std::fs;
use std::process;
use std::error::Error;

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
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

// returns unit type, or a type that implmenets Error
fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // ending with ?  replaces .expect and simply returns something that impls Error
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}


struct Config {
    query: String,
    filename: String
}


impl Config {
    // why is this String and not str
    fn new(args: &[String]) -> Result<Config, &str> {
        
        if args.len() < 3 {
            return Err("Not enough Arguments")
        }

        //let query = &args[1];
        //let filename = &args[2];
        // to copy strings into struct, we use clone (a la strcpy)
        // means that config now owns the string
        Ok (Config { query: args[1].clone(), filename: args[2].clone() })
    }
}
