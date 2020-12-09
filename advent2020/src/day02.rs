use crate::prelude::*;

fn part_1(input: &str) -> u32 {
    let mut count = 0_u32;

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let interval: Vec<&str> = words[0].split('-').collect();
        let interval: Vec<u32> = interval
            .iter()
            .map(|&c| c.parse::<u32>().unwrap())
            .collect();
        let target: Vec<char> = words[1].chars().filter(|&c| c != ':').collect();

        let internal_count = words[2]
            .chars()
            .fold(0, |acc, ch| if ch == target[0] { acc + 1 } else { acc });

        if internal_count >= interval[0] && internal_count <= interval[1] {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str) -> u32 {
    let mut count = 0_u32;

    for line in input.lines() {
        let words: Vec<&str> = line.split(' ').collect();
        let interval: Vec<&str> = words[0].split('-').collect();
        let interval: Vec<usize> = interval
            .iter()
            .map(|&c| c.parse::<usize>().unwrap())
            .collect();

        let target: Vec<char> = words[1].chars().filter(|&c| c != ':').collect();

        let mut internal_count = 0_u32;

        for (i, c) in words[2].chars().enumerate() {
            if i + 1 == interval[0] && c == target[0] {
                internal_count += 1;
            }

            if i + 1 == interval[1] && c == target[0] {
                internal_count += 1;
            }
        }

        if internal_count == 1 {
            count += 1;
        }
    }

    count
}

pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day02.txt").expect("Could not open day02.txt");
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    let start_part_1 = Instant::now();
    // let preable = 25_usize;
    let part_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------

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
    output::print_day(2);
    output::print_part(1, "廬 Valid passwords", &format!("{}", results.part_1));
    output::print_part(2, "廬 Valid passwords", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
