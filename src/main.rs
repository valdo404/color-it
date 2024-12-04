use nalgebra::{DMatrix};
use std::collections::{HashSet};
use std::error::Error;
use clap::{Arg, ArgAction, Command};

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    colors: usize,
    data: DMatrix<u8>,  // 2D matrix to represent the grid
}

impl Grid {
    fn new(width: usize, height: usize, colors: usize) -> Self {
        Grid {
            width,
            height,
            colors,
            data: DMatrix::from_element(height, width, 0),  // Initialize with default color (0)
        }
    }

    fn apply_solution(&mut self, solution: &[u8]) -> bool {
        for &color in solution {
            self.flood_fill(color);
        }
        self.is_complete()
    }

    fn print_stats(&self) {
        println!("Grid Statistics:");
        println!("  Size: {}x{}", self.width, self.height);
        println!("  Cells: {}", self.width * self.height);
        println!("  Colors: {}", self.colors);
        println!("  Search Space: {}", (self.colors as f64).powf((self.width * self.height) as f64));
    }

    fn from_csv(content: &str) -> Option<Self> {
        let rows: Vec<&str> = content.trim().split('\n').collect();
        let height = rows.len();
        let width = rows.get(0)?.split(',').count(); // Get the number of columns from the first row

        let mut data = DMatrix::zeros(height, width);
        let mut colors = 0;

        for (i, row) in rows.iter().enumerate() {
            for (j, val) in row.split(',').enumerate() {
                let color = val.trim().parse::<u8>().ok()?;
                colors = colors.max(color as usize + 1);
                data[(i, j)] = color;
            }
        }

        Some(Grid { width, height, colors, data })
    }

    fn to_csv(&self) -> String {
        let mut result = String::new();
        for i in 0..self.height {
            for j in 0..self.width {
                if j > 0 {
                    result.push(',');
                }
                result.push_str(&self.data[(i, j)].to_string());
            }
            result.push('\n');
        }
        result
    }

