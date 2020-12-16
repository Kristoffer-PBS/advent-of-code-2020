fn main() {
    let input: String =
        std::fs::read_to_string("input/input.txt").expect("Could not open input.txt");

    let example_1 = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    let example_2 = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

    // assert_eq!(part_1(example_1), 7 * 5);
    // assert_eq!(part_1(example_2), 22 * 10);

    // println!("part 1: {}", part_1(&input));

    assert_eq!(part_2(example_1), 8);
    assert_eq!(part_2(example_2), 19208);

    // println!("part 2: {}", part_1(&input));
}

// What is the number of 1-jolt differences multiplied with the number of 3-jolt differences?
fn part_1(input: &str) -> usize {
    let mut input: Vec<usize> = input
        .lines()
        .map(|line| line.parse().expect("Could not parse input"))
        .collect();

    input.sort();

    let mut one = 0_usize;
    let mut tree = 0_usize;

    println!("{:?}", input);

    input.iter().fold(0, |adapter, n| {
        match n - adapter {
            1 => one += 1,
            3 => tree += 1,
            _ => panic!("this adapter {} does not match into {}", n, adapter),
        };
        *n
    });

    // remember to count the device's built-in adapter is always 3 higher than the highest
    // adapter, so there is one additional with a difference of 3.
    one * (tree + 1)
}

#[allow(dead_code)]
fn fibonacci_sequence(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_sequence(n - 1) + fibonacci_sequence(n - 2),
    }
}

#[allow(dead_code)]
fn tribonacci_sequence(n: usize) -> usize {
    match n {
        0 => 0,
        1 | 2 => 1,
        _ => tribonacci_sequence(n - 1) + tribonacci_sequence(n - 2) + tribonacci_sequence(n - 3),
    }
}

// What is the total number of distinct ways you can arrange the adapters to connect
// the charging outlet to your device?
fn part_2(input: &str) -> usize {
    let mut input: Vec<usize> = input
        .lines()
        .map(|line| line.parse().expect("Could not parse input"))
        .collect();

    input.sort();

    let _device = input[input.len() - 1] + 3;

    let mut total = 0_usize;

    println!("{:?}", input);

    for (i, adapter) in input.iter().enumerate() {
        println!("total is {}", total);
        let futures = input.iter().skip(i + 1).take(3).fold(0, |acc, n| {
            println!("n {} - adapter {}", n, adapter);
            if n - adapter <= 3 && n - adapter > 1 {
                acc + 1
            } else {
                acc
            }
        });
        if futures > 1 {
            total += futures + 1;
        } else {
            total += futures;
        }
    }

    total + 1
}
