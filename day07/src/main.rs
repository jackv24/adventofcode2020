use std::io::{self, prelude::*, BufReader};
use std::{collections::HashMap, fs::File};

type ParentBag = HashMap<&'static str, ChildBag>;
type ChildBag = HashMap<&'static str, i32>;

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut parent_bags = ParentBag::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim_end_matches('.');

        let bag_descriptor = line.split(" bags contain ").map(|s| s).collect::<Vec<_>>();

        if bag_descriptor.len() != 2 {
            println!("Could not split into 2 part descriptor: {}", line);
            continue;
        }

        let child_bags = bag_descriptor[1]
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
            .collect::<ChildBag>();

        parent_bags.insert(bag_descriptor[0], child_bags);
    }

    Ok(())
}
