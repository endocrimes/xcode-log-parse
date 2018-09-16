extern crate clap;
extern crate xcode_log_parse;

use clap::{App, Arg};
use std::io::{self, BufRead};

use xcode_log_parse::common::{Parser, XcodebuildParser};
use xcode_log_parse::formatter::{Formatter, PlainTextFormatter};

fn main() {
    let matches = App::new("xcode-log-parse")
        .version("0.1.0")
        .about("Parse, filter, and output Xcode build logs.")
        .arg(Arg::with_name("formatter").default_value("plain").help(""))
        .get_matches();

    let formatter_name = matches.value_of("formatter").unwrap();
    let mut formatter = match resolve_formatter(formatter_name) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}: {}", e, formatter_name),
    };

    let mut parser = XcodebuildParser::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        match parser.read_line(&l) {
            Ok(r) => formatter.format(r),
            Err(_e) => unreachable!(),
        }
    }
}

fn resolve_formatter(name: &str) -> Result<impl Formatter, &'static str> {
    match name {
        "plain" => Ok(PlainTextFormatter::new()),
        _ => Err("Unknown formatter type"),
    }
}
