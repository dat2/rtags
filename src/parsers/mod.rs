use failure::Error;
use regex::Regex;
use std::path::Path;
use tag::{ExCmd, Tag};

pub trait TagParser {
    fn parse_tags(&self, s: &str) -> Result<Vec<Tag>, Error>;
}

struct NoOpTagParser;

impl TagParser for NoOpTagParser {
    fn parse_tags(&self, _: &str) -> Result<Vec<Tag>, Error> {
        Ok(Vec::new())
    }
}

struct JavaScriptTagParser {
    filename: String,
}

impl JavaScriptTagParser {
    fn new(filename: &str) -> JavaScriptTagParser {
        JavaScriptTagParser {
            filename: filename.to_owned(),
        }
    }
}

impl TagParser for JavaScriptTagParser {
    fn parse_tags(&self, s: &str) -> Result<Vec<Tag>, Error> {
        lazy_static! {
            static ref JS_RE: Regex =
                Regex::new("(?P<line>const\\s+(?P<name>\\w+)\\s+=.+)").unwrap();
        }

        let mut result = Vec::new();
        for c in JS_RE.captures_iter(s) {
            result.push(Tag {
                name: c.name("name").unwrap().as_str().to_owned(),
                filename: self.filename.clone(),
                excmd: ExCmd::GCmd(format!("^{}$", c.name("line").unwrap().as_str())),
            });
        }
        Ok(result)
    }
}

pub fn detect(filename: &str) -> Box<TagParser> {
    let p = Path::new(filename);
    match p.extension() {
        None => Box::new(NoOpTagParser),
        Some(os_str) => match os_str.to_str() {
            Some("js") => Box::new(JavaScriptTagParser::new(filename)),
            _ => Box::new(NoOpTagParser),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_are_parsed_correctly() {
        let expected = vec![Tag {
            name: "stuff".to_string(),
            filename: "main.js".to_string(),
            excmd: ExCmd::GCmd("^const stuff = {};$".to_string()),
        }];
        let parser = JavaScriptTagParser::new("main.js");
        let file_contents = "const stuff = {};";
        let actual = parser.parse_tags(file_contents).unwrap();
        assert_eq!(actual, expected)
    }
}
