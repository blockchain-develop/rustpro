use std::env;
use std::process;

fn main() {
    let config = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        println!("problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("searching {} in file {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("applicaton error: {e}");
        process::exit(1);
    }
}
