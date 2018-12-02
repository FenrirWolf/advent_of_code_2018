use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = fs::read_to_string("input.txt").unwrap();

    let ids: Vec<&str> = input.lines().collect();
    let mut char_counts: Vec<HashMap<char, i32>> = Vec::new();
    let mut twos = 0;
    let mut threes = 0;

    // loop over the IDs
    for (idx, id) in ids.iter().enumerate() {
        char_counts.push(HashMap::new());

        // count how many times each character occurs in an ID
        for ch in id.chars() {
            let count = char_counts[idx].entry(ch).or_insert(0);
            *count += 1;
        }

        // count how many IDs contain two or three of the same characters
        if let Some(&2) = char_counts[idx].values().find(|&&val| val == 2) {
            twos += 1;
        }
        if let Some(&3) = char_counts[idx].values().find(|&&val| val == 3) {
            threes += 1;
        }
    }

    // now compute the checksum
    let checksum = twos * threes;

    println!("{}", checksum);
}

fn part2() {
    let input = fs::read_to_string("input.txt").unwrap();
    
    let mut ids: Vec<&str> = input.lines().collect();
    let mut solution = String::new();

    // Sort the vec then iterate over neighboring IDs in the vector
    ids.sort();
    for windows in ids.windows(2) {
        let mut diff_count = 0;
        let mut diff_idx = 0; 

        // iterate over each character in the neighboring IDs, then count the number
        // and position of characters that differ between them
        for (idx, (ch1, ch2)) in windows[0].chars().zip(windows[1].chars()).enumerate() {
            if ch1 != ch2 {
                diff_count += 1;
                diff_idx = idx;
            }
        }

        // if only one character differs between them, then we've found the right pair
        // of strings and can use our computed index to get a string containing only
        // the chars they hold in common
        if diff_count == 1 {
            solution = windows[0].to_string();
            solution.remove(diff_idx);
            break;
        }
    }

    println!("{}", solution);
}
