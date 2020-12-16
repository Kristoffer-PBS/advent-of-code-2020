use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("could not open input file");

    assert_eq!(part_1("1721\n979\n366\n299\n675\n1456", 2020), 514579);

    println!("part 1: {}", part_1(&input, 2020));

    if let Some((a, b)) = part_1_set(&input, 2020) {
        println!("found a: {} and b: {}", a, b);
        println!("there product is: {}", a * b);
    }

    assert_eq!(part_2("1721\n979\n366\n299\n675\n1456", 2020), 241861950);

    println!("part 2: {}", part_2(&input, 2020));

    // if let Some((a, b, c)) = part_2_set(&input, 2020) {
    //     println!("found a: {}, b: {} and c: {}", a, b, c);
    //     println!("their product is: {}", a * b * c);
    // }
}

// O(N^2)
fn part_1(input: &str, sum_target: u32) -> u32 {
    // transform the string into a vector of u32
    let v: Vec<u32> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();

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

fn part_2(input: &str, sum_target: u64) -> u64 {
    // transform the string into a vector of u32
    let v: Vec<u64> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();

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
