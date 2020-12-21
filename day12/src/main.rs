use std::io::{prelude::*, BufReader};
use std::{error::Error, fs::File};

enum Direction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
}

enum Instruction {
    Direction(Direction),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn main() -> Result<(), Box<dyn Error>> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let mut chars = line.chars();
        let instruction_char = chars.nth(0).unwrap();
        let instruction_num = chars.collect::<String>().parse::<i32>().unwrap();
        instructions.push(match instruction_char {
            'N' => Instruction::Direction(Direction::North(instruction_num)),
            'S' => Instruction::Direction(Direction::South(instruction_num)),
            'E' => Instruction::Direction(Direction::East(instruction_num)),
            'W' => Instruction::Direction(Direction::West(instruction_num)),
            'L' => Instruction::Left(instruction_num),
            'R' => Instruction::Right(instruction_num),
            'F' => Instruction::Forward(instruction_num),
            _ => panic!(),
        });
    }

    let mut facing_degrees = 0;
    let mut position = (0, 0);

    for instruction in instructions {
        match instruction {
            Instruction::Direction(dir) => position = change_position_for_direction(position, dir),
            Instruction::Left(a) => facing_degrees += a,
            Instruction::Right(a) => facing_degrees -= a,
            Instruction::Forward(a) => {
                position = change_position_for_direction(position, get_direction(facing_degrees, a))
            }
        }
    }

    println!("End position: {:?}", position);

    println!(
        "Manhattan distance: {}",
        position.0.abs() + position.1.abs()
    );

    Ok(())
}

fn change_position_for_direction(position: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::North(a) => (position.0, position.1 + a),
        Direction::South(a) => (position.0, position.1 - a),
        Direction::East(a) => (position.0 + a, position.1),
        Direction::West(a) => (position.0 - a, position.1),
    }
}

fn get_direction(degrees: i32, move_amount: i32) -> Direction {
    // Degrees should be in range 0-359 (360 is 0)
    let mut fixed_degrees = degrees;
    while fixed_degrees < 0 {
        fixed_degrees += 360;
    }
    while fixed_degrees >= 360 {
        fixed_degrees -= 360;
    }

    match fixed_degrees {
        45..=135 => Direction::North(move_amount),
        136..=225 => Direction::West(move_amount),
        226..=315 => Direction::South(move_amount),
        _ => Direction::East(move_amount),
    }
}
