extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate regex;

mod parsers;
mod tag;

use failure::Error;
use parsers::TagParser;

fn run() -> Result<(), Error> {
    let parser = parsers::detect("filename.js");
    let input_file = r"
        const stuff = require('hello');
        const x = {};
        const hello = 123;
        ";
    let tags = parser.parse_tags(input_file)?;
    println!("{}", tag::stringify_tags(&tags));

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}
