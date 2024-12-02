use std::fs::File;
use std::io::env;
use std::io::read_to_string;
use std::io::Result;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let filename = env::args().nth(1).ok_or("Usage: day_one [file_name]")?;
    let input = read_to_string(filename)?;
    let result = process_input_stage_one(&input);

    println!("Part one: {:?}", result);
    println!("Part two: TODO");
    Ok(());
}
