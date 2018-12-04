extern crate lazy_static;
extern crate regex;

use lazy_static::lazy_static;
use regex::Regex;

use std::error::Error;
use std::fs;
use std::collections::HashSet;

// I've never used regex to parse anything and felt like trying it out. please don't crucify
// me <.<
lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"\#(\d+)\s+@\s+(\d+),(\d+):\s+(\d+)x(\d+)"#).unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Claim {
    id: i32,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;

    let mut claims = Vec::with_capacity(1350);

    // parse out the claims with our fancy (yeah right) regex
    for cap in REGEX.captures_iter(&input) {
        claims.push(Claim {
            id: cap[1].parse()?,
            x: cap[2].parse()?,
            y: cap[3].parse()?,
            width: cap[4].parse()?,
            height: cap[5].parse()?,
        })
    }

    solve_it(&claims)
}

fn solve_it(claims: &[Claim]) -> Result<(), Box<dyn Error>> {
    let mut array = [[0; 1000]; 1000];

    // make a bitmap-ish thing that represents how many times each square is claimed
    for claim in claims.iter() {
        for x in claim.x..claim.width + claim.x {
            for y in claim.y..claim.height + claim.y {
                array[x][y] += 1;
            }
        }
    }

    // count how many squares are claimed twice or more
    let overlapping = array.iter().flat_map(|a| a.iter()).filter(|&&x| x >= 2).count();

    println!("Number of overlapping square inches: {}", overlapping);

    // loop through each claim and find the one that has no conflicting claims
    for claim in claims.iter() {
        let mut seen = HashSet::new();

        // tally which numbers we see in the squares for each claim
        for x in claim.x..claim.width + claim.x {
            for y in claim.y..claim.height + claim.y {
                seen.insert(array[x][y]);
            }
        }

        // the unique claim will only have the number `1` in each square
        if seen.len() == 1 && seen.contains(&1) {
            println!("Nonoverlapping claim: #{}", claim.id);
            break;
        }
    }

    Ok(())
}
