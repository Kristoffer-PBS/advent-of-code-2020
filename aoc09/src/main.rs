#[allow(dead_code)]
fn factorial(n: usize) -> usize {
    (1..=n).fold(1, |acc, n| acc * n)
}

#[allow(dead_code)]
fn choose(n: usize, k: usize) -> usize {
    assert!(n > k);

    factorial(n) / (factorial(k) * factorial(n - k))
}

fn part_1(input: &[usize], preable: usize) -> Option<usize> {
    let mut start = 0_usize;

    for &n in input.iter().skip(preable) {
        let mut flag = (n, false);

        for i in start..preable + start {
            for j in i + 1..preable + start {
                // println!("i: {}, j:  {}, n: {}", input[i], input[j], n);
                // println!("sum: {}", input[i] + input[j]);

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
                // println!("min: {}, max: {}", min, max);
                return Some((*min, *max));
            } else if sum > invalid_number {
                break;
            }
        }

        sum = 0;
    }

    None
}

fn main() {
    let input: String =
        std::fs::read_to_string("input/input.txt").expect("Could not open day09.txt");

    let input: Vec<usize> = input
        .lines()
        .map(|line| line.parse().expect("Could not parse line"))
        .collect();

    let example: Vec<usize> =
        "35\n20\n15\n25\n47\n40\n62\n55\n65\n95\n102\n117\n150\n182\n127\n219\n299\n277\n309\n576"
            .lines()
            .map(|line| line.parse().expect("Could not parse example"))
            .collect();

    // ------------------------------------------------------------------------
    // PART 1
    // ------------------------------------------------------------------------
    assert_eq!(part_1(&example, 5).unwrap(), 127);

    let preable = 25_usize;
    let part_1 = part_1(&input, preable).unwrap();
    println!("part 1: {}", part_1);

    // ------------------------------------------------------------------------
    // PART 2
    // ------------------------------------------------------------------------
    let (min, max) = part_2(&example, 127).unwrap();
    assert_eq!(min + max, 15 + 47);

    let (min, max) = part_2(&input, part_1).unwrap();
    println!(
        "part 2:\n\tmin: {}\n\tmax: {}\n\tsum: {}",
        min,
        max,
        min + max
    );
}
