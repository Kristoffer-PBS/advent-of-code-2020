use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input file");

    // each line in a group represents the number of people in that group
    // the number of characters in a line represents the number of questions they answered yes to.
    // repeated answers for each group is not counted multiple times

    let example = "abc

        a
        b
        c

        ab
        ac

        a
        a
        a
        a

        b";

    assert_eq!(part_1(example), 11);

    println!("part 1: {}", part_1(&input));

    assert_eq!(part_2(example), 6);

    println!("part 2: {}", part_2(&input));
}

// For each group, count the number of questions to which anyone answered "yes".
// What is the sum of those counts?
fn part_1(input: &str) -> usize {
    let mut sum = 0_usize;

    // go through each group
    for group in input.split("\n\n") {
        let mut set: HashSet<char> = HashSet::with_capacity(group.len());

        sum += group
            .chars()
            .filter(|c| c.is_ascii_alphabetic()) // filter out escape characters: '\r' and '\n'
            .filter(|&c| set.insert(c)) // use the uniqueness property of a set to filter
            .count();
    }
    return sum;
}

fn part_2(input: &str) -> usize {
    let mut sum = 0_usize;

    // go through each group
    for group in input.split("\n\n") {
        // create a set containing the different questions, which have been answered "yes"
        let questions_answered: HashSet<char> =
            group.chars().filter(|c| c.is_ascii_alphabetic()).collect();

        let mut answers: HashMap<char, usize> = HashMap::new();

        // for each question type, use a map to count how many time a question has been answered.
        for letter in group.chars().filter(|c| c.is_ascii_alphabetic()) {
            let counter = answers.entry(letter).or_insert(0);
            *counter += 1;
        }

        for (key, value) in &answers {
            if questions_answered.contains(key) && *value == group.lines().count() {
                sum += 1;
            }
        }
    }

    sum
}
