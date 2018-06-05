use failure::Error;
use regex::Regex;
use tag::{ExCmd, Tag};

pub trait TagParser {
    fn parse_tags(&self, s: &str) -> Result<Vec<Tag>, Error>;
}

struct JavaScriptTagParser {
    filename: String,
}

impl JavaScriptTagParser {
    fn new(filename: &str) -> JavaScriptTagParser {
        JavaScriptTagParser { filename: filename.to_owned() }
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

pub fn detect(filename: &str) -> impl TagParser {
    JavaScriptTagParser::new(filename)
}
