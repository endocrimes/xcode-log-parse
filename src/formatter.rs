use common::{ParserEvent, ParserResult};
use std::path::Path;

pub trait Formatter {
    fn format(&mut self, r: ParserResult);
}

pub struct PlainTextFormatter {
    level: usize,
}

impl PlainTextFormatter {
    pub fn new() -> PlainTextFormatter {
        PlainTextFormatter {
            level: 0,
        }
    }

    fn print_event(&mut self, event: ParserEvent) {
        match event {
            ParserEvent::Message(message_type, contents) => {
                let type_emoji = match message_type.as_ref() {
                    "note" => "📝 " ,
                    "warning" => "⚠️ ",
                    "error" => "❌ ",
                    _ => "",
                };
                println!("{}{}{}: {}", self.indentation(), type_emoji, message_type, contents);
            },
            ParserEvent::Status(name, outcome, Some(duration)) => println!("{}{} {} [{}]\n", self.indentation(), name, outcome, duration),
            ParserEvent::Status(name, outcome, None) => println!("{}{} {}\n", self.indentation(), name, outcome),
            ParserEvent::BeginCommand(name, args) => {
                let pretty_name = match name.as_ref() {
                    "CompileC" | "CompileXIB" => "Compiling",
                    "CpResource" | "CopyStringsFile" => "Copying",
                    "ProcessPCH" => "Precompiling",
                    "Ld" => "Linking",
                    _ => return,
                };
                let first_arg = args.first().unwrap();
                let first_arg = Path::new(first_arg).file_name().unwrap().to_str().unwrap();
                self.level += 1;
                println!("{}{} {}", self.indentation(), pretty_name, first_arg);
            },
            ParserEvent::EndCommand(_, _) => self.level -= 1,
            ParserEvent::BeginSubCommand(_, _) => self.level += 1,
            ParserEvent::EndSubCommand(_, _) => self.level -= 1,
        }
    }

    fn indentation(&self) -> String {
        "  ".repeat(self.level - self.level)
    }
}

impl Formatter for PlainTextFormatter {
    fn format(&mut self, r: ParserResult) {
        match r {
            ParserResult::Commands(names) => {
                for name in names {
                    self.print_event(name);
                }
            }
            ParserResult::Continue => {},
            ParserResult::NoMatch => println!("NoMatch"),
        }
    }
}
