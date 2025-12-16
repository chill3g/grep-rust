use std::{env::{self}, fs, process};

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

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for \"{}\" in file {}", config.query, config.file_path);

    let content_result =  fs::read_to_string(config.file_path.clone());
    
    let content = match content_result {
        Ok(c) => c,
        Err(err) => panic!("problem opening the file {}: {err:?}", config.file_path)
    };

    println!("With text:\n{content}");
}
