// use bit_vec::*:

use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").expect("Could not open input.txt");

    println!("PART 1: {}", part_1(&input));
}

enum Instruction {
    Mask,
    Mem(usize),
}

// What is the sum of all values left in memory after it completes?
fn part_1(input: &str) -> usize {
    // let input: Vec<&str> = input
    //     .lines()
    //     .collect();

    let mut tape: Vec<(Instruction, usize)> = Vec::with_capacity(input.len());

    for line in input.lines() {
        let mut line = line.split(" = ");

        let mut a: usize = 0;

        let instruction = line.next().unwrap();
        if let Some(start) = instruction.find("[") {
            let end = instruction.find("]").unwrap();
            let address = &instruction[start..end].parse::<usize>().unwrap();
            tape.push()
        }

        let value = line.next().unwrap();
    }

    2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn part_1_example() {}
}
