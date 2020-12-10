use std::io::{prelude::*, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse numbers from file into vector
    let mut jolt_list = Vec::new();
    for line in reader.lines() {
        // Parse num into usize (i32 is too small for these numbers)
        let num = line?.parse::<usize>()?;
        jolt_list.push(num);
    }

    // Sort list so we can easily calculate differences from previous elements
    jolt_list.sort();

    let mut jolt_differences = Vec::new();

    // First difference is from 0
    let first_jolt = jolt_list[0];
    jolt_differences.push((first_jolt, first_jolt));

    // All subsequent differences are from their previous
    for i in 1..jolt_list.len() {
        let jolt = jolt_list[i];
        let prev_jolt = jolt_list[i - 1];
        jolt_differences.push((jolt, jolt - prev_jolt));
    }

    // Device joltage is always 3 above the highest
    let device_joltage = jolt_differences.last().unwrap().0 + 3;
    jolt_differences.push((device_joltage, 3));

    // Part 1
    let jolt_1_diffs = jolt_differences.iter().filter(|j| j.1 == 1).count();
    let jolt_3_diffs = jolt_differences.iter().filter(|j| j.1 == 3).count();
    println!(
        "1-jolt diffs: {}, 3-jolt diffs: {}, Multiplied: {}",
        jolt_1_diffs,
        jolt_3_diffs,
        jolt_1_diffs * jolt_3_diffs
    );

    Ok(())
}
