use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    match scan_lines(reader, 25) {
        Ok(num) => println!("Found wrong num: {}", num),
        Err(_) => println!("Did not find wrong num"),
    }

    Ok(())
}

fn scan_lines(reader: BufReader<File>, preamble: usize) -> Result<i32, ()> {
    let mut nums = Vec::new();

    for line in reader.lines() {
        let num = line.unwrap().parse::<i32>().unwrap();
        nums.push(num);

        if nums.len() <= preamble {
            continue;
        }

        let len = nums.len() - 1;

        let mut success = false;
        for i in len - preamble..len {
            for j in len - preamble..len {
                let num_a = nums[i];
                let num_b = nums[j];

                if num_a == num_b {
                    continue;
                }

                if num_a + num_b == num {
                    success = true;
                }
            }
        }

        if !success {
            return Ok(num);
        }
    }

    Err(())
}
