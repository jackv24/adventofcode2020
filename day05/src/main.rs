use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut highest_id = 0;

    for line in reader.lines() {
        let line = line?;
        let position = get_position(&line);
        if position.2 > highest_id {
            highest_id = position.2;
        }
    }

    println!("Highest ID: {}", highest_id);

    Ok(())
}

fn get_position(line: &str) -> (i32, i32, i32) {
    if line.chars().count() != 10 {
        panic!("Encountered line with wrong number of characters");
    }

    let mut row_range = (0, 127);
    let mut column_range = (0, 7);

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

    (row, column, seat_id)
}

#[test]
fn test_examples() {
    assert_eq!(get_position("FBFBBFFRLR"), (44, 5, 357));
    assert_eq!(get_position("BFFFBBFRRR"), (70, 7, 567));
    assert_eq!(get_position("FFFBBBFRRR"), (14, 7, 119));
    assert_eq!(get_position("BBFFBBFRLL"), (102, 4, 820));
}
