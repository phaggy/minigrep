use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        // config gets transferred to run
        eprintln!("\ncouldnt readfile: {}", err);
        process::exit(1);
    };
}
