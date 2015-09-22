use std::thread;
use std::vec::Vec;
use std::sync::{Arc, RwLock};

use filters::simple;

/// A single cell in a `Grid`
#[derive(Copy, Clone)]
pub struct Cell {
    /// Value of the cell (0-8), -1 if unknown
    pub value: i8,
    /// Bit field of possible values
    pub possible: u16
}
impl Cell {
    /// Sets the current value, returns `true` if changed
    pub fn set(&mut self, value: i8) -> bool {
        if self.value != -1 {
            false
        } else {
            self.value = value;
            true
        }
    }
    /// Removes a value from its possibilities
    /// Sets value if only one possibility remains
    /// Returns `true` if possibilities or value have changed
    pub fn remove_possible(&mut self, value: i8) -> bool {
        let bit = 1 << value;
        if self.possible & bit == 0 || self.value != -1 {
            false
        } else {
            self.possible &= !bit;
            if self.possible.count_ones() == 1 {
                self.value = match self.possible {
                    0x001 => 0,
                    0x002 => 1,
                    0x004 => 2,
                    0x008 => 3,
                    0x010 => 4,
                    0x020 => 5,
                    0x040 => 6,
                    0x080 => 7,
                    0x100 => 8,
                    _ => -1
                };
            }
            true
        }
    }
    pub fn is_possible(&self, value: i8) -> bool {
        self.possible & (1 << value) != 0
    }
}

/// A 9x9 sudoku grid
#[derive(Copy)]
pub struct Grid {
    /// Values in the grid in row-major order
    pub values: [Cell; 81],
    /// Whether or not this is a valid puzzle
    valid: bool,
    /// Whether or not this puzzle is solved
    solved: bool
}
impl Clone for Grid {
    fn clone(&self) -> Self {
        //println!("Grid copy");
        *self
    }
}

macro_rules! start_solver {
    ($func:expr, $data:ident, $handles:ident) => {{
        let data = $data.clone();
        $handles.push(thread::spawn(move || {
            $func(data);
        }));
    }}
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
        let mut g = Grid {
            values: [Cell {
                value: -1,
                possible: (1 << 9) - 1
            }; 81],
            valid: true,
            solved: false
        };
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

    /// Attempts to solve the puzzle
    ///
    /// Returns a filled out `Grid` when the solver has finished
    /// Returns `None` if the puzzle is invalid
    pub fn solve(&self) -> Option<Grid> {
        if self.valid {
            let mut g2 = self.clone();
            g2.solve_mut();
            if self.valid {
                return Some(g2);
            }
        }
        None
    }

    /// Solve the sudoku puzzle in the current `Grid`
    fn solve_mut(&mut self) {
        let cells = self.values.iter()
            .map(|cell| Arc::new(RwLock::new(cell.clone())))
            .collect::<Vec<_>>();
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();

        start_solver!(simple::rows, cells, handles);
        start_solver!(simple::columns, cells, handles);
        start_solver!(simple::boxes, cells, handles);

        for h in handles {
            h.join().unwrap();
        }
        for i in 0..81 {
            let cell = cells[i].read().unwrap();
            self.values[i] = *cell;
        }
        self.solved = self.values.iter().all(|c| c.value != -1)
    }
}
