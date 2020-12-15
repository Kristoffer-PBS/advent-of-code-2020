use std::collections::HashMap; // for memoization

fn main() {
    let input = vec![0, 5, 4, 1, 10, 14, 7];

    // println!("PART 1: {}", part_1(input, 2020));
    println!("PART 2: {}", part_1_memoization(input, 30_000_000));
}

// the first time the number has been spoken, the current player says 0.
// Otherwise, the number had been spoken before; the current player announces
// how many turns apart the number is from when it was previously spoken.
fn part_1(mut input: Vec<usize>, nth_number: usize) -> usize {
    for _ in 0..nth_number - input.len() {
        // 1. get the last number spoken.
        let last = input.last().unwrap();
        // 2. count how many times this number has been spoken.
        let number_of_times_spoken = input
            .iter()
            .fold(0, |acc, &n| if n == *last { acc + 1 } else { acc });

        // 3. check if this is the first time the number has been spoken
        let number = if number_of_times_spoken == 1 {
            0
        } else {
            let first = input
                .iter()
                .enumerate()
                .rev()
                .find(|(_, &n)| n == *last)
                .unwrap();
            let second = input
                .iter()
                .enumerate()
                .rev()
                .skip(input.len() - first.0)
                .find(|(_, &n)| n == *last)
                .unwrap();

            first.0 - second.0
        };

        input.push(number);
    }

    *input.last().unwrap()
}

fn part_1_memoization(mut input: Vec<usize>, nth_number: usize) -> usize {
    let mut lookup_table: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, n) in input.iter().enumerate() {
        if let Some(value) = lookup_table.get_mut(n) {
            value.push(i);
        } else {
            lookup_table.insert(*n, vec![i]);
        }
    }

    for _ in 0..nth_number - input.len() {
        // 1. get the last number spoken.
        let last = input.last().unwrap();

        // 2. count how many times this number has been spoken.
        let record = lookup_table.get(last).unwrap();
        let number_of_times_spoken = record.len();

        // 3. check if this is the first time the number has been spoken
        let number = if number_of_times_spoken == 1 {
            0
        } else {
            let first = record[record.len() - 1];
            let second = record[record.len() - 2];

            first - second
        };

        // append the new number to the sequence
        input.push(number);

        // update the lookup table
        if let Some(value) = lookup_table.get_mut(&number) {
            value.push(input.len() - 1);
        } else {
            lookup_table.insert(number, vec![input.len() - 1]);
        }
    }

    *input.last().unwrap()
}

// Just using a 30e6 sized array and the previous key as index does wonders.
fn array_memoization(mut input: Vec<u32>, nth_number: u32) -> u32 {
    // preallocate the vector to speedup
    input.reserve(nth_number as usize);

    let mut lookup_list: Vec<(i32, i32)> = Vec::new();
    // -1 is used to indicate that the index has not been computed before
    lookup_list.resize(nth_number as usize, (-1, -1));

    for (i, &n) in input.iter().enumerate() {
        // if not spoken before
        let number = lookup_list.get_mut(n as usize).unwrap();
        if number == &(-1, -1) {
            number.1 = number.0; // move 0 to 1
            number.0 = i as i32;
        }

        lookup_list
        if let Some(value) = lookup_table.get_mut(n) {
            value.push(i);
        } else {
            lookup_table.insert(*n, vec![i]);
        }
    }


    2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(vec![0, 3, 6], 2020), 436);
        assert_eq!(part_1(vec![1, 3, 2], 2020), 1);
        assert_eq!(part_1(vec![1, 2, 3], 2020), 27);
        assert_eq!(part_1(vec![2, 3, 1], 2020), 78);
        assert_eq!(part_1(vec![3, 2, 1], 2020), 438);
        assert_eq!(part_1(vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn memoization() {
        assert_eq!(part_1_memoization(vec![0, 3, 6], 2020), 436);
        assert_eq!(part_1_memoization(vec![1, 3, 2], 2020), 1);
        assert_eq!(part_1_memoization(vec![1, 2, 3], 2020), 27);
        assert_eq!(part_1_memoization(vec![2, 3, 1], 2020), 78);
        assert_eq!(part_1_memoization(vec![3, 2, 1], 2020), 438);
        assert_eq!(part_1_memoization(vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn part_2_example() {
        assert_eq!(part_1(vec![0, 3, 6], 30000000), 175594);
        assert_eq!(part_1(vec![1, 3, 2], 30000000), 2578);
        assert_eq!(part_1(vec![2, 1, 3], 30000000), 3544142);
        assert_eq!(part_1(vec![1, 2, 3], 30000000), 261214);
        assert_eq!(part_1(vec![2, 3, 1], 30000000), 6895259);
        assert_eq!(part_1(vec![3, 2, 1], 30000000), 18);
        assert_eq!(part_1(vec![3, 1, 2], 30000000), 362);
    }
}
