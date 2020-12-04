use regex::Regex;
use std::fs;

fn main() {
    let input = fs::read_to_string("input/input.txt").expect("Could not open input file");
    let example = fs::read_to_string("src/example.txt").expect("Could not open example file");

    assert_eq!(2, part_1(&example));

    println!("part 1: {}", part_1(&input));

    println!("\n\npart 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut count = 0_usize;

    let required_information = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    // split each passport information
    let passports: Vec<&str> = input.split("\n\n").collect();

    for passport in passports {
        let mut total = 0_usize;
        for pattern in &required_information {
            if let Some(_) = passport.find(pattern) {
                total += 1;
            }
        }

        if total == required_information.len() {
            count += 1;
        }
    }

    count
}

/// cid (Country ID) - ignored, missing or not.
fn part_2(input: &str) -> usize {
    let mut count = 0_usize;

    // split each passport information
    let passports: Vec<&str> = input.split("\n\n").collect();

    for passport in passports {
        let mut total = 0_usize;

        // byr (Birth Year) - four digits; at least 1920 and at most 2002
        if let Some(byr) = passport.find("byr") {
            if let Ok(year) = &passport[byr + 1 + 3..byr + 1 + 3 + 4].parse::<usize>() {
                if *year >= 1920 as usize && *year <= 2002 as usize {
                    total += 1;
                }
            }
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020
        if let Some(iyr) = passport.find("iyr") {
            if let Ok(year) = &passport[iyr + 1 + 3..iyr + 1 + 3 + 4].parse::<usize>() {
                if *year >= 2010 as usize && *year <= 2020 {
                    total += 1;
                }
            }
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030
        if let Some(eyr) = passport.find("eyr") {
            if let Ok(year) = &passport[eyr + 1 + 3..eyr + 1 + 3 + 4].parse::<usize>() {
                if *year >= 2020 && *year <= 2030 {
                    total += 1;
                }
            }
        }

        // hgt (Height) - a number followed by either cm or in
        //     - If cm, the number must be at least 150 and at most 193
        //     - if in, the number must be at least 59 and at most 76
        if let Some(hgt) = passport.find("hgt") {
            if let Some(cm) = &passport[hgt + 1 + 3..].find("cm") {
                if let Ok(h) = &passport[hgt + 1 + 3..hgt + 1 + 3 + *cm].parse::<usize>() {
                    if *h >= 150 && *h <= 193 {
                        total += 1;
                    }
                }
            } else if let Some(inch) = &passport[hgt + 1 + 3..].find("in") {
                if let Ok(h) = &passport[hgt + 1 + 3..hgt + 1 + 3 + *inch].parse::<usize>() {
                    if *h >= 59 && *h <= 76 {
                        total += 1;
                    }
                }
            }
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f
        if let Some(hcl) = passport.find("hcl") {
            let rgx = Regex::new(r"^#[0-9a-f]{6}").unwrap();
            if rgx.is_match(&passport[hcl + 1 + 3..]) {
                total += 1;
            }
        }
        // ecl (Eye Color) - exactly one of: amb, blu, brn, gry, grn, hzl, oth
        if let Some(ecl) = passport.find("ecl") {
            total += match &passport[ecl + 1 + 3..ecl + 1 + 3 + 3] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => 1,
                _ => 0,
            };
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes
        if let Some(pid) = passport.find("pid") {
            let mut local_count = 0_usize;
            for ch in passport[pid + 1 + 3..].chars() {
                match ch {
                    '0'..='9' => local_count += 1,
                    _ => break,
                };
            }

            if local_count == 9 {
                total += 1;
            }
        }

        println!("total is {}", total);

        if total == 7 {
            count += 1;
        }
    }

    count
}
