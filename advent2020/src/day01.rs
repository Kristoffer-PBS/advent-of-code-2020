use crate::prelude::*;
use std::collections::HashSet;

// Constant
const YEAR: usize = 2020;

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
// O(N^2)
fn part_1(input: &str, sum_target: usize) -> usize {
    // transform the string into a vector of usize
    let v: Vec<usize> = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }

            if v[i] + v[j] == sum_target {
                return v[i] * v[j];
            }
        }
    }

    return 0;
}

#[allow(dead_code)]
fn part_1_set(input: &str, sum_target: usize) -> Option<(usize, usize)> {
    let mut lookup: HashSet<usize> = HashSet::new();
    let input: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    for n in input {
        let to_add = sum_target - n;

        if lookup.contains(&to_add) {
            return Some((to_add, n));
        }

        lookup.insert(n);
    }

    None
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
fn part_2(input: &str, sum_target: usize) -> usize {
    // transform the string into a vector of u32
    let v: Vec<usize> = input.lines().map(|s| s.parse::<usize>().unwrap()).collect();

    for i in 0..v.len() {
        for j in 0..v.len() {
            for k in 0..v.len() {
                if i == j || i == k || j == k {
                    continue;
                }

                if v[i] + v[j] + v[k] == sum_target {
                    return v[i] * v[j] * v[k];
                }
            }
        }
    }

    return 0;
}

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day01.txt").expect("Could not open day01.txt");
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let start_part_1 = Instant::now();
    let part_1 = part_1(&input, YEAR);
    // let product_1 = tuple.0 * tuple.1;
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let part_2 = part_2(&input, YEAR);
    // let product_2 = triple.0 * triple.1 * triple.2;
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
    output::print_day(1);
    output::print_part(1, "📄 Product", &format!("{}", results.part_1));
    output::print_part(2, "📄 Product", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
