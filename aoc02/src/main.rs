use std::fs;

// How many passwords are valid according to their policies?
fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input file");

    println!("part 1: {}", part_1(&input));

    println!("part 2: {}", part_2(&input));
}

// parse input line into a lower limit, a higher limit, a char, and a string

struct PwdEntry {
    pub lower: u32,
    pub higher: u32,
    pub c: char,
    pub pwd: String,
    pub valid: bool,
}

// impl PwdEntry {
//     fn parse(line: &str) -> PwdEntry {}
// }

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
