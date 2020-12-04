use std::io::{self, prelude::*, BufReader};
use std::{fs::File, ops::RangeInclusive};

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
    validate_year_range(value, 1920..=2002)
}

fn validate_iyr(value: &str) -> bool {
    validate_year_range(value, 2010..=2020)
}

fn validate_eyr(value: &str) -> bool {
    validate_year_range(value, 2020..=2030)
}

fn validate_hgt(value: &str) -> bool {
    let num = value
        .chars()
        .take_while(|c| c.is_numeric())
        .collect::<String>()
        .parse::<i32>()
        .expect("Couldn't parse num");

    if value.ends_with("cm") {
        (150..=193).contains(&num)
    } else if value.ends_with("in") {
        (59..=76).contains(&num)
    } else {
        false
    }
}

fn validate_hcl(value: &str) -> bool {
    if value.chars().count() != 7 {
        return false;
    }
    let mut tested_first = false;
    for c in value.chars() {
        if !tested_first {
            if c != '#' {
                return false;
            }
            tested_first = true;
        } else {
            let is_num = ('0'..='9').contains(&c);
            let is_char = ('a'..='f').contains(&c);
            if !is_num && !is_char {
                return false;
            }
        }
    }

    true
}

fn validate_ecl(value: &str) -> bool {
    match value {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(value: &str) -> bool {
    if value.chars().count() != 9 {
        return false;
    }
    for c in value.chars() {
        if !c.is_numeric() {
            return false;
        }
    }

    true
}

fn validate_year_range(value: &str, range: RangeInclusive<i32>) -> bool {
    if value.chars().count() != 4 {
        return false;
    }

    match value.parse::<i32>() {
        Ok(num) => range.contains(&num),
        Err(_) => false,
    }
}

#[test]
fn byr_valid() {
    assert_eq!(validate_byr("2002"), true);
}

#[test]
fn byr_invalid() {
    assert_eq!(validate_byr("2003"), false);
}

#[test]
fn hgt_valid() {
    assert_eq!(validate_hgt("60in"), true);
    assert_eq!(validate_hgt("190cm"), true);
}

#[test]
fn hgt_invalid() {
    assert_eq!(validate_hgt("190in"), false);
    assert_eq!(validate_hgt("190"), false);
}

#[test]
fn hcl_valid() {
    assert_eq!(validate_hcl("#123abc"), true);
}

#[test]
fn hcl_invalid() {
    assert_eq!(validate_hcl("#123abz"), false);
    assert_eq!(validate_hcl("123abc"), false);
}

#[test]
fn ecl_valid() {
    assert_eq!(validate_ecl("brn"), true);
}

#[test]
fn ecl_invalid() {
    assert_eq!(validate_ecl("wat"), false);
}

#[test]
fn pid_valid() {
    assert_eq!(validate_pid("000000001"), true);
}

#[test]
fn pid_invalid() {
    assert_eq!(validate_pid("0123456789"), false);
}
