use std::error::Error;

pub fn process_input_stage_one(input: &str) -> Result<i32, Box<dyn Error>> {
    let (left, right) = parse_input(input)?;
    Ok(total_distance(left, right))
}

pub fn parse_input(input: &str) -> Result<(Vec<i32>, Vec<i32>), Box<dyn Error>> {
    let parse_line = |line: &str| -> Result<(i32, i32), Box<dyn Error>> {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        if numbers.len() != 2 {
            return Err("Expected each line to contain exactly two numbers".into());
        }

        Ok((numbers[0], numbers[1]))
    };

    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_line)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .unzip();

    Ok((left, right))
}

pub fn total_distance(mut left: Vec<i32>, mut right: Vec<i32>) -> i32 {
    left.sort_unstable();
    right.sort_unstable();

    return left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
}
