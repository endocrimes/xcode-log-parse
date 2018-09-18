use grammar_parser::{XcodebuildGrammarParser, Rule};
use pest::iterators::Pair;
use std::collections::HashSet;

#[derive(Debug)]
pub enum ParserEvent {
    Message(String, String),
    BeginTarget(String),
    BeginCommand(String, Vec<String>),
    BeginSubCommand(String, Vec<String>),
    EndSubCommand(String, Vec<String>),
    EndCommand(String, Vec<String>),
    Status(String, String, Option<String>),
}

pub enum ParserResult {
    Commands(Vec<ParserEvent>),
    Continue,
    NoMatch,
}

pub struct ParserError {}

pub trait Parser {
    fn read_line(&mut self, line: &str) -> Result<ParserResult, ParserError>;
}

pub struct XcodebuildParser {
    buffer: String,
    index: usize,
    targets: HashSet<String>,
}

impl XcodebuildParser {
    pub fn new() -> XcodebuildParser {
        XcodebuildParser {
            buffer: String::new(),
            index: 0,
            targets: HashSet::new(),
        }
    }

    fn parse_buffer(&mut self) -> Result<ParserResult, ParserError> {
        let mut targets = &mut self.targets.clone();
        let result = match XcodebuildGrammarParser::parse_input(&self.buffer[self.index..]) {
            Ok(pairs) => {
                self.index = self.buffer.len();
                Ok(ParserResult::Commands(
                    pairs.flat_map(|pair| self.transform_pair(pair, &mut targets)).collect()
                ))
            },
            Err(_) => Ok(ParserResult::Continue),
        };
        self.targets = targets.clone();

        result
    }

    fn transform_pair(&self, pair: Pair<'_, Rule>, targets: &mut HashSet<String>) -> Vec<ParserEvent> {
        let mut events = vec!();

        match pair.as_rule() {
            Rule::message => {
                events.push(ParserEvent::Message(
                    self.find_first(pair.clone(), Rule::message_type).unwrap_or_else(|| panic!() ),
                    self.find_first(pair.clone(), Rule::message_contents).unwrap_or_else(|| panic!() ),
                ));
            },
            Rule::build_status => {
                events.push(ParserEvent::Status(
                    self.find_first(pair.clone(), Rule::action_name).unwrap_or_else(|| panic!() ),
                    self.find_first(pair.clone(), Rule::build_outcome).unwrap_or_else(|| panic!() ),
                    self.find_first(pair.clone(), Rule::duration),
                ));
            },
            Rule::full_command => {
                for inner_pair in pair.into_inner() {
                    events.append(&mut self.transform_pair(inner_pair, targets));
                }
            },
            Rule::toplevel_command => {
                if let Some(target) = self.find_first(pair.clone(), Rule::target_name) {
                    if targets.insert(target.clone()) {
                        events.push(ParserEvent::BeginTarget(
                            target,
                        ));
                    }
                }
                let pair = pair.into_inner().find(|pair| pair.as_rule() == Rule::command).unwrap();
                events.push(ParserEvent::BeginCommand(
                    self.find_first(pair.clone(), Rule::command_name).unwrap(),
                    self.find_all(pair.clone(), Rule::arg),
                ));
            },
            Rule::nested_command => {
                let pair = pair.into_inner().find(|pair| pair.as_rule() == Rule::command).unwrap();
                events.push(ParserEvent::BeginSubCommand(
                    self.find_first(pair.clone(), Rule::command_name).unwrap(),
                    self.find_all(pair.clone(), Rule::arg),
                ));
            },
            Rule::commenty_bits => {},
            _ => unreachable!("unexpected rule: {:?}", pair.as_rule()),
        };

        events
    }

    fn find_first(&self, pair: Pair<'_, Rule>, rule: Rule) -> Option<String> {
        pair.into_inner().flatten().find(|pair| {
            rule == pair.as_rule()
        }).map(|pair| pair.into_span().as_str().to_string())
    }

    fn find_all(&self, pair: Pair<'_, Rule>, rule: Rule) -> Vec<String> {
        pair.into_inner().flatten().filter(|pair| {
            rule == pair.as_rule()
        }).map(|pair| pair.into_span().as_str().to_string()).collect()
    }
}

impl Parser for XcodebuildParser {
    fn read_line(&mut self, line: &str) -> Result<ParserResult, ParserError> {
        self.buffer.push_str(line);
        self.buffer.push_str("\n");
        self.parse_buffer()
    }
}
