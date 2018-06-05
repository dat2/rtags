extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use failure::Error;
use regex::Regex;
use std::fmt;

#[derive(Debug)]
enum ExCmd {
    GCmd(String),
    QPattern(String),
    LineNo(usize),
}

impl fmt::Display for ExCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExCmd::GCmd(p) => write!(f, "/{}/;\"", p),
            ExCmd::QPattern(p) => write!(f, "?{}?", p),
            ExCmd::LineNo(line) => write!(f, "{}", line),
        }
    }
}

#[derive(Debug)]
struct Tag {
    name: String,
    filename: String,
    excmd: ExCmd,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.name, self.filename, self.excmd)
    }
}

trait TagParser {
    fn parse_tags(&self, s: &str, filename: &str) -> Result<Vec<Tag>, Error>;
}

struct JavaScriptTagParser;

impl JavaScriptTagParser {
    fn new() -> JavaScriptTagParser {
        JavaScriptTagParser
    }
}

impl TagParser for JavaScriptTagParser {
    fn parse_tags(&self, s: &str, filename: &str) -> Result<Vec<Tag>, Error> {
        lazy_static! {
            static ref JS_RE: Regex =
                Regex::new("(?P<line>const\\s+(?P<name>\\w+)\\s+=.+)").unwrap();
        }

        let mut result = Vec::new();
        for c in JS_RE.captures_iter(s) {
            result.push(Tag {
                name: c.name("name").unwrap().as_str().to_owned(),
                filename: filename.to_owned(),
                excmd: ExCmd::GCmd(format!("^{}$", c.name("line").unwrap().as_str())),
            });
        }
        Ok(result)
    }
}

fn stringify_tags(tags: &[Tag]) -> String {
    let mut result = String::new();
    for tag in tags {
        result.push_str(&format!("{}\n", tag));
    }
    result
}

fn run() -> Result<(), Error> {
    let parser = JavaScriptTagParser::new();
    let input_file = r"
        const stuff = require('hello');
        const x = {};
        const hello = 123;
        ";
    let tags = parser.parse_tags(input_file, "main.js")?;
    println!("{}", stringify_tags(&tags));

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
