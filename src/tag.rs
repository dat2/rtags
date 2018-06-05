use std::fmt;

#[derive(Debug)]
pub enum ExCmd {
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
pub struct Tag {
    pub name: String,
    pub filename: String,
    pub excmd: ExCmd,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.name, self.filename, self.excmd)
    }
}

pub fn stringify_tags(tags: &[Tag]) -> String {
    let mut result = String::new();
    for tag in tags {
        result.push_str(&format!("{}\n", tag));
    }
    result
}
