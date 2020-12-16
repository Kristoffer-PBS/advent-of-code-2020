fn main() {
    let input = std::fs::read_to_string("input/input.txt").expect("Could not open input.txt");
    let example = std::fs::read_to_string("input/example.txt").expect("Could not open example.txt");

    assert_eq!(part_1(&example), 37);

    println!("part 1: {}", part_1(&input));
}

// '.' = floor
// 'L' = empty seat
// '#' = occupied seat

// If a seat is empty 'L' and there are NO occupied seats adjacent to it,
// the seat becomes occupied.
// If a seat is occupied '#' and four or more seats adjacent to it are also
// occupied, the seat becomes empty.
// Otherwise, the seat's state does not change.

// CELLULAR AUTOMATA

// Simulate your seating area by applying the seating rules repeatedly until
// no seats change state. How many seats end up occupied.

#[derive(Clone, PartialEq, Eq)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

#[derive(Clone)]
enum Neighbours {
    East,
    West,
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Cell {
    pub fn new(symbol: char) -> Option<Self> {
        match symbol {
            '.' => Some(Cell::Floor),
            'L' => Some(Cell::Empty),
            '#' => Some(Cell::Occupied),
            _ => None,
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(input.len());

    for i in 0..input.lines().count() {
        grid.push(vec![]);
    }

    for (i, line) in input.lines().enumerate() {
        line.chars()
            .for_each(|c| grid[i].push(Cell::new(c).unwrap()));
    }

    // keeps a copy of the previous state of the automata
    let mut copy = grid.clone();

    let mut difference = true;

    while difference {
        // print seats
        for a in 0..grid.len() {
            for b in 0..grid[0].len() {
                print!(
                    "{}",
                    match grid[a][b] {
                        Cell::Empty => 'L',
                        Cell::Floor => '.',
                        Cell::Occupied => '#',
                    }
                );
            }
            print!("\n");
        }
        println!("\n\n\n");

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                let mut alive_neighbours = 0_usize;

                let mut neighbours: Vec<Neighbours> = Vec::new();

                // handle all corner cases (I know, its ugly and bad code :D)
                if i == 0 && j == 0 {
                    neighbours.push(Neighbours::East);
                    neighbours.push(Neighbours::SouthEast);
                    neighbours.push(Neighbours::South);
                } else if i == 0 && j == grid[0].len() - 1 {
                    neighbours.push(Neighbours::West);
                    neighbours.push(Neighbours::SouthWest);
                    neighbours.push(Neighbours::South);
                } else if i == grid.len() - 1 && j == 0 {
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::NorthEast);
                    neighbours.push(Neighbours::East);
                } else if i == grid.len() - 1 && j == grid[0].len() - 1 {
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::NorthWest);
                    neighbours.push(Neighbours::West);
                } else if i == 0 {
                    neighbours.push(Neighbours::West);
                    neighbours.push(Neighbours::SouthWest);
                    neighbours.push(Neighbours::South);
                    neighbours.push(Neighbours::SouthEast);
                    neighbours.push(Neighbours::East);
                } else if i == grid.len() - 1 {
                    neighbours.push(Neighbours::West);
                    neighbours.push(Neighbours::NorthWest);
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::NorthEast);
                    neighbours.push(Neighbours::East);
                } else if j == 0 {
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::NorthEast);
                    neighbours.push(Neighbours::East);
                    neighbours.push(Neighbours::SouthEast);
                    neighbours.push(Neighbours::South);
                } else if j == grid[0].len() - 1 {
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::NorthWest);
                    neighbours.push(Neighbours::West);
                    neighbours.push(Neighbours::SouthWest);
                    neighbours.push(Neighbours::South);
                } else {
                    neighbours.push(Neighbours::East);
                    neighbours.push(Neighbours::West);
                    neighbours.push(Neighbours::North);
                    neighbours.push(Neighbours::South);
                    neighbours.push(Neighbours::NorthEast);
                    neighbours.push(Neighbours::NorthWest);
                    neighbours.push(Neighbours::SouthEast);
                    neighbours.push(Neighbours::SouthWest);
                }

                for n in neighbours {
                    let (a, b) = match n {
                        Neighbours::North => (i - 1, j),
                        Neighbours::South => (i + 1, j),
                        Neighbours::East => (i, j + 1),
                        Neighbours::West => (i, j - 1),
                        Neighbours::NorthWest => (i - 1, j - 1),
                        Neighbours::NorthEast => (i - 1, j + 1),
                        Neighbours::SouthWest => (i + 1, j - 1),
                        Neighbours::SouthEast => (i + 1, j + 1),
                    };

                    alive_neighbours += match grid[a][b] {
                        Cell::Floor => 0,
                        Cell::Empty => 0,
                        Cell::Occupied => 1,
                    }
                }

                // update cell state
                match copy[i][j] {
                    Cell::Floor => {}
                    Cell::Occupied => {
                        if alive_neighbours >= 4 {
                            copy[i][j] = Cell::Empty;
                        }
                    }
                    Cell::Empty => {
                        if alive_neighbours == 0 {
                            copy[i][j] = Cell::Occupied;
                        }
                    }
                }

                let mut foo = false;
                for a in 0..grid.len() {
                    for b in 0..grid[0].len() {
                        if grid[a][b] != copy[a][b] {
                            foo = true;
                        }
                    }
                }

                if foo {
                    difference = true;
                } else {
                    difference = false;
                }
            }
        }

        grid = copy.clone();
    }

    // return available sets
    grid.iter().flatten().fold(0, |acc, n| {
        acc + match n {
            Cell::Empty => 0,
            Cell::Floor => 0,
            Cell::Occupied => 1,
        }
    })
}

fn part_2(input: &str) -> usize {
    let mut count = 0_usize;
    count
}
