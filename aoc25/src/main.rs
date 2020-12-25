use std::fs::{File, Path};

fn main() {
    let input = read_file("input/input.txt").unwrap();
}

/// Return the string contents of a file
pub fn read_file<P: AsRef<Path>>(file_name: P) -> std::io::Result<String> {
    let mut file = File::open(file_name)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    Ok(data)
}

fn part_1() -> usize {
    2
}

fn transform_subject_number(subject_number: usize) -> usize {
    let divisor = 20201227;

    return loop_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(), 14897079);
    }
}
