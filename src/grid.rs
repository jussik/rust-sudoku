use std::sync::mpsc::{channel, Receiver};

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
    fn set(&mut self, value: i8) -> bool {
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
    fn remove_possible(&mut self, value: i8) -> bool {
        let bit = 1 << value;
        if self.possible & bit == 0 || self.value != -1 {
            false
        } else {
            self.possible &= !bit;
            if self.possible.count_ones() == 1 {
                self.value =  match self.possible {
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
    valid: bool
}
impl Clone for Grid {
    fn clone(&self) -> Self {
        //println!("Grid copy");
        *self
    }
}

pub enum Op {
    SetValue(usize, i8),
    RemovePossible(usize, i8),
    Invalidate(&'static str)
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
            valid: true
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
            g2.solve_mut(100);
            if self.valid {
                return Some(g2);
            }
        }
        None
    }

    /// Runs a single iteration of solving the puzzle
    ///
    /// Returns a tuple of:
    /// 1. the iterated `Grid` instance
    /// 2. a bool which is `true` when the solver has finished
    /// Returns `None` if the puzzle is invalid
    pub fn step(&self) -> Option<(Grid, bool)> {
        if self.valid {
            let mut g2 = self.clone();
            let done = g2.solve_mut(1);
            if self.valid {
                return Some((g2, done));
            }
        }
        None
    }

    /// Solve the sudoku puzzle in the current `Grid`
    ///
    /// Returns `true` if the solver has finished
    fn solve_mut(&mut self, iterations: u32) -> bool {
        let (tx, rx) = channel::<Option<Op>>();
        let mut count = iterations;
        while count > 0 {
            let mut changed = false;
            
            simple::remove_possibles(self, tx.clone());
            //simple::find_hidden(self, tx.clone());
            changed |= self.apply_ops(&rx);

            if !changed {
                return true
            }
            count -= 1;
        }
        self.is_solved()
    }

    /// Apply all operations received up to this point
    fn apply_ops(&mut self, rx: &Receiver<Option<Op>>) -> bool {
        let mut changed = false;
        while let Some(op) = rx.recv().unwrap() {
            match op {
                Op::SetValue(c, v) => {
                    //println!("set value {},{}", c, v);
                    self.values[c].value = v;
                    changed |= self.values[c].set(v);
                },
                Op::RemovePossible(c, v) => {
                    //println!("remove possible {},{}", c, v);
                    changed |= self.values[c].remove_possible(v);
                },
                Op::Invalidate(s) => {
                    println!("Invalid: {}", s);
                    self.valid = false;
                }
            }
        }
        changed
    }

    /// Checks if the puzzle has been solved
    pub fn is_solved(&self) -> bool {
        self.values.iter().all(|c| c.value != -1)
    }
}
