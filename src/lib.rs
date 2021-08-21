use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub pattern: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments, need two where 1st arg is pattern to search from and 2nd is filename");
        }
        return Ok(Config {
            filename: args[2].clone(),
            pattern: args[1].clone(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.filename)?;
    for line in search(&config.pattern, &file_contents) {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
