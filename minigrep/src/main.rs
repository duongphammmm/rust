use std::env;
use std::process;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in file {}", config.query, config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Problem reading file"); // return a Result<String>

    println!("With text\n{}", contents);
}

struct Config<'a> {
    query: &'a str,
    filename: &'a str,
}

impl Config<'_> {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = &args[1];
        let filename = &args[2];
        Ok(Config { query, filename })
    }
}
