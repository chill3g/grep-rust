use std::{env::{self}, error::Error, fs, process};
use grep::{search, search_case_insensitive};

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("You need to provide a query and a file name!");
        }
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query: args[1].clone(), file_path: args[2].clone(), ignore_case })
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content =  fs::read_to_string(config.file_path.clone())?;
    
    // let content = match content_result {
    //     Ok(c) => c,
    //     Err(err) => {
    //         println!("Error reading the file {}, {err}", config.file_path);
    //         process::exit(1);
    //     }
    // };
    let lines = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };
    if lines.len() == 0 {
        println!("No result!");
    }
    for line in lines {
        println!("{line}");
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for \"{}\" in file {}", config.query, config.file_path);

    if let Err(e) = run(&config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
