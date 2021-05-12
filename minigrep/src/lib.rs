use std::{error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String
}


impl Config {
    // why is this String and not str
    pub fn new(args: &[String]) -> Result<Config, &str> {
        
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


// returns unit type, or a type that implmenets Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    // ending with ?  replaces .expect and simply returns something that impls Error
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
