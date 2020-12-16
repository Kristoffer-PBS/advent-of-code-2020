use colored::*;

use std::{thread, time};

type Grid = Vec<Vec<char>>;

fn num_occupied_adjacent(grid: &Grid, row: i32, col: i32, max_steps: usize) -> usize {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    let mut num_adjacent = 0;
    for (row_step, col_step) in [
        (-1, 0),
        (-1, -1),
        (-1, 1),
        (1, 0),
        (1, -1),
        (1, 1),
        (0, 1),
        (0, -1),
    ]
    .iter()
    {
        let mut row_curr = row + row_step;
        let mut col_curr = col + col_step;
        let mut steps = 0;

        while row_curr >= 0
            && row_curr < height
            && col_curr >= 0
            && col_curr < width
            && steps < max_steps
        {
            let seat = grid[row_curr as usize][col_curr as usize];

            if seat == '#' {
                num_adjacent += 1;
                break;
            } else if seat == 'L' {
                break;
            }

            row_curr += row_step;
            col_curr += col_step;
            steps += 1;
        }
    }

    num_adjacent
}

fn solve_grid(grid: &Grid, min_adjacent: usize, max_steps: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut curr_grid = grid.clone();
    let mut any_change = true;

    while any_change {
        any_change = false;

        print_grid(&curr_grid, 40, 1, 500);

        let mut grid_new = curr_grid.clone();
        for row in 0..height {
            for col in 0..width {
                let seat = curr_grid[row][col];
                if seat == '.' {
                    continue;
                }

                let num_adj = num_occupied_adjacent(&curr_grid, row as i32, col as i32, max_steps);
                if seat == 'L' && num_adj == 0 {
                    grid_new[row][col] = '#';
                    any_change = true;
                } else if seat == '#' && num_adj >= min_adjacent {
                    grid_new[row][col] = 'L';
                    any_change = true;
                }
            }
        }
        curr_grid = grid_new;
    }

    curr_grid
        .iter()
        .map(|list| list.iter().filter(|&c| *c == '#').count())
        .sum()
}

fn game_of_life(grid: &Grid, min_adjacent: usize, max_steps: usize, iterations: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    let mut curr_grid = grid.clone();

    for _ in 0..iterations {
        print_grid(&curr_grid, 40, 1, 1000);

        let mut grid_new = curr_grid.clone();
        for row in 0..height {
            for col in 0..width {
                let seat = curr_grid[row][col];
                // if seat == ' ' {
                //     continue;
                // }

                let num_adj = num_occupied_adjacent(&curr_grid, row as i32, col as i32, max_steps);

                // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
                if seat == '#' && num_adj < 2 {
                    grid_new[row][col] = ' ';
                }
                // Any live cell with two or three live neighbours lives on to the next generation.
                else if seat == '#' && (num_adj == 2 || num_adj == 3) {
                }
                // Any live cell with more than three live neighbours dies, as if by overpopulation.
                else if seat == '#' && num_adj > 3 {
                    grid_new[row][col] = ' ';
                }
                // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                else if seat == ' ' && num_adj == 3 {
                    grid_new[row][col] = '#';
                }
            }
        }
        curr_grid = grid_new;
    }

    curr_grid
        .iter()
        .map(|list| list.iter().filter(|&c| *c == '#').count())
        .sum()
}

fn part_one(grid: &Grid) -> usize {
    solve_grid(grid, 4, 1)
}

fn part_two(grid: &Grid) -> usize {
    solve_grid(grid, 5, std::usize::MAX)
}

fn parse_input(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid(grid: &Grid, vertical_offset: usize, horizontal_offset: usize, update_time: u64) {
    let width = grid[0].len();
    let height = grid.len();

    for row in 0..height {
        for col in 0..width {
            let cur = grid[row][col];

            match cur {
                '.' => print!(
                    "{esq}[{row};{col}H.",
                    esq = 27 as char,
                    row = row + horizontal_offset,
                    col = col + vertical_offset
                ),
                'L' => print!(
                    "{esq}[{row};{col}H{empty}",
                    esq = 27 as char,
                    row = row + horizontal_offset,
                    col = col + vertical_offset,
                    empty = "L".green()
                ),
                '#' => print!(
                    "{esq}[{row};{col}H{occupied}",
                    esq = 27 as char,
                    row = row + horizontal_offset,
                    col = col + vertical_offset,
                    occupied = "#".red()
                ),
                _ => {}
            }
        }
        println!("");
    }

    std::thread::sleep(time::Duration::from_millis(update_time));

    clear_screen();
}

fn clear_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    // let reduced_input = std::fs::read_to_string("input/input_reduced.txt").unwrap();
    // let conway = std::fs::read_to_string("input/gospel_glider.txt").unwrap();
    // let conway_grid = parse_input(&conway);
    let grid = parse_input(&input);
    // let reduced_grid = parse_input(&reduced_input);

    // let example = [
    //     "L.LL.LL.LL",
    //     "LLLLLLL.LL",
    //     "L.L.L..L..",
    //     "LLLL.LL.LL",
    //     "L.LL.LL.LL",
    //     "L.LLLLL.LL",
    //     "..L.L.....",
    //     "LLLLLLLLLL",
    //     "L.LLLLLL.L",
    //     "L.LLLLL.LL",
    // ]
    // .join("\n");

    clear_screen();

    // assert_eq!(part_one(&parse_input(&example)), 37);

    // println!("input {}", reduced_input);

    // print_grid(&reduced_grid, 1, 1, 2);

    // part_one(&reduced_grid);
    // let part_1 = part_one(&grid);

    println!("PART 1: {}", part_one(&grid));
    // part_two(&reduced_grid);
    // game_of_life(&conway_grid, 1, 1, 10000);
    // println!("PART 2: {}", part_two(&grid));
}
