use std::io::{self, prelude::*, BufReader};
use std::{collections::HashMap, fs::File};

struct ParentBags(HashMap<String, ChildBags>);
struct ChildBags(HashMap<String, i32>);

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parent_bags = ParentBags(HashMap::new());

    for line in reader.lines() {
        let line = line?;
        let line = line.trim_end_matches('.');

        let bag_descriptor = line.split(" bags contain ").map(|s| s).collect::<Vec<_>>();

        if bag_descriptor.len() != 2 {
            println!("Could not split into 2 part descriptor: {}", line);
            continue;
        }

        let child_bags = ChildBags(
            bag_descriptor[1]
                .split(", ")
                .map(|bag| {
                    let index_left = bag.find(' ').unwrap();
                    let index_right = bag.rfind(' ').unwrap();
                    let num = match &bag[0..index_left] {
                        "no" => 0,
                        _ => bag[0..index_left].parse::<i32>().unwrap(),
                    };
                    let bag_name = &bag[index_left + 1..index_right];
                    (bag_name, num)
                })
                .filter(|(_, count)| count > &0)
                .map(|(s, c)| (s.to_string(), c))
                .collect(),
        );

        parent_bags
            .0
            .insert(bag_descriptor[0].to_string(), child_bags);
    }

    let innermost_bag = "shiny gold";
    let count = parent_bags
        .0
        .values()
        .filter(|parent_bag| can_hold(&parent_bags, parent_bag, innermost_bag))
        .count();

    println!(
        "Bags that will eventually contain a {} bag: {}",
        innermost_bag, count
    );

    Ok(())
}

fn can_hold(parent_bags: &ParentBags, child_bags: &ChildBags, innermost_bag: &str) -> bool {
    if child_bags.0.contains_key(innermost_bag) {
        return true;
    }

    for kvp in &child_bags.0 {
        if can_hold(
            parent_bags,
            parent_bags.0.get(kvp.0).unwrap(),
            innermost_bag,
        ) {
            return true;
        }
    }

    false
}
