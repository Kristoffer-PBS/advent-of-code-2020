use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input file");

    assert_eq!(part_1("BFFFBBFRRR"), 567);
    assert_eq!(part_1("FFFBBBFRRR"), 119);
    assert_eq!(part_1("BBFFBBFRLL"), 820);

    println!("part 1: {}", part_1(&input));

    if let Some(seat) = part_2(&input) {
        println!("part 2: {}", seat);
    }
}

fn difference(a: usize, b: usize) -> usize {
    (a - b + 1) / 2
}

// As a sanity check, look through your list of boarding
// passes. What is the highest seat ID on a boarding pass?
fn part_1(input: &str) -> usize {
    let mut highest = 0_usize;

    // the first 7 characters will either be F or B
    // there are 128 rows; 0 to 127.

    // The last three characters will be either L or R
    // there are 8 columns; 0 to 7

    for line in input.lines() {
        let mut back = 0_usize;
        let mut front = 127_usize;
        let mut right = 0_usize;
        let mut left = 7_usize;

        for direction in line.chars() {
            match direction {
                'F' => front -= difference(front, back),
                'B' => back += difference(front, back),
                'L' => left -= difference(left, right),
                'R' => right += difference(left, right),
                _ => {}
            }
        }

        if front * 8 + left > highest {
            highest = front * 8 + left;
        }
    }

    highest
}

// 2^7 * 2^3 = 2^10 = 1024 seats
fn part_2(input: &str) -> Option<usize> {
    let mut passenger_list: Vec<usize> = Vec::with_capacity(input.lines().count());

    for line in input.lines() {
        let mut back = 0_usize;
        let mut front = 127_usize;
        let mut right = 0_usize;
        let mut left = 7_usize;

        for direction in line.chars() {
            match direction {
                'F' => front -= difference(front, back),
                'B' => back += difference(front, back),
                'L' => left -= difference(left, right),
                'R' => right += difference(left, right),
                _ => {}
            }
        }
        passenger_list.push(front * 8 + left);
    }

    passenger_list.sort();

    // find the 2 seat numbers with a absolute difference greater than 1
    for (i, x) in passenger_list.iter().enumerate() {
        if i + 1 != passenger_list.len() {
            if passenger_list[i + 1] - x > 1 {
                return Some(x + 1);
            }
        }
    }

    None
}
