use std::env::args;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    println!(
        "Searching for '{}' inside file {}",
        config.query, config.filename
    );

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
