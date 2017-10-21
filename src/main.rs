extern crate ansi_term;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Lines;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use std::iter::Iterator;
use ansi_term::ANSIString;

#[derive(Debug)]
struct Config {
    query: String,
    filename: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("{}  <search string> <file name>", &args[0]);
    }
    let config = parse_config(&args);
    for (i, line) in file(&config).enumerate() {
        let l = line.unwrap();
        if l.contains(&config.query) {
            println!(
                "line {}  - {}",
                i + 1,
                color_tokens(tokens(&config.query, &l), &config.query)
            );
        }
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

fn file(config: &Config) -> Lines<BufReader<File>> {
    let f = File::open(&config.filename).expect("file not found!");
    let f = BufReader::new(f);
    f.lines()
}

fn tokens<'a>(query: &'a str, line: &'a str) -> Vec<&'a str> {
    let v = line.split(query).collect();
    v
}

fn color_tokens(tokens: Vec<&str>, query: &str) -> String {
    let v: Vec<ANSIString> = tokens.iter().map(|t| Yellow.paint(*t)).collect();
    let v: String = v.iter()
        .map(|t| format!("{}", t))
        .collect::<Vec<String>>()
        .join(&format!("{}", Red.paint(query)).to_string()[..]);
    return v;
}
