use crate::prelude::*;

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

pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day05.txt").expect("Could not open day05.txt");
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    assert_eq!(part_1("BFFFBBFRRR"), 567);
    assert_eq!(part_1("FFFBBBFRRR"), 119);
    assert_eq!(part_1("BBFFBBFRLL"), 820);

    let start_part_1 = Instant::now();
    let part_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------

    let start_part_2 = Instant::now();
    let part_2 = part_2(&input).unwrap();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        part_1 as i64,
        part_2 as i64,
        Timing::new(
            time_setup,
            time_part_1,
            time_part_2,
            time_setup + time_part_1 + time_part_2,
        ),
    )
}

// -----------------------------------------------------------------------------
// Report
// -----------------------------------------------------------------------------
pub(crate) fn report(results: &Results) {
    output::print_day(5);
    output::print_part(1, " Highest seat ID", &format!("{}", results.part_1));
    output::print_part(2, " your seat ID", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
