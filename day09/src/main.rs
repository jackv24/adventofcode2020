use std::io::{prelude::*, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Parse numbers from file into vector
    let mut nums = Vec::new();
    for line in reader.lines() {
        // Parse num into usize (i32 is too small for these numbers)
        let num = line?.parse::<usize>()?;
        nums.push(num);
    }

    // Significant help from https://github.com/ChevyRay/advent_of_code_2020/blob/main/src/bin/day9.rs
    // for converting from complicated loops to this

    // Part 1
    let preamble = 25;
    let (invalid_index, invalid_num) = nums
        .iter()
        .enumerate()
        // Skip over the preamble since we'll be searching that far back
        .skip(preamble)
        .find(|&(i, num)| {
            nums[i - preamble..i]
                .iter()
                // Need to use flat_map since we end with another map in this closure
                .flat_map(|a| nums[i - preamble..i].iter().map(move |b| (a, b)))
                .find(|&(a, b)| a + b == *num)
                .is_none()
        })
        .unwrap();

    println!(
        "Found invalid num {} at index {}",
        invalid_num, invalid_index
    );

    // Part 2
    let (i, j) = (0..nums.len())
        .find_map(|i| {
            (i..nums.len())
                .map(|j| (j, nums[i..j].iter().sum::<usize>()))
                .find(|(_, sum)| sum == invalid_num)
                .and_then(|(j, _)| Some((i, j)))
        })
        .unwrap();

    let min = nums[i..j].iter().min().unwrap();
    let max = nums[i..j].iter().max().unwrap();

    println!("Encryption weakness: {} ({} + {})", min + max, min, max);

    Ok(())
}
