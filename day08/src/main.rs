use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Copy, Clone)]
enum Instructions {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

struct EndState {
    accumulator: i32,
    index: usize,
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
        let instruction_num = line[line.rfind(' ').unwrap() + 1..].parse::<i32>().unwrap();

        let instruction = match instruction_str {
            "nop" => Instructions::Nop(instruction_num),
            "acc" => Instructions::Acc(instruction_num),
            "jmp" => Instructions::Jmp(instruction_num),
            _ => panic!(),
        };

        instructions.push(instruction);
    }

    // Part 1
    let end_state = execute_instructions(&instructions);
    println!("Part 1 accumulator: {}", end_state.accumulator);

    let mut did_change_any = false;
    let mut changed_index = 0;
    let mut changed_instr = Instructions::Nop(0);

    // Part 2
    // Prevent infinite loop - we shouldn't need to loop more than this
    for _ in 0..instructions.len() {
        let end_state = execute_instructions(&instructions);

        // We didn't hit an infinite loop
        if end_state.index >= instructions.len() - 1 {
            println!("Part 2 accumulator: {}", end_state.accumulator);
            break;
        }

        // Previous change attempt failed, so restore it before changing another
        if did_change_any {
            instructions[changed_index] = changed_instr;
        }

        let start_index = if did_change_any {
            changed_index + 1
        } else {
            changed_index
        };

        // Find and flip next match
        for i in start_index..instructions.len() {
            let new_instr = match instructions[i] {
                Instructions::Nop(num) => Instructions::Jmp(num),
                Instructions::Jmp(num) => Instructions::Nop(num),
                Instructions::Acc(_) => continue,
            };

            changed_index = i;
            changed_instr = instructions[changed_index];
            instructions[changed_index] = new_instr;
            did_change_any = true;

            break;
        }
    }

    Ok(())
}

fn execute_instructions(instructions: &Vec<Instructions>) -> EndState {
    let instruction_count = instructions.len();
    let mut encountered = vec![false; instruction_count];

    let mut accumulator = 0;
    let mut i: usize = 0;

    while i < instruction_count {
        if encountered[i] {
            break;
        }
        encountered[i] = true;

        match instructions[i] {
            Instructions::Nop(_) => (),
            Instructions::Acc(num) => accumulator += num,
            Instructions::Jmp(num) => {
                i = (i as i32 + num) as usize;

                // Since we're jumping to another index cancel here
                continue;
            }
        }

        i += 1;
    }

    EndState {
        accumulator,
        index: i,
    }
}
