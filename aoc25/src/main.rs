use std::fs::{File, Path};

fn main() {
    let input = read_file("input/input.txt").unwrap();

    // the loop size is secret
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

fn transform_subject_number(mut subject_number: usize, loop_size: usize) -> usize {
    let mut a = 1;
    let modulo = 20201227;

    for _ in 0..loop_size {
        a = a * subject_number;
        a = a % modulo;
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example() {
        assert_eq!(part_1(), 14897079);
    }
}
