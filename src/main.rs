use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::VecDeque;
use std::iter::FromIterator;

const INPUT_FILE: &str = "input.csv";
const OUTPUT_FILE: &str = "output.csv";
const DIRECTIONS: &[(i32, i32)] = &[(0, 1), (1, 0), (0, -1), (-1, 0)];

// The main function now returns a Result
// If anything fails, the error will be returned and printed by Rust automatically
fn main() -> std::io::Result<()> {
    // Try to read the grid from the input file
    let mut grid = read_grid(INPUT_FILE)?;
    let max_color_value = grid.iter().map(|row| *row.iter().max().unwrap()).max().unwrap();
    let number_of_color_combinations = max_color_value + 1;
    let mut color_order = Vec::new();

    for _ in 0..number_of_color_combinations.pow(2) {
        let current_color = grid[0][0];

        // Search for a color that is not the current one and flood fill the grid with it
        for color in 0..number_of_color_combinations {
            if color != current_color {
                bfs(&mut grid, color);
                color_order.push(color);
                break;
            }
        }
    }
    // Try to write the color sequence to the output file
    write_colors(OUTPUT_FILE, &color_order)
}

// Read_grid now returns a Result type
// It reads a file and parses it into a 2D grid of usize values
fn read_grid(filename: &str) -> std::io::Result<Vec<Vec<usize>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut grid = Vec::new();

    for line in reader.lines() {
        let numbers_row: std::io::Result<Vec<usize>> = line?.split(',')
            .map(|s| s.parse().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
            .collect();
        grid.push(numbers_row?);
    }
    Ok(grid)
}

// Write_colors now returns a Result type
// It writes the given sequence of colors into a file, each color on its own line
fn write_colors(filename: &str, colors: &[usize]) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for &color in colors {
        writeln!(file, "{}", color)?;
    }
    Ok(())
}

// Perform a breadth-first search (BFS) starting from the top left corner
// Replace the current color with the given color
fn bfs(grid: &mut [Vec<usize>], color: usize) {
    let mut queue = VecDeque::from_iter([(0, 0)]);
    while let Some((x, y)) = queue.pop_front() {
        let current_color = grid[x][y];
        grid[x][y] = color;

        // Check all 4 directions around the current point
        for &(dx, dy) in DIRECTIONS {
            let new_x = (x as i32 + dx) as usize;
            let new_y = (y as i32 + dy) as usize;

            // If the new point is valid and has the same color as the current one, enqueue it
            if new_x < grid.len() && new_y < grid[0].len() && grid[new_x][new_y] == current_color {
                queue.push_back((new_x, new_y));
            }
        }
    }
}
