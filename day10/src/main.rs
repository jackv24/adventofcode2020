use std::io::{prelude::*, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse numbers from file into vector
    let mut jolt_list = Vec::new();
    for line in reader.lines() {
        let num = line?.parse::<u64>()?;
        jolt_list.push(num);
    }
    jolt_list.push(0);

    // Sort list so we can easily calculate differences from previous elements
    jolt_list.sort();

    // Add device on to end (always 3 jolts higher than last)
    jolt_list.push(jolt_list.last().unwrap() + 3);

    // Part 1
    let mut jolt_differences = Vec::new();

    // All subsequent differences are from their previous
    for i in 1..jolt_list.len() {
        let jolt = jolt_list[i];
        let prev_jolt = jolt_list[i - 1];
        jolt_differences.push((jolt, jolt - prev_jolt));
    }

    let jolt_1_diffs = jolt_differences.iter().filter(|j| j.1 == 1).count();
    let jolt_3_diffs = jolt_differences.iter().filter(|j| j.1 == 3).count();
    println!(
        "1-jolt diffs: {}, 3-jolt diffs: {}, Multiplied: {}",
        jolt_1_diffs,
        jolt_3_diffs,
        jolt_1_diffs * jolt_3_diffs
    );

    // Part 2
    // Need to use u64 as the number of permutations is huge
    let mut permutations = vec![0 as u64; jolt_list.len()];
    permutations[0] = 1;

    // Still not entirely sure why this works? ಥ_ಥ https://www.reddit.com/r/adventofcode/comments/ka8z8x/2020_day_10_solutions/gf9f6fv/
    for i in 1..jolt_list.len() {
        for (j, _) in jolt_list[..i]
            .iter()
            .rev()
            .take_while(|&v| v + 3 >= jolt_list[i])
            .enumerate()
        {
            permutations[i] += permutations[i - (j + 1)];
        }
    }

    println!("Permutations: {}", permutations.last().unwrap());

    Ok(())
}
