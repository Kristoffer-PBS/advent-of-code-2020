use crate::prelude::*;
use num_complex::Complex;

// takes an integer 'value' [0, 90, 180, 270, -90, -180, -270], and returns a complex number,
// which when multiplied by correspond to the rotation of 'value' degrees.
// example: rotate(180) = Complex::(-1, 0) = i^2.
fn rotate(value: i32) -> Result<Complex<i32>, &'static str> {
    if value % 90 != 0 {
        return Err("Modulo should be divisible by 90");
    }
    Ok(Complex::i().powi(value / 90))
}

// -----------------------------------------------------------------------------
// Part 1
// -----------------------------------------------------------------------------
// The navigation instructions consists of a sequence of single-character 'actions'
// paired with integer input 'values'.
//    - Action N means to move north by the given value.
//    - Action S means to move south by the given value.
//    - Action E means to move east by the given value.
//    - Action W means to move west by the given value.
//    - Action L means to turn left by the given number of degrees.
//    - Action R means to turn right by the given number of degrees.
//    - Action F means to move forward by the given value in the direction the
//      ship is currently facing.

// What is the Manhattan distance between the ending- and starting location of the ship.
fn part_1(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // The ship starts by facing east, at coordinates (0, 0)
    let mut position = Complex::new(0, 0);
    // north is -1, east is i, south is 1, and west is -i.
    let mut direction = Complex::i();

    for line in input.lines() {
        let mut chars = line.chars();
        let action = chars.next().ok_or("Missing action")?;
        let value = chars.as_str().parse::<i32>()?;

        match action {
            'N' => position += value,
            'S' => position -= value,
            'E' => position.im += value,
            'W' => position.im -= value,
            'L' => direction *= rotate(-value)?,
            'R' => direction *= rotate(value)?,
            'F' => position += direction * value,
            _ => return Err("Unrocognized action".into()),
        }
    }

    Ok((position.re.abs() + position.im.abs()) as usize)
}

// -----------------------------------------------------------------------------
// Part 2
// -----------------------------------------------------------------------------
//    - Action N means to move the waypoint north by the given value.
//    - Action S means to move the waypoint south by the given value.
//    - Action E means to move the waypoint east by the given value.
//    - Action W means to move the waypoint west by the given value.
//    - Action L means to rotate the waypoint around the ship left (counter-clockwise) the
//      given number of degrees.
//    - Action R means to rotate the waypoint around the ship right (clockwise) the
//      given number of degress.
//    - Action F means to move forward to the waypoint a number of times equal to the given value.

// What is the Manhattan distance between the ending- and starting location of the ship.
fn part_2(input: &str) -> Result<usize, Box<dyn std::error::Error>> {
    // The ship starts by facing east, at coordinates (0, 0)
    let mut position = Complex::new(0, 0);
    // The waypoint starts 10 units east and 1 unit north relative to the ship.
    let mut waypoint = Complex::new(10, 1);

    for line in input.lines() {
        let mut chars = line.chars();
        let action = chars.next().ok_or("Unregognized action")?;
        let value = chars.as_str().parse::<i32>()?;

        match action {
            'E' => waypoint += value,
            'W' => waypoint -= value,
            'N' => waypoint.im += value,
            'S' => waypoint.im -= value,
            'L' => waypoint *= rotate(value)?,
            'R' => waypoint *= rotate(-value)?,
            'F' => position += value * waypoint,
            _ => return Err("Unrognized action".into()),
        }
    }

    Ok((position.re.abs() + position.im.abs()) as usize)
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
    let input: String =
        std::fs::read_to_string("input/day12.txt").expect("Could not open day12.txt");
    let time_setup = start_setup.elapsed();

    // -------------------------------------------------------------------------
    // Part 1
    // -------------------------------------------------------------------------
    let start_part_1 = Instant::now();
    let part_1 = part_1(&input).unwrap();
    let time_part_1 = start_part_1.elapsed();

    // -------------------------------------------------------------------------
    // Part 2
    // -------------------------------------------------------------------------
    // Look for triple
    let start_part_2 = Instant::now();
    let part_2 = part_2(&input).unwrap();
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
    output::print_day(12);
    output::print_part(1, " Manhattan distance", &format!("{}", results.part_1));
    output::print_part(2, " Manhattan distance", &format!("{}", results.part_2));
    output::print_timing(&results.times);
}

// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_invalid_direction() {
        assert_eq!(rotate(170), Err("Modulo should be divisible by 90"));
    }

    #[test]
    fn rotate_valid_direction() {
        assert_eq!(rotate(180), Ok(Complex::new(-1, 0)));
    }

    #[test]
    fn rotate_valid_negative_direction() {
        assert_eq!(rotate(-270), Ok(Complex::new(0, 1)));
    }

    #[test]
    fn part_1_example() {
        let example = ["F10", "N3", "F7", "R90", "F11"].join("\n");
        assert_eq!(part_1(&example).unwrap(), 17 + 8);
    }

    #[test]
    fn part_2_exaple() {
        let example = ["F10", "N3", "F7", "R90", "F11"].join("\n");
        assert_eq!(part_2(&example).unwrap(), 214 + 72);
    }
}
