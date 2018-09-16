use grammar_parser::XcodebuildGrammarParser;

pub enum ParserResult {
    Command(String),
    Continue,
    NoMatch,
}

pub struct ParserError {}

pub trait Parser {
    fn read_line(&mut self, line: &str) -> Result<ParserResult, ParserError>;
}

pub struct XcodebuildParser {
    buffer: String,
}

impl XcodebuildParser {
    pub fn new() -> XcodebuildParser {
        XcodebuildParser {
            buffer: String::new(),
        }
    }

    fn parse_buffer(&mut self) -> Result<ParserResult, ParserError> {
        match XcodebuildGrammarParser::parse_input(self.buffer.as_str()) {
            Ok(_garbage) => Ok(ParserResult::Command("My Command".to_string())),
            Err(e) => {
                println!("{}", e);
                Ok(ParserResult::Continue)
            }
        }
    }
}

impl Parser for XcodebuildParser {
    fn read_line(&mut self, line: &str) -> Result<ParserResult, ParserError> {
        self.buffer.push_str(line);
        self.parse_buffer()
    }
}
