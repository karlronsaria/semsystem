use atty::Stream;
use std::io::{self, BufRead};

#[allow(unused_imports)]
use clap::{Parser, Args, Subcommand};

use crate::myquery;

/// Command Me!
#[derive(Parser, Debug)]
#[command(author, version, about = "todo: Um, Hi! 🙂")]
pub struct Cli {
    /// todo
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// todo
#[derive(Subcommand, Debug)]
pub enum Command {
    /// todo
    Search(Query),

    /// todo
    #[command(subcommand)]
    Item(GetItem),

    /// todo
    Tag {
        names: Vec<String>,
    },

    /// todo
    Date {
        #[arg(value_parser = myquery::parse_naive_date_time)]
        r#in: Option<Vec<chrono::NaiveDateTime>>,

        #[arg(long, value_parser = myquery::parse_naive_date_time)]
        before: Option<chrono::NaiveDateTime>,

        #[arg(long, value_parser = myquery::parse_naive_date_time)]
        after: Option<chrono::NaiveDateTime>,

        #[arg(long, value_parser = myquery::parse_naive_date_time)]
        atleast: Option<chrono::NaiveDateTime>,

        #[arg(long, value_parser = myquery::parse_naive_date_time)]
        atmost: Option<chrono::NaiveDateTime>,
    },
}

/// todo
#[derive(Subcommand, Debug)]
pub enum GetItem {
    /// todo
    Id { id: i32 },

    /// todo
    Name { name: String },
}

/// todo
#[derive(Args, Debug)]
pub struct Query {
    /// todo
    pub strings: Vec<String>,
}

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

    Ok(lines)
}

