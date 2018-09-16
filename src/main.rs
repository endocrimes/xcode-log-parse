extern crate clap;
use clap::{App, Arg};
use std::io::{self, BufRead};

pub struct ParserResult {}
pub struct ParserError {}

pub trait Parser {
    fn read_line(line: &str) -> Result<ParserResult, ParserError>;
}

pub trait Formatter {
    fn format(&mut self, r: ParserResult);
}

fn main() {
    let matches = App::new("xcode-log-parse")
        .version("0.1.0")
        .about("Parse, filter, and output Xcode build logs.")
        .arg(Arg::with_name("formatter").default_value("plain").help(""))
        .get_matches();

    let formatter_name = matches.value_of("formatter").unwrap();
    let _ = match resolve_formatter(formatter_name) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}: {}", e, formatter_name)
    };

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}

fn resolve_formatter(name: &str) -> Result<impl Formatter, &'static str> {
    match name {
        "plain" => Ok(PlainTextFormatter::new()),
        _ => Err("Unknown formatter type"),
    }
}

pub struct PlainTextFormatter {
}

impl PlainTextFormatter {
    pub fn new() -> PlainTextFormatter {
        PlainTextFormatter{}
    }
}

impl Formatter for PlainTextFormatter {
    fn format(&mut self, _r: ParserResult) {
    }
}
