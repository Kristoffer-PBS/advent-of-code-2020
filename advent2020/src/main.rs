#[macro_use]
extern crate lazy_static;

// -----------------------------------------------------------------------------
// Modules
// -----------------------------------------------------------------------------
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
// mod day10;
// mod day11;
mod day12;
mod output;

// -----------------------------------------------------------------------------
// Results struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Results {
    part_1: i64,
    part_2: i64,
    times: Timing,
}

impl Results {
    fn new(part_1: i64, part_2: i64, times: Timing) -> Self {
        Self {
            part_1,
            part_2,
            times,
        }
    }
}

// -----------------------------------------------------------------------------
// Timing struct
// -----------------------------------------------------------------------------
#[derive(Debug)]
pub(crate) struct Timing {
    setup: std::time::Duration,
    part_1: std::time::Duration,
    part_2: std::time::Duration,
    combined: std::time::Duration,
}

impl Timing {
    fn new(
        setup: std::time::Duration,
        part_1: std::time::Duration,
        part_2: std::time::Duration,
        combined: std::time::Duration,
    ) -> Self {
        Self {
            setup,
            part_1,
            part_2,
            combined,
        }
    }
}

// -----------------------------------------------------------------------------
// Prelude
// -----------------------------------------------------------------------------
const REPETITIONS: u32 = 10;
pub(crate) mod prelude {
    pub(crate) use crate::REPETITIONS;
    pub(crate) use crate::{output, Results, Timing};
    pub(crate) use colored::*;
    pub(crate) use std::time::Instant;
}

// -----------------------------------------------------------------------------
// Main Driver
// -----------------------------------------------------------------------------
fn main() {
    // Setup
    const DAYS: usize = 10;

    let runs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        // day10::run,
        // day11::run,
        day12::run,
    ];
    let reports = [
        day01::report,
        day02::report,
        day03::report,
        day04::report,
        day05::report,
        day06::report,
        day07::report,
        day08::report,
        day09::report,
        // day10::report,
        // day11::report,
        day12::report,
    ];

    // Each day
    output::print_header();
    let mut day_results: [Vec<Results>; DAYS] = [
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    for _ in 0..REPETITIONS {
        for (i, day) in runs.iter().enumerate() {
            day_results[i].push(day());
        }
    }

    let average_times: Vec<Timing> = day_results
        .iter()
        .map(|day| {
            day.iter().fold(
                Timing::new(
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                    std::time::Duration::new(0, 0),
                ),
                |acc, result| {
                    Timing::new(
                        acc.setup + result.times.setup / REPETITIONS,
                        acc.part_1 + result.times.part_1 / REPETITIONS,
                        acc.part_2 + result.times.part_2 / REPETITIONS,
                        acc.combined + result.times.combined / REPETITIONS,
                    )
                },
            )
        })
        .collect();

    for i in 0..DAYS {
        let result = day_results[i].first().unwrap();
        let timing = &average_times[i];

        reports[i](&Results::new(
            result.part_1,
            result.part_2,
            Timing::new(timing.setup, timing.part_1, timing.part_2, timing.combined),
        ));
    }

    // Day comparison
    output::print_header();
    let time_averages = average_times.iter().map(|day| day.combined).collect();
    let time_std_devs: Vec<f64> = average_times
        .iter()
        .zip(day_results.iter())
        .map(|(averages, day)| {
            (day.iter().fold(0.0, |acc, repetition| {
                acc + ((averages.combined.as_nanos() as f64
                    - repetition.times.combined.as_nanos() as f64)
                    / 1000.0)
                    .powf(2.0)
                    / ((REPETITIONS - 1) as f64)
            }))
            .powf(0.5)
        })
        .collect();
    output::print_days_timing(&time_averages, &time_std_devs);
    output::print_header();
}

#[cfg(test)]
mod tests {
    use super::*;
    const MAX_TIME: u128 = 300;

    macro_rules! test_day {
        ($results:expr, $part_1:expr, $part_2:expr) => {
            assert_eq!($results.part_1, $part_1);
            assert_eq!($results.part_2, $part_2);
            assert!($results.times.combined.as_millis() < MAX_TIME);
        };
    }

    #[test]
    fn test_01() {
        let results = day01::run();
        test_day!(results, 1020036, 286977330);
    }
    #[test]
    fn test_02() {
        let results = day02::run();
        test_day!(results, 586, 352);
    }

    #[test]
    fn test_03() {
        let results = day03::run();
        test_day!(results, 294, 5774564250);
    }

    #[test]
    fn test_04() {
        let results = day04::run();
        test_day!(results, 192, 101);
    }

    #[test]
    fn test_05() {
        let results = day05::run();
        test_day!(results, 970, 587);
    }

    #[test]
    fn test_06() {
        let results = day06::run();
        test_day!(results, 6630, 3437);
    }

    #[test]
    fn test_07() {
        let results = day07::run();
        test_day!(results, 222, 13264);
    }

    #[test]
    fn test_08() {
        let results = day08::run();
        test_day!(results, 1949, 2092);
    }

    #[test]
    fn test_09() {
        let results = day09::run();
        test_day!(results, 90433990, 11691646);
    }

    #[test]
    fn test_12() {
        let results = day12::run();
        test_day!(results, 923, 24769);
    }
}

// -----------------------------------------------------------------------------
