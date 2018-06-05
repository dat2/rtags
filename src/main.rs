use std::fmt;

#[derive(Debug)]
enum ExCmd {
    RegexPattern(String),
    QPattern(String),
    LineNo(usize),
}

impl fmt::Display for ExCmd {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExCmd::RegexPattern(p) => write!(f, "/{}/;\"", p),
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
    fn parse_tags(&self, s: &str) -> Vec<Tag>;
}

struct JavaScriptTagParser;

impl JavaScriptTagParser {
    fn new() -> JavaScriptTagParser {
        JavaScriptTagParser
    }
}

impl TagParser for JavaScriptTagParser {
    fn parse_tags(&self, s: &str) -> Vec<Tag> {
        Vec::new()
    }
}

fn stringify_tags(tags: &[Tag]) -> String {
    let mut result = String::new();
    for tag in tags {
        result.push_str(&format!("{}\n", tag));
    }
    result
}

fn main() {
    let parser = JavaScriptTagParser::new();
    let input_file = r"
        const stuff = require('hello');
        let x = {};
        var hello = 123;
        function x() {

        }";
    let tags = parser.parse_tags(input_file);
    println!("{}", stringify_tags(&tags));
}
