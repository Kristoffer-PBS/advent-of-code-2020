use crate::prelude::*;

fn part_1(input: &str) -> usize {
    // modulo
    // number of lines - 1
    // compute width of lines and use modulo to index

    let mut count = 0_usize;
    let mut right = 0_usize;
    let mut down = 0_usize;

    let lines: Vec<&str> = input.lines().collect();

    for _ in 0..lines.len() - 1 {
        right += 3;
        down += 1;

        if lines[down].chars().nth(right % lines[0].len()).unwrap() == '#' {
            count += 1;
        }
    }

    count
}

fn part_2(input: &str, directions: Vec<(usize, usize)>) -> usize {
    let mut total = 1_usize;

    let lines: Vec<&str> = input.lines().collect();

    for i in 0..directions.len() {
        let mut count = 0_usize;
        let mut right = 0_usize;
        let mut down = 0_usize;

        while down < lines.len() - 1 {
            right += directions[i].0;
            down += directions[i].1;

            if lines[down % lines.len()]
                .chars()
                .nth(right % lines[0].len())
                .unwrap()
                == '#'
            {
                count += 1;
            }
        }

        // println!(
        //     "collision for strategy right: {}, down: {} is {}",
        //     directions[i].0, directions[i].1, count
        // );

        total *= count;
    }

    total
}

pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day03.txt").expect("Could not open day03.txt");

    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    let start_part_1 = Instant::now();
    let part_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    let start_part_2 = Instant::now();
    let inputs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let part_2 = part_2(&input, inputs);
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
    output::print_day(3);
    output::print_part(1, " trees encountered", &format!("{}", results.part_1));
    output::print_part(
        2,
        "樂 sum of trees encountered",
        &format!("{}", results.part_2),
    );
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
