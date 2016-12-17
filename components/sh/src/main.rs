#[macro_use]
extern crate nom;
extern crate getopts;

use std::fs::File;
use std::process::exit;

mod opts;
mod parser;
mod chars;
mod lexer;
mod tokens;

fn main() {
    let (source, opts) = opts::from_cmdline_args();
    execute(source, opts);
    exit(0);
}

pub fn execute(source: opts::Source, opts: opts::Opts) {
    match source {
        opts::Source::Interactive(s) => {
    },
    opts::Source::File(s) => {
    let mut input = File::open(s).expect("Unable to open named file");
    parser::parse(chars::from(&mut input));
}
opts::Source::StandardInput(s) => {
            },
            opts::Source::CommandString(s) => {
            },
        }
    }
