use atty::Stream;

#[allow(unused_imports)]
use clap::{Parser, Args, Subcommand};

use std::io::{self, BufRead};

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "todo: Um, Hi! 🙂")]

/// Command Me!
pub struct MyCli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// todo
    Add,

    /// todo
    Remove,
}

// #[derive(Args, Debug)]
// pub struct What {
//     string: Option<String>,
// }

pub fn load_stdin() -> io::Result<Vec<String>> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }

    let mut lines: Vec<String> = vec![];
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        lines.push(line);
    }

    return Ok(lines);
}





