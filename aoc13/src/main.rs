fn main() {
    let input = std::fs::read_to_string("input/input.txt").expect("Could not open input.txt");

    println!("PART 1: {}", part_1(&input));

    // The first line is irrelevant in part 2
    let iter = &mut input.split('\n');
    let _ = iter.next().unwrap();
    // println!("PART 2: {}", part2(&input));
    // println!("PART 2: {}", part_2(iter.next().unwrap()));
    let (target, mut ids) = read_file("input/input.txt");
    let p2_result = part2(&mut ids);
    println!("PART 2: {}", p2_result);
}

type ID = u64;
type Delta = u64;

fn read_file(filename: &str) -> (u64, Vec<(Delta, ID)>) {
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut parts = contents.split('\n');
    let time = parts.next().unwrap().parse::<u64>().unwrap();
    let ids: Vec<(Delta, ID)> = parts
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i as Delta, s.parse::<ID>().unwrap()))
        .collect();

    (time, ids)
}

// ID number indicates how often the bus leaves for the airport.
// What is the ID of the earliest bus you can take to the airport
// multiplied by the number of minutes you'll need to wait for that bus?
fn part_1(input: &str) -> usize {
    let mut iter = input.lines();

    let departure: i32 = iter
        .next()
        .unwrap()
        .parse::<i32>()
        .expect("Could not parse departure time");

    let mut busses: Vec<(i32, i32, bool)> = iter
        .next()
        .unwrap()
        .split(",")
        .filter(|bus| *bus != "x")
        .map(|id| (id.parse::<i32>().expect("Could not parse bus ID"), 0, false))
        .collect();

    loop {
        // 1. progress one cycle
        for (id, time, found) in &mut busses {
            if !(*found) {
                *time += *id;
            }
        }

        // 2. check if any bus is past the departure time, and set their flag if true.
        for (_, time, found) in &mut busses {
            if *time - departure >= 0 && !(*found) {
                *found = true;
            }
        }

        // 3. count how many have been found.
        let ready = busses
            .iter()
            .fold(0, |acc, (_, _, found)| if *found { acc + 1 } else { acc });

        // 4. If the number found is equal to the number of busses we have all the data we need and can break.
        if busses.len() == ready {
            break;
        }
    }

    // find the most optimal bus to take.
    // NOTE 10000, is just to have a really high value, that later can be relaxed.
    let result = busses.iter().fold((10000, 0), |best, (id, time, _)| {
        if (*time - departure) < best.0 {
            (*time - departure, *id)
        } else {
            best
        }
    });

    result.0 as usize * result.1 as usize
}

// the gcd of two numbers also divide their difference
// euclids algorithm with long division
fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b;
    } else if b == 0 {
        return a;
    }

    while a != b {
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    }
    return a;
}

fn extended_euclid_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut s, mut old_s) = (0, 1);
    let (mut t, mut old_t) = (1, 0);
    let (mut r, mut old_r) = (b, a);

    while r != 0 {
        let quotient = old_r / r;
        let tmp = r;
        r = old_r - quotient * r;
        old_r = tmp;

        let tmp = s;
        s = old_s - quotient * s;
        old_s = tmp;

        let tmp = t;
        t = old_t - quotient * t;
        old_t = tmp;
    }

    (old_r, old_s, old_t)
}

// find modulat multiplicative inverse in O(log(m))
fn modulo_multiplicative_inverse(a: i64, m: i64) -> i64 {
    // Find gcd using Extended Euclid's Algorithm
    let (gcd, mut x, y) = extended_euclid_gcd(a, m);

    // In the case x is negative, we handle it by adding an extra M.
    // Because we know that multiplicative inverse of A in range M lies
    // in the range [0, M-1]
    if x < 0 {
        x += m;
    }

    x
}

