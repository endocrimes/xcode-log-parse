extern crate clap;
use clap::{App, Arg};
use std::io::{self, BufRead, Read};

pub struct ParserResult {}
pub struct ParserError {}

pub trait ParserTrait {
    fn read_line(line: &str) -> Result<ParserResult, ParserError>;
}

pub trait Formatter {
    fn format(&mut self, r: ParserResult);
}

fn main() {
    let matches = App::new("xcode-log-parse")
        .version("0.1.0")
        .about("Parse, filter, and output Xcode build logs.")
        .arg(Arg::with_name("do-foo").long("do-foo").help(""))
        .arg(Arg::with_name("formatter").default_value("plain").help(""))
        .get_matches();

    let formatter_name = matches.value_of("formatter").unwrap();
    let _ = match resolve_formatter(formatter_name) {
        Ok(f) => f,
        Err(e) => panic!("Error: {}: {}", e, formatter_name)
    };

    match matches.occurrences_of("do-foo") {
        0 => {},
        _ => { do_foo(); return },
    }

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

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("xcodebuild.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "xcodebuild.pest"]
struct XcodebuildParser;

fn do_foo() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
 
    let pairs = XcodebuildParser::parse(Rule::entire, &input).unwrap_or_else(|e| panic!("{}", e));

    // Because ident_list is silent, the iterator will contain idents
    for full_command in pairs {
        match full_command.as_rule() {
            Rule::full_command => println!("\n\n***full command***\n\n"),
            Rule::build_status => println!("\n\n***build status***\n\n"),
            Rule::message => println!("\n\n***top-level message***\n\n"),
            _ => unreachable!("only full_command expected, got {:?}", full_command.as_rule())
        }

        for pair in full_command.into_inner() {
            // A pair is a combination of the rule which matched and a span of input
            println!("Rule:    {:?}", pair.as_rule());
            println!("Span:    {:?}", pair.clone().into_span());
            println!("Text:    {:?}", pair.clone().into_span().as_str());

            // A pair can be converted to an iterator of the tokens which make it up:
            for inner_pair in pair.into_inner() {
                println!("\t{:?}: {}", inner_pair.as_rule(), inner_pair.into_span().as_str());
            }
        }
    }
}
