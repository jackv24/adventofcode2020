use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let required_fields: [(&str, fn(&str) -> bool); 7] = [
        ("byr", validate_byr),
        ("iyr", validate_iyr),
        ("eyr", validate_eyr),
        ("hgt", validate_hgt),
        ("hcl", validate_hcl),
        ("ecl", validate_ecl),
        ("pid", validate_pid),
    ];

    // Construct bitmask with all bits set for array indices (for comparison)
    let mut all_fields_mask = 0;
    for i in 0..required_fields.len() {
        let mask = 1 << i;
        all_fields_mask |= mask;
    }

    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut has_fields_mask = 0;
    let mut valid_count = 0;

    for line in reader.lines() {
        let line = line?;

        // End current passport validation when we read a blank line
        if line.len() == 0 {
            // All required bits must be set for passport to be valid
            if has_fields_mask == all_fields_mask {
                valid_count += 1;
            }

            has_fields_mask = 0;
            continue;
        }

        // Some fields are on the same line
        let pairs = line.split_whitespace().collect::<Vec<&str>>();
        for pair in pairs {
            let parts = pair.split(':').collect::<Vec<&str>>();

            // Sit bits in mask for matching required fields
            for i in 0..required_fields.len() {
                let match_tuple = required_fields[i];
                // Matched field
                if parts[0] == match_tuple.0 && match_tuple.1(parts[1]) {
                    let mask = 1 << i;
                    has_fields_mask |= mask;
                }
            }
        }
    }

    // One more check at the end in case we didn't end on a blank line
    if has_fields_mask == all_fields_mask {
        valid_count += 1;
    }

    println!("Valid Passports: {}", valid_count);

    Ok(())
}

fn validate_byr(value: &str) -> bool {
    if value.chars().count() != 4 {
        return false;
    }

    match value.parse::<i32>() {
        Ok(num) => (1920..=2002).contains(&num),
        Err(_) => false,
    }
}

fn validate_iyr(value: &str) -> bool {
    true // TODO
}

fn validate_eyr(value: &str) -> bool {
    true // TODO
}

fn validate_hgt(value: &str) -> bool {
    true // TODO
}

fn validate_hcl(value: &str) -> bool {
    true // TODO
}

fn validate_ecl(value: &str) -> bool {
    true // TODO
}

fn validate_pid(value: &str) -> bool {
    true // TODO
}
