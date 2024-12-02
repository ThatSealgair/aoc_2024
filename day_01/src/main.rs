use day_01::process_input_stage_one;
use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();

    let expected_arguments: usize = 2;
    let filename_position: usize = 1;

    if arguments.len() != expected_arguments {
        panic!("Useage: day_one [file_name]");
    }

    let filename = &arguments[filename_position];
    let input = fs::read_to_string(filename)?;
    let result = process_input_stage_one(&input);

    println!("Part one: {:?}", result);
    println!("Part two: TODO");

    return Ok(());
}
