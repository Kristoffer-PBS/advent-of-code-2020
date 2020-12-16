use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input.txt");

    println!("part 1: {}", part_1(&input));

    let inputs = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!("part 2: {}", part_2(&input, inputs));
}

// starting position top left
// right 3, down 1
#[allow(dead_code)]
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

        println!(
            "collision for strategy right: {}, down: {} is {}",
            directions[i].0, directions[i].1, count
        );

        total *= count;
    }

    total
}
