use std::io::{prelude::*, BufReader};
use std::{error::Error, fs::File};

fn main() -> Result<(), Box<dyn Error>> {
    // load input file as buffered reader, in case the file is long
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Vector of vectors to be a dynamic 2D array - this can be improved!
    let mut chars = Vec::new();
    let mut width = 0;

    for line in reader.lines() {
        let line = line?;
        for c in line.chars() {
            chars.push(c);
        }
        if width == 0 {
            width = line.chars().count();
        }
    }

    let mut char_buf = Vec::new();

    let mut round_count: usize = 0;
    while execute_round(&chars, &mut char_buf, width) {
        round_count += 1;
        println!("Completed round: {}", round_count);
        println!();

        // Transfer new char buffer into chars for next round
        for i in 0..chars.len() {
            chars[i] = char_buf[i];
        }
        char_buf.clear();
    }

    println!("Nothing changed last round!");

    let mut occupied_seats: usize = 0;
    for c in chars {
        if c == '#' {
            occupied_seats += 1;
        }
    }

    println!("Occupied seats: {}", occupied_seats);

    Ok(())
}

fn execute_round(chars: &Vec<char>, char_buf: &mut Vec<char>, width: usize) -> bool {
    let mut did_change = false;
    for i in 0..chars.len() {
        let x = i % width;
        let y = i / width;
        let c = *get_2d(&chars, width, x, y);

        let mut new_c = c;
        if c == 'L' {
            if count_adjacent(&chars, width, x, y, &'#') == 0 {
                new_c = '#';
            }
        } else if c == '#' {
            if count_adjacent(&chars, width, x, y, &'#') >= 5 {
                new_c = 'L';
            }
        }

        print!("{}", new_c);
        if x >= width - 1 {
            println!();
        }

        if new_c != c {
            did_change = true;
        }

        char_buf.push(new_c);
    }
    did_change
}

fn get_2d<T>(vec: &Vec<T>, width: usize, x: usize, y: usize) -> &T {
    &vec[x + width * y]
}

fn count_adjacent(vec: &Vec<char>, width: usize, x: usize, y: usize, value: &char) -> usize {
    let mut occupied_count = 0;

    // Right
    if can_see(vec, width, x, y, 1, 0, value) {
        occupied_count += 1;
    }

    // Right-Up
    if can_see(vec, width, x, y, 1, -1, value) {
        occupied_count += 1;
    }

    // Up
    if can_see(vec, width, x, y, 0, -1, value) {
        occupied_count += 1;
    }

    // Up-Left
    if can_see(vec, width, x, y, -1, -1, value) {
        occupied_count += 1;
    }

    // Left
    if can_see(vec, width, x, y, -1, 0, value) {
        occupied_count += 1;
    }

    // Left-Down
    if can_see(vec, width, x, y, -1, 1, value) {
        occupied_count += 1;
    }

    // Down
    if can_see(vec, width, x, y, 0, 1, value) {
        occupied_count += 1;
    }

    // Down-Right
    if can_see(vec, width, x, y, 1, 1, value) {
        occupied_count += 1;
    }

    occupied_count
}

fn can_see(
    vec: &Vec<char>,
    width: usize,
    x: usize,
    y: usize,
    move_x: i32,
    move_y: i32,
    value: &char,
) -> bool {
    let max_x = width - 1;
    let max_y = (vec.len() / width) - 1;

    if move_x < 0 {
        if x == 0 {
            return false;
        }
    } else {
        if x >= max_x {
            return false;
        }
    }

    if move_y < 0 {
        if y == 0 {
            return false;
        }
    } else {
        if y >= max_y {
            return false;
        }
    }

    let new_x = (x as i32 + move_x) as usize;
    let new_y = (y as i32 + move_y) as usize;

    if get_2d(vec, width, new_x, new_y) == value {
        true
    } else {
        can_see(vec, width, new_x, new_y, move_x, move_y, value)
    }
}
