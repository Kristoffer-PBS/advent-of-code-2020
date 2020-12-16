fn main() {
    for _ in 2..2 {
        println!("loop");
    }

    // let input = std::fs::read_to_string("input/input.txt").unwrap();
}

fn part_1(input: &str) -> usize {
    let mut iter = input.split("\n\n");

    let mut valid_values: Vec<bool> = Vec::new();
    valid_values.resize(100_000, false);

    let mut invalid_values: Vec<usize> = Vec::new();

    // step 1: parse ruleset
    for line in iter.next().unwrap().lines() {
        let mut description = line.split_whitespace().skip(1);
        let first = description.next().unwrap();
        println!("first = {}", first);
        // println!("first[0] = {}",);

        // first range before 'or'
        let (lower, upper) = {
            let delimiter = first.find("-").unwrap();
            (
                &first[..delimiter].parse::<usize>().unwrap(),
                &first[delimiter + 1..].parse::<usize>().unwrap(),
            )
        };

        for i in *lower..*upper {
            valid_values[i] = true;
        }

        // skip the 'or'
        description.next().unwrap();

        // second range after 'or'
        let second = description.next().unwrap();
        println!("second = {}", second);

        let (lower, upper) = {
            let delimiter = second.find("-").unwrap();
            println!("delimiter is {}", delimiter);
            (
                &first[..delimiter].parse::<usize>().unwrap(),
                &first[delimiter + 1..].parse::<usize>().unwrap(),
            )
        };

        for i in *lower..*upper {
            valid_values[i] = true;
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
