mod cell;
pub use self::cell::Cell;

use ::solver::Solver;

/// A 9x9 sudoku grid
#[derive(Copy)]
pub struct Grid {
    /// Values in the grid in row-major order
    pub values: [Cell; 81],
    /// Whether or not this is a valid puzzle
    valid: bool,
    /// Whether or not this puzzle is solved
    pub solved: bool
}
impl Clone for Grid {
    fn clone(&self) -> Self {
        //println!("Grid copy");
        *self
    }
}

impl Grid {
    /// Parse a string containing a list of numbers into a `Grid`
    ///
    /// Values are read in row-major order
    /// All non-digit characters are ignored
    /// 0 implies the value is unknown
    /// If less than 81 values are found, the remainder are unknown
    /// Values after the 81st are ignored
    pub fn parse(input: &str) -> Grid {
        let mut g = Grid::new();
        for (i, d) in input.chars()
            .filter_map(move |c| c.to_digit(10))
            .take(81)
            .enumerate() {
                if d != 0 {
                    let v = d as i8 - 1;
                    g.values[i].value = v;
                    g.values[i].possible = 1 << v;
                }
            }
        g
    }

    pub fn load(values: &[i8; 81]) -> Grid {
        let mut g = Grid::new();
        for i in 0..81 {
            let v = values[i];
            if v != -1 {
                g.values[i].value = v;
                g.values[i].possible = 1 << v;
            }
        }
        g
    }

    fn new() -> Grid {
        Grid {
            values: [Cell {
                value: -1,
                possible: (1 << 9) - 1
            }; 81],
            valid: true,
            solved: false
        }
    }

    /// Attempts to solve the puzzle
    ///
    /// Returns a filled out `Grid` when the solver has finished
    /// Returns `None` if the puzzle is invalid
    pub fn solve(&self) -> Option<Grid> {
        if self.valid {
            let mut g2 = self.clone();
            let solver = Solver { allow_guessing: true };
            if solver.solve_mut(&mut g2.values) {
                return Some(g2);
            }
        }
        None
    }
}
