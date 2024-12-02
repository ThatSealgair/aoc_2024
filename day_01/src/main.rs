use std::fs::File;
use std::io::env;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    NoFilename,
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

fn main() {
    let filename = env::args().nth(1).ok_or(Error::NoFilename)?;
    let path = Path::new(&filename);
    let (left_set, right_set) = process_two_columns(path)?;

    println!("Part one: TODO");
    println!("Part two: TODO");
    Ok(());
}
