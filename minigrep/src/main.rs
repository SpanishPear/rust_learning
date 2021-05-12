use std::env;
use std::fs;
use std::process;

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

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file!");

    println!("With text:\n{}", contents);
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
