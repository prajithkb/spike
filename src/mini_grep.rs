use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Lines;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use std::iter::Iterator;
use ansi_term::ANSIString;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {

    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
    fn file(&self) -> Lines<BufReader<File>> {
        let f = File::open(&self.filename).expect("file not found!");
        let f = BufReader::new(f);
        f.lines()
    }
    fn tokens<'a>(&self, line: &'a str) -> Vec<&'a str> {
        let v = line.split(&self.query[..]).collect();
        v
    }

    fn color_tokens(&self, tokens: Vec<&str>) -> String {
        let v: Vec<ANSIString> = tokens.iter().map(|t| Yellow.paint(*t)).collect();
        let v: String = v.iter()
            .map(|t| format!("{}", t))
            .collect::<Vec<String>>()
            .join(&format!("{}", Red.paint(&self.query[..])).to_string()[..]);
        return v;
    }

    pub fn search(&self) {
        for (i, line) in self.file().enumerate() {
            let l = line.unwrap();
            if l.contains(&self.query[..]) {
                println!(
                    "line {}  - {}",
                    i + 1,
                    self.color_tokens(self.tokens(&l))
                );
            }
        }
    }
}




