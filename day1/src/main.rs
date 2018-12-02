use std::collections::HashSet;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    part_one()?;
    part_two()?;
    Ok(())
}

fn part_one() -> Result<(), Box<dyn Error>> {
    let mut frequency = 0;

    let input = fs::read_to_string("input.txt")?;

    for line in input.lines() {
        frequency += line.trim().parse::<i32>()?
    }

    println!("{}", frequency);

    Ok(())
}

fn part_two() -> Result<(), Box<dyn Error>> {
    let mut frequency = 0;
    let mut frequency_map = HashSet::new();

    let input = fs::read_to_string("input.txt")?;

    for line in input.lines().cycle() {
        frequency += line.parse::<i32>()?;
        if !frequency_map.insert(frequency) {
            break;
        }
    }

    println!("{}", frequency);

    Ok(())
}
