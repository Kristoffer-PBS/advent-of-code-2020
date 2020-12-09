use crate::prelude::*;
use std::collections::{HashMap, HashSet};

// For each group, count the number of questions to which anyone answered "yes".
// What is the sum of those counts?
fn part_1(input: &str) -> usize {
    let mut sum = 0_usize;

    // go through each group
    for group in input.split("\n\n") {
        let mut set: HashSet<char> = HashSet::with_capacity(group.len());

        sum += group
            .chars()
            .filter(|c| c.is_ascii_alphabetic()) // filter out escape characters: '\r' and '\n'
            .filter(|&c| set.insert(c)) // use the uniqueness property of a set to filter
            .count();
    }
    return sum;
}

fn part_2(input: &str) -> usize {
    let mut sum = 0_usize;

    // go through each group
    for group in input.split("\n\n") {
        // create a set containing the different questions, which have been answered "yes"
        let questions_answered: HashSet<char> =
            group.chars().filter(|c| c.is_ascii_alphabetic()).collect();

        let mut answers: HashMap<char, usize> = HashMap::new();

        // for each question type, use a map to count how many time a question has been answered.
        for letter in group.chars().filter(|c| c.is_ascii_alphabetic()) {
            let counter = answers.entry(letter).or_insert(0);
            *counter += 1;
        }

        for (key, value) in &answers {
            if questions_answered.contains(key) && *value == group.lines().count() {
                sum += 1;
            }
        }
    }

    sum
}

pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day06.txt").expect("Could not open day06.txt");
    let time_setup = start_setup.elapsed();

    let example = "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    // each line in a group represents the number of people in that group
    // the number of characters in a line represents the number of questions they answered yes to.
    // repeated answers for each group is not counted multiple times
    assert_eq!(part_1(example), 11);
    let start_part_1 = Instant::now();
    // let preable = 25_usize;
    let part_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------

    assert_eq!(part_2(example), 6);
    let start_part_2 = Instant::now();
    let part_2 = part_2(&input);
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
    output::print_day(6);
    output::print_part(1, "+ sum of yes'", &format!("{}", results.part_1));
    output::print_part(2, "+ sum of answers", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
