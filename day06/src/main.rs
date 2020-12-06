use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut yes_any_map: HashMap<char, bool> = HashMap::new();
    let mut any_yes_count = 0;

    let mut yes_all_map: HashMap<char, i32> = HashMap::new();
    let mut all_yes_count = 0;

    let mut group_line_count = 0;

    for line in reader.lines() {
        let line = line?;

        // Hit blank line, end current group
        if line.len() == 0 {
            any_yes_count += end_group_any(&mut yes_any_map);
            all_yes_count += end_group_all(&mut yes_all_map, &group_line_count);
            group_line_count = 0;
            continue;
        }

        group_line_count += 1;

        for c in line.chars() {
            if !c.is_whitespace() {
                yes_any_map.insert(c, true);

                if yes_all_map.contains_key(&c) {
                    let val = yes_all_map[&c];
                    yes_all_map.insert(c, val + 1);
                } else {
                    yes_all_map.insert(c, 1);
                }
            }
        }
    }

    // One more at end in case input doesn't end on blank line
    any_yes_count += end_group_any(&mut yes_any_map);
    all_yes_count += end_group_all(&mut yes_all_map, &group_line_count);

    // Part 1
    println!("Sum of Any Counts: {}", any_yes_count);

    // Part 2
    println!("Sum of All Counts: {}", all_yes_count);

    Ok(())
}

fn end_group_any(map: &mut HashMap<char, bool>) -> i32 {
    let mut group_yes_count = 0;

    // Count each answer the group said yes to at least once
    for kvp in map {
        if *kvp.1 {
            group_yes_count += 1;

            // Clear array for next group
            *kvp.1 = false;
        }
    }

    //println!("Group any yes count: {}", group_yes_count);
    group_yes_count
}

fn end_group_all(map: &mut HashMap<char, i32>, group_line_count: &i32) -> i32 {
    let mut group_yes_count = 0;

    // Count each answer the group said yes to at least once
    for kvp in map {
        if *kvp.1 >= *group_line_count {
            group_yes_count += 1;
        }

        // Clear array for next group
        *kvp.1 = 0;
    }

    //println!("Group all yes count: {}", group_yes_count);
    group_yes_count
}
