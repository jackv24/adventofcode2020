use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut yes_answer_map: HashMap<char, bool> = HashMap::new();
    let mut yes_count = 0;

    for line in reader.lines() {
        let line = line?;

        // Hit blank line, end current group
        if line.len() == 0 {
            yes_count += end_group(&mut yes_answer_map);
            continue;
        }

        for c in line.chars() {
            if !c.is_whitespace() {
                yes_answer_map.insert(c, true);
            }
        }
    }

    // One more at end in case input doesn't end on blank line
    yes_count += end_group(&mut yes_answer_map);

    println!("Sum of Counts: {}", yes_count);

    Ok(())
}

fn end_group(yes_answer_map: &mut HashMap<char, bool>) -> i32 {
    let mut group_yes_count = 0;

    // Count each answer the group said yes to at least once
    for kvp in yes_answer_map {
        if *kvp.1 {
            group_yes_count += 1;

            // Clear array for next group
            *kvp.1 = false;
        }
    }

    println!("Group yes count: {}", group_yes_count);
    group_yes_count
}
