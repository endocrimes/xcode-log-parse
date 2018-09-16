use common::ParserResult;

pub trait Formatter {
    fn format(&mut self, r: ParserResult);
}

pub struct PlainTextFormatter {}

impl PlainTextFormatter {
    pub fn new() -> PlainTextFormatter {
        PlainTextFormatter {}
    }
}

impl Formatter for PlainTextFormatter {
    fn format(&mut self, r: ParserResult) {
        match r {
            ParserResult::Command(name) => println!("- {}", name),
            ParserResult::Continue => println!("Continue"),
            ParserResult::NoMatch => println!("NoMatch"),
        }
    }
}
