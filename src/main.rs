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

        let mut stack = vec![(0, 0)];
        while let Some((x, y)) = stack.pop() {
            if self.data[y * self.size + x] != source_color {
                continue;
            }

            self.data[y * self.size + x] = target_color;

            // Add adjacent cells
            if x > 0 { stack.push((x-1, y)); }
            if y > 0 { stack.push((x, y-1)); }
            if x + 1 < self.size { stack.push((x+1, y)); }
            if y + 1 < self.size { stack.push((x, y+1)); }
        }
    }

    fn is_complete(&self) -> bool {
        self.data.iter().all(|&x| x == self.data[0])
    }
}

fn solve(grid: &mut Grid) -> Vec<u8> {
    let mut moves: Vec<u8> = Vec::new();
    let mut current_grid = grid.clone();
    let mut min_moves = vec![0u8; grid.size * grid.size];
    let mut min_length = grid.size * grid.size;

    fn try_moves(
        current: &mut Grid,
        moves_so_far: &mut Vec<u8>,
        min_moves: &mut Vec<u8>,
        min_length: &mut usize
    ) {
        if current.is_complete() {
            if moves_so_far.len() < *min_length {
                min_moves.clear();
                min_moves.extend(moves_so_far.iter());
                *min_length = moves_so_far.len();
            }
            return;
        }

        if moves_so_far.len() >= *min_length {
            return;
        }

        for color in 0..current.colors {
            let color = color as u8;
            if color == current.data[0] {
                continue;
            }

            let mut next_grid = current.clone();
            next_grid.flood_fill(color);

            moves_so_far.push(color);
            try_moves(&mut next_grid, moves_so_far, min_moves, min_length);
            moves_so_far.pop();
        }
    }

    let mut current_moves = Vec::new();
    try_moves(&mut current_grid, &mut current_moves, &mut min_moves, &mut min_length);

    min_moves[..min_length].to_vec()
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
        println!("Initial grid:\n{}", grid.to_csv());
        let solution = solve(&mut grid);
        println!("Found solution in {} moves", solution.len());
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
    fn test_sample_input() {
        let input = "1,2,0,0\n0,1,1,0\n2,2,0,1\n0,0,0,1";
        let mut grid = Grid::from_csv(input).unwrap();
        let solution = solve(&mut grid);
        assert!(solution.len() <= 5);
    }

    #[test]
    fn test_flood_fill() {
        let mut grid = Grid::new(2, 3);
        grid.data = vec![0, 1, 1, 1];
        grid.flood_fill(2);
        assert_eq!(grid.data, vec![2, 1, 1, 1]);
    }
}