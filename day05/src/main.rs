use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Eq, PartialEq, Debug)]
struct Position {
    row: i32,
    column: i32,
    seat_id: i32,
}

impl Position {
    fn new(row: i32, column: i32, seat_id: i32) -> Self {
        Self {
            row,
            column,
            seat_id,
        }
    }
}

const ROWS: i32 = 128;
const COLUMNS: i32 = 8;

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut highest_id = 0;
    let mut found_seat_ids = [false; ((ROWS * 8) + COLUMNS) as usize];

    // Part 1
    for line in reader.lines() {
        let line = line?;
        let position = get_position(&line);
        if position.seat_id > highest_id {
            highest_id = position.seat_id;
        }
        found_seat_ids[position.seat_id as usize] = true;
    }

    println!("Highest ID: {}", highest_id);

    // Part 2
    for i in 1..found_seat_ids.len() - 1 {
        if found_seat_ids[i] {
            continue;
        }
        if found_seat_ids[i - 1] && found_seat_ids[i + 1] {
            println!("Found missing seat ID: {}", i);
        }
    }

    Ok(())
}

fn get_position(line: &str) -> Position {
    if line.chars().count() != 10 {
        panic!("Encountered line with wrong number of characters");
    }

    let mut row_range = (0, ROWS - 1);
    let mut column_range = (0, COLUMNS - 1);

    let chars = line.chars().collect::<Vec<_>>();
    for i in 0..chars.len() {
        let c = chars[i];
        match c {
            'F' => {
                row_range.1 =
                    row_range.1 - ((row_range.1 - row_range.0) as f32 / 2.0).floor() as i32;
            }
            'B' => {
                row_range.0 =
                    row_range.0 + ((row_range.1 - row_range.0) as f32 / 2.0).ceil() as i32;
            }
            'L' => {
                column_range.1 = column_range.1
                    - ((column_range.1 - column_range.0) as f32 / 2.0).floor() as i32;
            }
            'R' => {
                column_range.0 =
                    column_range.0 + ((column_range.1 - column_range.0) as f32 / 2.0).ceil() as i32;
            }
            _ => panic!("Encountered char that was not FBLR"),
        };
    }

    // Assumes tuple 0 and 1 are the same
    let row = row_range.0;
    let column = column_range.0;
    let seat_id = (row * 8) + column;

    Position {
        row,
        column,
        seat_id,
    }
}

#[test]
fn test_examples() {
    assert_eq!(get_position("FBFBBFFRLR"), Position::new(44, 5, 357));
    assert_eq!(get_position("BFFFBBFRRR"), Position::new(70, 7, 567));
    assert_eq!(get_position("FFFBBBFRRR"), Position::new(14, 7, 119));
    assert_eq!(get_position("BBFFBBFRLL"), Position::new(102, 4, 820));
}
