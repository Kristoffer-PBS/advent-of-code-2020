use crate::prelude::*;

#[derive(Debug)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    // if self is Instruction::Jmp change to Instruction::Nop, and vice versa.
    // if self is Instruction::Acc, do nothing
    // FIXME does nothing apparently
    #[allow(dead_code)]
    pub fn change_instruction(&mut self, new: Instruction) {
        match *self {
            Instruction::Acc(_) => {}
            Instruction::Jmp(_) | Instruction::Nop(_) => match new {
                Instruction::Jmp(arg) => {
                    *self = Instruction::Nop(arg);
                }
                Instruction::Nop(arg) => {
                    *self = Instruction::Jmp(arg);
                }
                Instruction::Acc(_) => {}
            },
        }
    }
}

#[allow(dead_code)]
fn print_instruction_set(tape: &Vec<(Instruction, bool)>) {
    for instruction in tape {
        println!("{:?}", instruction);
    }
}

// TODO
// Refactor
fn parse(input: &str) -> Vec<(Instruction, bool)> {
    let mut tape: Vec<(Instruction, bool)> = Vec::with_capacity(input.len());

    // parse instructions
    for line in input.lines() {
        let mut iter = line.split_whitespace();

        match iter.next().unwrap() {
            "acc" => {
                match iter.next() {
                    Some(sign) => {
                        let mut idx = sign.chars();
                        match idx.next().unwrap() {
                            '+' => {
                                tape.push((
                                    Instruction::Acc(sign[1..].parse::<i32>().unwrap()),
                                    false,
                                ));
                            }
                            '-' => {
                                tape.push((
                                    Instruction::Acc(sign[1..].parse::<i32>().unwrap() * -1),
                                    false,
                                ));
                            }
                            _ => {
                                panic!("Could not parse instruction argument")
                            }
                        }
                    }
                    None => {
                        panic!("Instruction has no argumnet")
                    }
                };
            }

            "jmp" => {
                match iter.next() {
                    Some(sign) => match sign.chars().next().unwrap() {
                        '+' => {
                            tape.push((Instruction::Jmp(sign[1..].parse::<i32>().unwrap()), false));
                        }
                        '-' => {
                            tape.push((
                                Instruction::Jmp(sign[1..].parse::<i32>().unwrap() * -1),
                                false,
                            ));
                        }
                        _ => {
                            panic!("Could not parse instruction argument")
                        }
                    },
                    None => {
                        panic!("Instruction has no argument")
                    }
                };
            }

            "nop" => {
                match iter.next() {
                    Some(sign) => match sign.chars().next().unwrap() {
                        '+' => {
                            tape.push((Instruction::Nop(sign[1..].parse::<i32>().unwrap()), false));
                        }
                        '-' => {
                            tape.push((
                                Instruction::Nop(sign[1..].parse::<i32>().unwrap() * -1),
                                false,
                            ));
                        }
                        _ => {
                            panic!("Could not parse instruction argument")
                        }
                    },
                    None => {
                        panic!("Instruction has no argument")
                    }
                };
            }

            _ => {
                panic!("invalid instruction")
            }
        };
    }

    tape
}

fn part_1(input: &str) -> i32 {
    let mut tape = parse(input);

    let mut acc = 0_i32; // accumelator
    let mut index = 0_i32;

    loop {
        // if the instruction has allready been visisted we break
        if tape[index as usize].1 {
            break;
        } else {
            // set flag to visited
            tape[index as usize].1 = true;

            match tape[index as usize].0 {
                Instruction::Acc(arg) => {
                    acc += arg;
                    index += 1;
                }
                Instruction::Jmp(arg) => index += arg,
                Instruction::Nop(_) => index += 1,
            }
        }
    }

    acc
}

fn part_2(input: &str) -> i32 {
    let mut tape = parse(input);
    let len = tape.len();

    let mut acc = 0_i32; // accumelator

    for i in 0..len {
        // try new change
        match tape[i].0 {
            Instruction::Acc(_) => {}
            Instruction::Jmp(arg) => {
                tape[i].0 = Instruction::Nop(arg);
                // tape[i].0.change_instruction(Instruction::Nop(arg));
            }
            Instruction::Nop(arg) => {
                tape[i].0 = Instruction::Jmp(arg);
                // tape[i].0.change_instruction(Instruction::Jmp(arg));
            }
        }

        // test
        let mut index = 0_i32;

        // run instruction set
        while index as usize != len - 1 {
            if tape[index as usize].1 {
                break;
            } else {
                tape[index as usize].1 = true;

                match tape[index as usize].0 {
                    Instruction::Acc(arg) => {
                        acc += arg;
                        index += 1;
                    }
                    Instruction::Jmp(arg) => index += arg,
                    Instruction::Nop(_) => index += 1,
                }
            }
        }

        // if the end of instruction was reached, we have found the right
        // instruction to change, and we break
        if index as usize == len - 1 {
            break;
        }

        // reset accumelator
        acc = 0;

        // if not the answer then change back
        match tape[i].0 {
            Instruction::Acc(_) => {}
            Instruction::Jmp(arg) => {
                tape[i].0 = Instruction::Nop(arg);
                // tape[i].0.change_instruction(Instruction::Nop(arg));
            }
            Instruction::Nop(arg) => {
                tape[i].0 = Instruction::Jmp(arg);
                // tape[i].0.change_instruction(Instruction::Jmp(arg));
            }
        }

        // reset visited instruction flag
        for j in 0..len {
            tape[j].1 = false;
        }
    }

    acc
}

// -----------------------------------------------------------------------------
// Run
// -----------------------------------------------------------------------------
pub(crate) fn run() -> Results {
    // -------------------------------------------------------------------------
    // Setup
    // -------------------------------------------------------------------------
    // Open file
    let start_setup = Instant::now();
    let input = std::fs::read_to_string("input/day08.txt").expect("Could not open day08.txt");
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------

    let start_part_1 = Instant::now();
    let part_1 = part_1(&input);
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------

    let start_part_2 = Instant::now();
    let part_2 = part_2(&input);
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
    output::print_day(8);
    output::print_part(1, "樂accumelator value", &format!("{}", results.part_1));
    output::print_part(
        2,
        "樂 accumelator value after program terminates",
        &format!("{}", results.part_2),
    );
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------
