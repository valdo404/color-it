use std::collections::{HashSet, VecDeque};
use std::error::Error;

#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    colors: usize,
    data: Vec<u8>,
}

impl Grid {
    fn new(size: usize, colors: usize) -> Self {
        Grid {
            size,
            colors,
            data: vec![0; size * size],
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
        println!("  Size: {}x{}", self.size, self.size);
        println!("  Cells: {}", self.size * self.size);
        println!("  Colors: {}", self.colors);
        println!("  Search Space: {}", (self.colors as f64).powf((self.size * self.size) as f64));
    }

    fn from_csv(content: &str) -> Option<Self> {
        let rows: Vec<&str> = content.trim().split('\n').collect();
        let size = rows.len();
        let mut data = Vec::with_capacity(size * size);
        let mut colors = 0;

        for row in rows {
            for val in row.split(',') {
                let color = val.trim().parse::<u8>().ok()?;
                colors = colors.max(color as usize + 1);
                data.push(color);
            }
        }

        Some(Grid { size, colors, data })
    }

    fn to_csv(&self) -> String {
        let mut result = String::new();
        for y in 0..self.size {
            for x in 0..self.size {
                if x > 0 {
                    result.push(',');
                }
                result.push_str(&self.data[y * self.size + x].to_string());
            }
            result.push('\n');
        }
        result
    }

    fn flood_fill(&mut self, target_color: u8) {
        let source_color = self.data[0];
        if source_color == target_color {
            return;
        }

        let mut queue = VecDeque::new();
        queue.push_back(0); // Start from the top-left corner, index 0

        while let Some(index) = queue.pop_front() {
            let (x, y) = (index % self.size, index / self.size);

            if self.data[index] != source_color {
                continue;
            }

            self.data[index] = target_color;

            // Add adjacent cells to the queue by calculating their indices
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let nx = (x as isize + dx) as usize;
                let ny = (y as isize + dy) as usize;

                if nx < self.size && ny < self.size && self.data[ny * self.size + nx] == source_color {
                    queue.push_back(ny * self.size + nx);
                }
            }
        }
    }

    fn is_complete(&self) -> bool {
        let target = self.data[0];
        self.data.iter().all(|&color| color == target)
    }
}

fn solve(grid: &mut Grid) -> Vec<u8> {
    grid.print_stats();

    if grid.is_complete() {
        println!("Grid already complete");
        return Vec::new();
    }

    struct SearchState {
        moves: Vec<u8>,
        grid_state: Vec<u8>, // Grid state as a flat vector
    }

    let mut stack: Vec<SearchState> = Vec::new();
    let mut visited: HashSet<Vec<u8>> = HashSet::new();
    let mut best_solution = Vec::new();
    let mut min_length = grid.size * grid.size;

    // Initialize the stack with the first moves
    for color in 0..grid.colors {
        let color = color as u8;
        if color != grid.data[0] {
            let mut temp_grid = grid.clone();
            temp_grid.flood_fill(color);

            if visited.insert(temp_grid.data.clone()) {
                stack.push(SearchState {
                    moves: vec![color],
                    grid_state: temp_grid.data,
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
        let mut temp_grid = Grid {
            size: grid.size,
            colors: grid.colors,
            data: state.grid_state.clone(),
        };

        // Check if the grid is complete
        if temp_grid.is_complete() {
            if state.moves.len() < min_length {
                println!("New best solution: {:?} (length {})", state.moves, state.moves.len());
                best_solution = state.moves.clone();
                min_length = state.moves.len();
            }
            continue;
        }

        // Add next possible moves
        for color in 0..grid.colors {
            let color = color as u8;

            // Skip moves that repeat the current color or backtrack
            if color == temp_grid.data[0] || (state.moves.last() == Some(&color)) {
                continue;
            }

            let mut next_grid = temp_grid.clone();
            next_grid.flood_fill(color);

            // Only consider this move if it results in a new grid state
            if visited.insert(next_grid.data.clone()) {
                let mut next_moves = state.moves.clone();
                next_moves.push(color);

                stack.push(SearchState {
                    moves: next_moves,
                    grid_state: next_grid.data,
                });
            }
        }
    }

    println!("Final solution: {:?}", best_solution);
    best_solution
}



fn save_solution(moves: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut output = String::new();
    for &m in moves {
        output.push_str(&m.to_string());
        output.push('\n');
    }
    std::fs::write("solution.csv", output)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input.csv")?;

    if let Some(mut grid) = Grid::from_csv(&input) {
        let solution = solve(&mut grid);
        println!("Found solution in {} moves", solution.len());

        if grid.apply_solution(&solution) {
            println!("Solution verified successfully!");
        } else {
            println!("Solution failed to complete the grid.");
        }

        save_solution(&solution)?;
    } else {
        println!("Failed to parse input grid");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplest_input() {
        let input = "0,1\n1,1";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid);
        assert_eq!(solution, vec![1]);
    }

    #[test]
    fn test_medium_input() {
        let input = "2,1,3,0,4\n1,2,2,3,1\n0,3,1,2,4\n4,1,0,3,2\n3,2,4,1,0";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid);
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
        let solution = solve(&mut grid);
        assert_eq!(solution, vec![2, 1, 2, 0, 1]);

        let mut test_grid = grid.clone();
        assert!(test_grid.apply_solution(&solution));
    }

    #[test]
    fn test_flood_fill() {
        let mut grid = Grid::new(3, 3);
        grid.data = vec![0, 1, 0, 1, 0, 0, 1, 0, 0];

        grid.flood_fill(2);
        assert_eq!(grid.data, vec![2, 1, 2, 1, 2, 2, 1, 2, 2]);
    }

    #[test]
    fn test_large_grid() {
        let mut input = String::new();
        for _ in 0..6 {
            for j in 0..6 {
                if j > 0 { input.push(','); }
                input.push_str(&(j % 3).to_string());
            }
            input.push('\n');
        }
        let mut grid = Grid::from_csv(&input).unwrap();
        let solution = solve(&mut grid);
        assert!(solution.len() <= 10);

        let mut test_grid = grid.clone();
        assert!(test_grid.apply_solution(&solution));
    }

    #[test]
    fn test_worst_case() {
        let mut input = String::new();
        for i in 0..8 {
            for j in 0..8 {
                if j > 0 { input.push(','); }
                input.push_str(&((i + j) % 4).to_string());
            }
            input.push('\n');
        }
        let mut grid = Grid::from_csv(&input).unwrap();
        let solution = solve(&mut grid);
        assert!(solution.len() <= 60);

        let mut test_grid = grid.clone();
        assert!(test_grid.apply_solution(&solution));
    }

    #[test]
    fn test_single_color() {
        let mut grid = Grid::new(5, 1);
        let solution = solve(&mut grid);
        assert_eq!(solution.len(), 0);

        let mut test_grid = grid.clone();
        assert!(test_grid.apply_solution(&solution));
    }
}