use std::char;
use std::sync::mpsc::{channel, Sender, Receiver};

use filters::singles;

/// A single cell in a `Grid`
#[derive(Copy, Clone)]
pub struct Cell {
    /// Value of the cell (0-8), -1 if unknown
    pub value: i8,
    /// Bit field of possible values
    pub possible: u16
}
impl Cell {
    /// Returns '0'-'9' if the cell has a value, ' ' otherwise
    pub fn to_char(&self) -> char {
        if self.value == -1 {
            ' '
        } else {
            char::from_digit(self.value as u32 + 1, 10).unwrap()
        }
    }
    /// Sets the current value, returns `true` if changed
    fn set(&mut self, value: i8) -> bool {
        if self.value != -1 {
            false
        } else {
            self.value = value;
            true
        }
    }
    /// Removes a value from its possibilities, returns `true` if changed
    fn remove_possible(&mut self, value: i8) -> bool {
        let bit = 1 << value;
        if self.possible & bit == 0 {
            false
        } else {
            self.possible &= !bit;
            true
        }
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

    /// Solve the sudoku puzzle and return the solved `Grid`
    ///
    /// Returns a filled out `Some(Grid)` if successful, `None` otherwise
    pub fn solve(&self) -> Option<Grid> {
        if self.valid {
            let mut g2 = self.clone();
            if g2.solve_mut() {
                return Some(g2);
            }
        }
        None
    }

    /// Solve the sudoku puzzle in the current `Grid`
    ///
    /// Returns true if succesful, false otherwise
    fn solve_mut(&mut self) -> bool {
        let (tx, rx) = channel::<Option<Op>>();

        loop {
            let mut changed = false;
            singles::remove_possibles(self, tx.clone());
            changed |= self.apply_ops(&tx, &rx);
            singles::set_uniques(self, tx.clone());
            changed |= self.apply_ops(&tx, &rx);
            if !changed {
                break;
            }
        }

        self.valid
    }

    /// Apply all operations received up to this point
    fn apply_ops(&mut self, tx: &Sender<Option<Op>>, rx: &Receiver<Option<Op>>) -> bool {
        tx.send(None).unwrap();
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

    /// Generate 21x11 ascii table representing the `Grid`
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(241);
        for (i, v) in self.values.iter().enumerate() {
            if (i % 9) == 0 {
                if i > 0 {
                    if (i / 9) % 3 == 0 {
                        buf.push_str("\n------+-------+------\n");
                    } else {
                        buf.push('\n');
                    }
                }
            } else if i % 3 == 0 {
                buf.push_str(" | ");
            } else {
                buf.push(' ');
            }

            buf.push(v.to_char());
        }
        buf
    }
}

