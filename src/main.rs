extern crate ansi_term;

use std::env;
use mini_grep::Config;
use std::process;

mod mini_grep;

fn main() {
    App::run(Config::new( env::args()));
}

struct App {}

impl App {
    fn run(c: Result<Config, &'static str>) {
        let config = c.unwrap_or_else(|e| {
            println!("Problem parsing arguments: {}",  e);
            process::exit(1);
        });
        config.search();
    }
}


