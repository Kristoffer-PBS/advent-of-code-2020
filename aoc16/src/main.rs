use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();

    println!("PART 1: {}", part_1(&input));
}

fn parse_range(input: &str) -> Option<(usize, usize)> {
    let delimiter = input.find('-')?;

    Some((
        input[..delimiter].parse::<usize>().unwrap(),
        input[delimiter + 1..].parse::<usize>().unwrap(),
    ))
}

// fn parse_tickets(input: &str) -> Vec<bool> {
//     let mut iter = input.split("\n\n");

//     let mut valid_values: Vec<bool> = Vec::new();
//     valid_values.resize(100_000, false);

//     let mut invalid_values: Vec<usize> = Vec::new();

//     // step 1: parse ruleset
//     for line in iter.next().unwrap().lines() {
//         let mut description = line.split_whitespace().skip(1);

//         // first range before 'or'
//         if let Some((lower, upper)) = parse_range(description.next().unwrap()) {
//             for i in lower..=upper {
//                 valid_values[i] = true;
//             }
//         }

//         // skip the 'or'
//         description.next().unwrap();

//         // second range after 'or'
//         if let Some((lower, upper)) = parse_range(description.next().unwrap()) {
//             for i in lower..=upper {
//                 valid_values[i] = true;
//             }
//         }
//     }

//     valid_values
// }

fn part_1(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let mut valid_values: Vec<bool> = Vec::new();
    valid_values.resize(100_000, false);

    let mut invalid_values: Vec<usize> = Vec::new();

    // step 1: parse ruleset
    for line in iter.next().unwrap().lines() {
        let mut description = line.split_whitespace().skip(1);

        // first range before 'or'
        if let Some((lower, upper)) = parse_range(description.next().unwrap()) {
            for i in lower..=upper {
                valid_values[i] = true;
            }
        }

        // skip the 'or'
        description.next().unwrap();

        // second range after 'or'
        if let Some((lower, upper)) = parse_range(description.next().unwrap()) {
            for i in lower..=upper {
                valid_values[i] = true;
            }
        }
    }

    // step 2: check your ticket
    for ticket in iter.next().unwrap().split("\n").next().unwrap().split(',') {
        if let Ok(ticket) = ticket.parse::<usize>() {
            if !valid_values[ticket] {
                invalid_values.push(ticket);
            }
        }
    }

    // step 3: check other tickets
    for ticket in iter.next().unwrap().lines() {
        for ticket in ticket.split(',') {
            if let Ok(ticket) = ticket.parse::<usize>() {
                if !valid_values[ticket] {
                    invalid_values.push(ticket);
                }
            }
        }
    }

    // step 4: return sum of all invalid tickets
    invalid_values.iter().sum()
}

#[derive(Debug)]
struct TicketField {
    // name: String,
    first: (usize, usize),
    second: (usize, usize),
}

impl TicketField {
    pub fn new(name: String, first: (usize, usize), second: (usize, usize)) -> Self {
        Self {
            // name,
            first,
            second
        }
    }
    
    // This is some janky syntax!
    pub fn from_line(line: &str) -> Option<Self> {
        let mut iter = line.split_whitespace().skip(1);
        Some(Self {
            // name: iter.next()?.to_string(),
            first: parse_range(iter.next()?)?,
            second: parse_range(iter.next()?)?
        })
    }

    pub fn contains(&self, value: usize) -> bool {
        if value >= self.first.0 && value <= self.first.1 {
            true
        } else if value >= self.second.0 && value <= self.second.1 {
            true
        } else {
            false
        }
    }
}


// look for the six fields on your ticket that starts with the word 'departure' 
// What do you get if you multiply those six values together?
fn part_2(input: &str) -> usize {
    let mut iter = input.split("\n\n");
    
    // TODO
    // 1. discard the ticket which contains invalid values 

    let mut valid_values: Vec<bool> = Vec::new();
    valid_values.resize(100_000, false);

    // let mut invalid_values: Vec<usize> = Vec::new();

    let mut map: HashMap<&str, TicketField> = HashMap::new();


    // step 1: parse ruleset
    for line in iter.next().unwrap().lines() {
        map.insert(line, TicketField::from_line(line).unwrap());
    }

    hah.iter.product()

    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        let rules = [
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
        ]
        .join("\n");

        let your_ticket = ["your_ticket:", "7,1,14"].join("\n");
        let nearby_tickets =
            ["nearby_tickets:", "7,3,47", "40,4,50", "55,2,20", "38,6,12"].join("\n");

        let input = [rules, your_ticket, nearby_tickets].join("\n\n");

        println!("{}", input);

        assert_eq!(part_1(&input), 4 + 55 + 12);
    }
}