    fn flood_fill(&mut self, target_color: u8) {
        // Ensure grid dimensions are valid
        assert!(self.width > 0, "Width must be greater than zero");
        assert!(self.height > 0, "Height must be greater than zero");

        let source_color = self.data[(0, 0)]; // Starting color is the color at position (0, 0)

        // If the source color is the same as the target color, no need to do anything
        if source_color == target_color {
            println!("Source color {} is the same as target color {}", source_color, target_color);
            return;
        }

        // Use a stack to implement depth-first search (DFS)
        let mut stack = Vec::new();
        stack.push((0, 0)); // Start from the top-left corner

        // Create a visited set to avoid revisiting cells
        let mut visited = vec![vec![false; self.width]; self.height];

        while let Some((x, y)) = stack.pop() {
            // Ensure that the index is within the grid bounds
            assert!(x < self.width && y < self.height, "Index out of bounds: ({}, {})", x, y);

            // Skip if already visited
            if visited[y][x] {
                continue;
            }

            // Check if the current cell has the source color
            if self.data[(y, x)] == source_color {
                // Fill the current cell with the target color
                self.data[(y, x)] = target_color;
                visited[y][x] = true; // Mark as visited

                // Left
                if x > 0 {
                    if !visited[y][x - 1] && self.data[(y, x - 1)] == source_color {
                        stack.push((x - 1, y));
                    }
                }

                // Right
                if x < self.width - 1 {
                    if !visited[y][x + 1] && self.data[(y, x + 1)] == source_color {
                        stack.push((x + 1, y));
                    }
                }

                // Up
                if y > 0 {
                    if !visited[y - 1][x] && self.data[(y - 1, x)] == source_color {
                        stack.push((x, y - 1));
                    }
                }

                // Down
                if y < self.height - 1 {
                    if !visited[y + 1][x] && self.data[(y + 1, x)] == source_color {
                        stack.push((x, y + 1));
                    }
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        let target = self.data[(0, 0)];
        self.data.iter().all(|&color| color == target)
    }
}

fn solve(grid: &mut Grid, output_grids: bool) -> Vec<u8> {
    grid.print_stats();
    if(output_grids) {
        println!("Initial grid:\n{}", grid.data);
    }

    if grid.is_complete() {
        return Vec::new();
    }

    struct SearchState {
        moves: Vec<u8>,
        grid_state: DMatrix<u8>, // Grid state as a matrix
    }

    let mut stack: Vec<SearchState> = Vec::new();
    let mut visited: HashSet<DMatrix<u8>> = HashSet::new();
    let mut best_solution = Vec::new();
    let mut min_length = grid.width * grid.height;

    // Initialize the stack with the first moves
    for color in 0..grid.colors {
        let color = color as u8;
        if color != grid.data[(0, 0)] {
            let mut temp_grid = grid.clone();
            temp_grid.flood_fill(color);

            if visited.insert(temp_grid.data.clone()) {
                stack.push(SearchState {
                    moves: vec![color],
                    grid_state: temp_grid.data.clone(),
                });
            }
        }
    }

    // Perform the search
    while let Some(state) = stack.pop() {
        if state.moves.len() >= min_length {
            continue;
        }

        // Reconstruct the grid from the current state
        let temp_grid = Grid {
            width: grid.width,
            height: grid.height,
            colors: grid.colors,
            data: state.grid_state.clone(),
        };

        // Check if the grid is complete
        if temp_grid.is_complete() {
            if state.moves.len() < min_length {
                best_solution = state.moves.clone();
                min_length = state.moves.len();

                if output_grids {
                    let mut original_grid = grid.clone();

                    for &color in &best_solution {
                        original_grid.flood_fill(color);
                        if output_grids {
                            println!("Applying move: {}, Current grid state:\n{}", color, original_grid.data);
                        }
                    }
                }
            }
            continue;
        }

        // Add next possible moves
        for color in 0..grid.colors {
            let color = color as u8;

            // Skip moves that repeat the current color or backtrack
            if color == temp_grid.data[(0, 0)] || (state.moves.last() == Some(&color)) {
                continue;
            }

            let mut next_grid = temp_grid.clone();
            next_grid.flood_fill(color);

            // Only consider this move if it results in a new grid state
            if visited.insert(next_grid.data.clone()) {
                let mut next_moves = state.moves.clone();
                next_moves.push(color);

                if(output_grids) {
                    // println!("Applying move: {}, Current grid state:\n{}", color, next_grid.data);
                }

                stack.push(SearchState {
                    moves: next_moves,
                    grid_state: next_grid.data.clone(),
                });
            }
        }
    }

    if(output_grids) {
        println!("Final solution: {:?}", best_solution);
    }

    best_solution
}

fn save_solution(moves: &[u8], output_file: Option<&str>) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    for &m in moves {
        output.push_str(&m.to_string());
        output.push('\n');
    }

    if let Some(file) = output_file {
        std::fs::write(file, output)?;
    } else {
        println!("Solution: {:?}", moves);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("Grid Coloring Solver")
        .version("1.0")
        .author("Laurent Valdes <valderama@gmail.com>")
        .about("Solves a grid coloring problem")
        .arg(
            Arg::new("input")
                .short('i')
                .default_value("small-grid.csv")
                .long("input")
                .value_name("FILE")
                .help("Input CSV file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .default_value("output.csv")
                .long("output")
                .value_name("FILE")
                .help("Output file for the solution"),
        )
        .arg(
            Arg::new("output-grids")
                .short('g')
                .long("output-grids")
                .action(ArgAction::SetTrue)
                .help("Output the grids to the console"),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").expect("required input file");
    let output_file = matches.get_one::<String>("output");
    let output_grids = matches.get_flag("output-grids");

    let input = std::fs::read_to_string(input_file)?;
    let mut grid = Grid::from_csv(&input).unwrap();
    let solution = solve(&mut grid, output_grids);
    save_solution(&solution, output_file.map(|x| x.as_str()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplest_input() {
        let input = "0,1\n1,1";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid, false);
        assert_eq!(solution, vec![1]);
    }

    #[test]
    fn test_medium_input() {
        let input = "2,1,3,0,4\n1,2,2,3,1\n0,3,1,2,4\n4,1,0,3,2\n3,2,4,1,0";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid, false);
        let mut test_grid = grid.clone();
        for &color in &solution {
            test_grid.flood_fill(color);
        }
        assert!(test_grid.is_complete());
        assert!(solution.len() <= 16);
    }

    #[test]
    fn test_sample_input() {
        let input = "1,2,0,0\n0,1,1,0\n2,2,0,1\n0,0,0,1";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid, false);
        assert_eq!(solution, vec![2, 1, 2, 0, 1]);

        let mut test_grid = grid.clone();
        assert!(test_grid.apply_solution(&solution));
    }

    #[test]
    fn test_flood_fill() {
        let mut grid = Grid::new(3, 3, 3);
        let original = DMatrix::from_vec(3, 3, vec![0, 0, 0, 1, 1, 1, 1, 0, 0]);
        grid.data = original.clone();
        grid.flood_fill(2);
        let expected = DMatrix::from_vec(3, 3, vec![2, 2, 2, 1, 1, 1, 1, 0, 0]);

        assert_eq!(grid.data, expected);
    }
}
