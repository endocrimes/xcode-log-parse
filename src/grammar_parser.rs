use pest::{iterators::Pairs, Error, Parser};

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("xcodebuild.pest"); // relative to this file

#[derive(Parser)]
#[grammar = "xcodebuild.pest"]
pub struct XcodebuildGrammarParser;

impl XcodebuildGrammarParser {
    pub fn parse_input(input: &str) -> Result<Pairs<'_, Rule>, Error<Rule>> {
        XcodebuildGrammarParser::parse(Rule::thing_we_care_about, input)
    }
}
