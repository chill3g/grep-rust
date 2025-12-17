use std::{env::{self}, error::Error, f32::consts::E, fs, process};
use grep::search;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("You need to provide a query and a file name!");
        }
        Ok(Config { query: args[1].clone(), file_path: args[2].clone() })
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
    let lines = search(&config.query, &content);
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
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for \"{}\" in file {}", config.query, config.file_path);

    if let Err(e) = run(&config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
