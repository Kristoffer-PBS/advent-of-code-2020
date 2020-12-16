// use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input file");
    let example = fs::read_to_string("input/example.txt").expect("Could not open example file");
    let exaple_2 =
        fs::read_to_string("input/example_2.txt").expect("Could not open example_2 file");

    let mut solution = Solution::new();
    if let Ok(_) = solution.parse(example.to_string()) {}

    assert_eq!(solution.part_1(&example, "shiny gold"), 4);

    let mut solution = Solution::new();
    solution.parse(input.to_string()).unwrap();

    println!("part 1: {}", solution.part_1(&input, "shiny gold"));

    let s2 = Solution2::new(example).unwrap();
    assert_eq!(s2.part_2("shiny gold").unwrap(), 32);
    // s2.print();
    let s2 = Solution2::new(exaple_2).unwrap();
    assert_eq!(s2.part_2("shiny gold").unwrap(), 126);
    // s2.print();

    let s2 = Solution2::new(input).unwrap();
    println!("part 2: {}", s2.part_2("shiny gold").unwrap());
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
                // .filter_map(|c| c.to_digit(10) as usize)
                // .collect();
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
                // total += self.number_of_bags(bag.1);

                // if bag == query {
                //     println!("{:?}", bags);
                //     return 1;
                // } else {
                //     total += self.bag_count(bag, query);
                //     // return self.bag_count(bag, query);
                // }
            }
            // if total > 0 {
            //     return 1;
            // } else {
            //     return 0;
            // }

            return total;
        }

        println!("SHOULD NOT BE HERE------------------------------------------------");

        0
    }

    pub fn part_2(&self, query: &str) -> Option<usize> {
        let mut total = 0_usize;

        let _ = self.rule_set.get(query)?;

        total = self.number_of_bags(query);

        // println!("THE len is {}", self.rule_set.len());

        // for (key, _) in &self.rule_set {
        //     // println!("bag is {}", key);
        //     total += self.bag_count(key, query);
        // }

        Some(total)
    }
}

struct Solution {
    // rule_set: HashMap<String, Vec<Option<String>>>,
    rule_set: HashMap<String, Vec<String>>,
}

impl Solution {
    pub fn new() -> Solution {
        Solution {
            rule_set: HashMap::new(),
        }
    }

    // pub fn parse(&mut self, input: String) -> Result<String, Box<dyn std::error::Error>> {
    //     self.rule_set.reserve(input.len());

    //     for line in input.lines() {
    //         // let rgx = Regex::new("[0-9]+").unwrap();
    //         let v: Vec<&str> = line.split(char::is_numeric).collect();

    //         // println!("{}", v.len());
    //         if v.len() == 1 {
    //             if let Some(index) = v[0].find("bags") {
    //                 self.rule_set
    //                     .insert(v[0][..index - 1].to_string(), vec![None]);
    //             }

    //         // println!("NONE");
    //         } else {
    //             let mut tmp: Vec<Option<String>> = Vec::new();

    //             for color in &v[1..] {
    //                 // println!("{}", color);
    //                 if let Some(index) = color.find("bags") {
    //                     tmp.push(Some(color[1..index - 1].to_string()));
    //                 } else if let Some(index) = color.find("bag") {
    //                     tmp.push(Some(color[1..index - 1].to_string()));
    //                 }
    //             }
    //             // println!("NEXT\n");

    //             if let Some(index) = v[0].find("bags") {
    //                 self.rule_set.insert(v[0][..index - 1].to_string(), tmp);
    //             }
    //         }
    //     }

    //     Ok("it parsed".to_string())
    // }

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
                    // println!("{:?}", bags);
                    return 1;
                } else {
                    total += self.bag_count(bag, query);
                    // return self.bag_count(bag, query);
                }
            }
            if total > 0 {
                return 1;
            } else {
                return 0;
            }
        }

        println!("SHOULD NOT BE HERE------------------------------------------------");

        0
    }

    // pub fn bag_count(&self, start: &str, query: &str) -> usize {
    //     if let Some(bags) = self.rule_set.get(start) {
    //         for bag in bags {
    //             if let Some(content) = bag {
    //                 if content == query {
    //                     return 1;
    //                 } else {
    //                 }
    //             }
    //         }
    //     }

    //     2
    // }

    pub fn part_1(&self, input: &str, query: &str) -> usize {
        let mut total = 0_usize;

        // println!("THE len is {}", self.rule_set.len());

        for (key, _) in &self.rule_set {
            // println!("bag is {}", key);
            total += self.bag_count(key, query);
        }

        total
    }
}
