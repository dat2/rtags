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

fn main() {

}
