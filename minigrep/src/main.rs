#![allow(dead_code)]
use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application errror: {e}");
        process::exit(1);
    }
}
