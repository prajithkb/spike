extern crate ansi_term;

use std::env;
use mini_grep::Config;
use std::process;

mod mini_grep;

fn main() {
    let args: Vec<String> = env::args().collect();
    App::run(Config::new(&args), &args[0]);
}

struct App {}

impl App {
    fn run(c: Result<Config, &'static str>, file_name: &String) {
        let config = c.unwrap_or_else(|e| {
            println!("Problem parsing arguments: {} - exception [{}]", file_name, e);
            process::exit(1);
        });
        config.search();
    }
}


