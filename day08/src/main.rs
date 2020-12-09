use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
enum Instructions {
    Nop,
    Acc(i32),
    Jmp(i32),
}

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    // Load input file into instructions
    for line in reader.lines() {
        let line = line?;
        let instruction_str = &line[0..line.find(' ').unwrap()];
        let instruction_num = &line[line.rfind(' ').unwrap() + 1..].parse::<i32>().unwrap();

        let instruction = match instruction_str {
            "nop" => Instructions::Nop,
            "acc" => Instructions::Acc(*instruction_num),
            "jmp" => Instructions::Jmp(*instruction_num),
            _ => panic!(),
        };

        instructions.push((instruction, 0));
    }

    let mut accumulator = 0;

    let instruction_count = instructions.len();
    let mut i: usize = 0;

    while i < instruction_count {
        let instr_ref = &mut instructions[i];

        if instr_ref.1 > 0 {
            println!("Encountered instruction at index {} again, breaking...", i);
            break;
        }

        instr_ref.1 += 1;

        match instr_ref.0 {
            Instructions::Nop => (),
            Instructions::Acc(num) => accumulator += num,
            Instructions::Jmp(num) => {
                i = (i as i32 + num) as usize;

                // Since we're jumping to another index cancel here
                continue;
            }
        }

        i += 1;
    }

    println!("Accumulator Value: {}", accumulator);

    Ok(())
}
