use crate::prelude::*;

// #[allow(dead_code)]
// fn factorial(n: u32) -> u32 {
//     (1..=n).iter().fold(1, |acc, n| acc * n)
// }
// #[allow(dead_code)]
// fn choose(n: u32, k: u32) -> u32 {
//     assert!(n > k);

//     factorial(n) / (factorial(k) * (factorial(n - k)))
// }

// #[allow(dead_code)]
// fn comb(n: u32, k: u32) -> Vec<(u32, u32)> {
//     assert!(n > k);

//     vec![]
// }

fn part_1(input: &[usize], preable: usize) -> Option<usize> {
    let mut start = 0_usize;

    for &n in input.iter().skip(preable) {
        let mut flag = (n, false);

        for i in start..preable + start {
            for j in i + 1..preable + start {
                if input[i] + input[j] == n {
                    flag = (n, true);
                    break;
                }
            }
            if flag.1 {
                break;
            }
        }

        if !flag.1 {
            return Some(flag.0);
        }
        start += 1;
    }

    None
}

fn part_2(input: &[usize], invalid_number: usize) -> Option<(usize, usize)> {
    let mut sum = 0_usize;

    for i in 0..input.len() {
        for j in i..input.len() {
            sum += input[j];

            if sum == invalid_number {
                let min = &input[i..j]
                    .iter()
                    .fold(input[i], |min, &n| std::cmp::min(min, n));
                let max = &input[i..j]
                    .iter()
                    .fold(input[i], |max, &n| std::cmp::max(max, n));
                return Some((*min, *max));
            } else if sum > invalid_number {
                break;
            }
        }

        sum = 0;
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
        std::fs::read_to_string("input/day09.txt").expect("Could not open day09.txt");

    // Read to vector
    let input: Vec<usize> = input
        .lines()
        .map(|line| line.parse().expect("Could not parse input"))
        .collect();

    let time_setup = start_setup.elapsed();

    let example: Vec<usize> =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
            .lines()
            .map(|line| line.parse().expect("Could not parse example"))
            .collect();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    // example
    assert_eq!(part_1(&example, 5).unwrap(), 127);

    let start_part_1 = Instant::now();
    let preable = 25_usize;
    let part_1 = part_1(&input, preable).unwrap();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // example
    let (min, max) = part_2(&example, 127).unwrap();
    assert_eq!(min + max, 15 + 47);

    let start_part_2 = Instant::now();
    let (min, max) = part_2(&input, part_1).unwrap();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        part_1 as i64,
        min as i64 + max as i64,
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
    output::print_day(9);
    output::print_part(1, "📄 Invalid number", &format!("{}", results.part_1));
    output::print_part(2, "📄 encryption weakness", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
