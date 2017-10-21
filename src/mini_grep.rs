use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::Lines;
use ansi_term::Colour::Red;
use ansi_term::Colour::Yellow;
use std::iter::Iterator;
use ansi_term::ANSIString;
use std::env::Args;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        Ok(Config { query, filename })
    }
    fn file(&self) -> Lines<BufReader<File>> {
        let f = File::open(&self.filename)
            .expect(&format!("file [{}] not found! ", Yellow.paint(&self.filename[..])).to_string()[..]);
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
        self.file()
            .map(|l| l.unwrap())
            .enumerate()
            .filter(|&(_, ref line)| line.contains(&self.query[..]))
            .for_each(|(i, line)| println!(
                "line {}  - {}",
                i + 1,
                self.color_tokens(self.tokens(&line))
            ));
    }
}




