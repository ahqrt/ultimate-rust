#![allow(dead_code)]
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application errror: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n {}", contents);
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

// create a fn to parse config
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
}

// but to associate with the struct is more idiomatic
impl Config {
    // it's much better to return a Result rather than panic

    // typically new functions never to fail
    // fn new(args: &[String]) -> Self {
    //     if args.len() < 3 {
    //         panic!("not enough arguments")
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    //     Self { query, file_path }
    // }

    // so we create new function build
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