fn chinese_remainder_theorem(integers: &[i64], moduli: &[i64]) -> Result<i64, String> {
    // check if all the moduli/divisors are relatively coprime
    for i in 0..moduli.len() {
        for j in i + 1..moduli.len() {
            // println!(
            //     "gcd({}, {}) = {}",
            //     moduli[i],
            //     moduli[j],
            //     gcd(moduli[i], moduli[j])
            // );
            if gcd(moduli[i], moduli[j]) != 1 {
                return Err(format!(
                    "ERRROR: {} and {} are not relatively coprime gcd({}, {}) = {}",
                    moduli[i],
                    moduli[j],
                    moduli[i],
                    moduli[j],
                    gcd(moduli[i], moduli[j])
                ));
            }
        }
    }

    let N = moduli.iter().fold(1, |product, n| product * n);

    let n: Vec<i64> = moduli.iter().map(|i| N / i).collect();

    let x: Vec<i64> = n
        .iter()
        .zip(moduli.iter())
        .map(|(&a, &b)| modulo_multiplicative_inverse(a, b))
        .collect();

    let bnx: Vec<i64> = integers
        .iter()
        .zip(n.iter())
        .zip(x.iter())
        .map(|((&a, &b), &c)| a * b * c)
        .collect();

    Ok(bnx.iter().sum::<i64>())
}

fn part2(ids: &mut [(Delta, ID)]) -> u64 {
    // Sort and start with largest factors first
    ids.sort_by(|(_, x), (_, y)| y.cmp(x));
    let offset = ids[0].0;

    // Note that all the ids in the list are prime
    let mut base = ids[0].1;
    let mut time = base;
    for (d, id) in ids.iter().skip(1) {
        while (time + d - offset) % id != 0 {
            time += base;
        }
        // Skip ahead by the next factor
        base *= id;
    }
    return time - offset;
}

// fn part2(input: &str) -> i64 {
//     let mut busses: Vec<(i64, i64)> = input
//         .split(',')
//         .enumerate()
//         .filter(|(_, bus)| *bus != "x")
//         .map(|(i, bus)| (i as i64, bus.parse::<i64>().unwrap()))
//         .collect();

//     // sort and start with the largest factors first
//     // let ids: Vec<i64> = busses.iter().map(|(_, bus)| *bus).collect();
//     // ids.sort_by(|)

//     busses.sort_by(|(_, x), (_, y)| y.cmp(x));
//     let offset = busses[0].0;

//     // note that all the ids in the list are prime
//     let mut base = busses[0].1;
//     let mut time = base;
//     for (d, id) in busses.iter().skip(1) {
//         while (time + d - offset) % id != 0 {
//             time += base;
//         }
//         // Skip ahead by the factor
//         base *= id;
//     }
//     return time - offset;
// }

fn part_2(input: &str) -> i64 {
    let busses: Vec<(i64, i64)> = input
        .split(",")
        .enumerate()
        .filter(|(_, bus)| *bus != "x")
        // .skip(1)
        .map(|(i, id)| (id.parse::<i64>().expect("Could not parse ID"), i as i64))
        .collect();

    let moduli: Vec<i64> = busses.iter().map(|(bus, _)| *bus).collect();
    let integers: Vec<i64> = busses.iter().map(|(_, i)| *i).collect();

    println!("integers {:?}", integers);
    println!("moduli {:?}", moduli);

    chinese_remainder_theorem(&integers, &moduli).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo_multiplicative_inverse_test() {
        assert_eq!(modulo_multiplicative_inverse(5, 11), 9);
        assert_eq!(modulo_multiplicative_inverse(3, 8), 3);
        assert_eq!(modulo_multiplicative_inverse(23, 1000000007), 739130440);
    }

    #[test]
    fn chinese_remainder_theorem_test() {
        let integers = &[3, 1, 6];
        let moduli = &[5, 7, 8];
        assert_eq!(chinese_remainder_theorem(integers, moduli), Ok(918));

        // let integers = &[0, 1, 2, 3];
        // let moduli = &[67, 7, 59, 61];
        // assert_eq!(chinese_remainder_theorem(integers, moduli), Ok(3417));
    }

    #[test]
    fn part_1_example() {
        let input = ["939", "7,13,x,x,59,x,31,19"].join("\n");
        assert_eq!(part_1(&input), (944 - 939) * 59);
    }

    #[test]
    fn part_2_example() {
        // assert_eq!(part_2("7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(part2("17,x,13,19"), 3417);
        assert_eq!(part2("67,7,59,61"), 754018);
        assert_eq!(part2("67,7,x,59,61"), 779210);
        assert_eq!(part2("67,7,x,59,61"), 1261476);
        assert_eq!(part2("1789,37,47,1889"), 1202161486);
    }
}
