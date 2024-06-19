use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_config(&args);

    let contents =
        fs::read_to_string(config.file_path).expect("Error: file is corrupt or do not exist");
    println!("With text: {contents}");
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
