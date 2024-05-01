use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let cfg = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing args: {err}");
        process::exit(1);
    });

    println!("Searching for {} in {}!", cfg.query, cfg.file_path);
    if let Err(e) = minigrep::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}




