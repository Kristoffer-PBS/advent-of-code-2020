use crate::prelude::*;
use std::collections::HashMap;

struct Solution {
    rule_set: HashMap<String, Vec<String>>,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            rule_set: HashMap::new(),
        }
    }

    pub fn parse(&mut self, input: String) -> Result<String, Box<dyn std::error::Error>> {
        self.rule_set.reserve(input.len());

        for line in input.lines() {
            let bags: Vec<&str> = line.split(char::is_numeric).collect();

            if bags.len() == 1 {
                if let Some(index) = bags[0].find("bags") {
                    self.rule_set
                        .insert(bags[0][..index - 1].to_string(), vec![]);
                }
            } else {
                let mut tmp: Vec<&str> = vec![];

                for bag in &bags[1..] {
                    if let Some(index) = bag.find("bags") {
                        tmp.push(&bag[1..index - 1]);
                    } else if let Some(index) = bag.find("bag") {
                        tmp.push(&bag[1..index - 1]);
                    }
                }

                if let Some(index) = bags[0].find("bags") {
                    let tmp: Vec<String> = tmp.iter().map(|s| s.to_string()).collect();
                    self.rule_set.insert(bags[0][..index - 1].to_string(), tmp);
                }
            }
        }

        Ok("it parsed".to_string())
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for item in &self.rule_set {
            println!("{:?}", item);
        }
    }

    pub fn bag_count(&self, start: &str, query: &str) -> usize {
        if let Some(bags) = self.rule_set.get(start) {
            if bags.is_empty() {
                return 0;
            }
            let mut total = 0_usize;
            for bag in bags {
                if bag == query {
                    return 1;
                } else {
                    total += self.bag_count(bag, query);
                }
            }
            if total > 0 {
                return 1;
            } else {
                return 0;
            }
        }
        0
    }

    pub fn part_1(&self, input: &str, query: &str) -> usize {
        let mut total = 0_usize;

        for (key, _) in &self.rule_set {
            total += self.bag_count(key, query);
        }

        total
    }
}

struct Solution2 {
    rule_set: HashMap<String, Vec<(usize, String)>>,
}

impl Solution2 {
    pub fn new(input: String) -> Result<Solution2, Box<dyn std::error::Error>> {
        let mut rule_set: HashMap<String, Vec<(usize, String)>> =
            HashMap::with_capacity(input.len());

        // zip an iterator pair
        for line in input.lines() {
            let amounts: Vec<usize> = line
                .chars()
                .filter(|c| c.is_digit(10))
                .map(|n| n.to_digit(10).unwrap() as usize)
                .collect();

            let bags: Vec<&str> = line.split(char::is_numeric).collect();

            if bags.len() == 1 {
                if let Some(index) = bags[0].find("bags") {
                    rule_set.insert(bags[0][..index - 1].to_string(), vec![]);
                }
            } else {
                let mut tmp: Vec<(usize, String)> = vec![];

                // rust iterators are amazing
                let mut iter = bags.iter().skip(1).zip(amounts.iter());

                while let Some((bag, &i)) = iter.next() {
                    if let Some(index) = bag.find("bags") {
                        tmp.push((i, bag[1..index - 1].to_string()));
                    } else if let Some(index) = bag.find("bag") {
                        tmp.push((i, bag[1..index - 1].to_string()));
                    }
                }

                if let Some(index) = bags[0].find("bags") {
                    rule_set.insert(bags[0][..index - 1].to_string(), tmp);
                }
            }
        }

        Ok(Solution2 { rule_set })
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for item in &self.rule_set {
            println!("{:?}", item);
        }
    }

    pub fn number_of_bags(&self, start: &str) -> usize {
        let mut total = 0_usize;

        if let Some(bags) = self.rule_set.get(start) {
            if bags.is_empty() {
                return 0;
            }

            for bag in bags {
                total += bag.0 + bag.0 * self.number_of_bags(&bag.1);
            }

            return total;
        }

        0
    }

    pub fn part_2(&self, query: &str) -> Option<usize> {
        // let mut total = 0_usize;

        let _ = self.rule_set.get(query)?;

        let total = self.number_of_bags(query);

        Some(total)
    }
}

pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input: String =
        std::fs::read_to_string("input/day07.txt").expect("Could not open day07.txt");

    let mut solution = Solution::new();
    solution.parse(input.to_string()).unwrap();
    let s2 = Solution2::new(input.clone()).unwrap();
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    let start_part_1 = Instant::now();
    let part_1 = solution.part_1(&input, "shiny gold");
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------

    let start_part_2 = Instant::now();
    let part_2 = s2.part_2("shiny gold").unwrap();
    let time_part_2 = start_part_2.elapsed();

    // -------------------------------------------------------------------------
    // Return
    // -------------------------------------------------------------------------
    Results::new(
        part_1 as i64,
        part_2 as i64,
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
    output::print_day(7);
    output::print_part(
        1,
        " at least one 'shiny gold' bag",
        &format!("{}", results.part_1),
    );
    output::print_part(2, " required bags", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
