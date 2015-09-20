use std::char;

use super::row_iterator::RowIterator;
use super::column_iterator::ColumnIterator;

/// A single cell in a `Grid`
#[derive(Copy, Clone)]
pub struct Cell {
    /// Value of the cell (0-8), -1 if unknown
    value: i8,
    /// Bit field of possible values
    possible: u16
}
impl Cell {
    /// Returns '0'-'9' if the cell has a value, ' ' otherwise
    fn to_char(&self) -> char {
        if self.value == -1 {
            ' '
        } else {
            char::from_digit(self.value as u32 + 1, 10).unwrap()
        }
    }
    fn remove_possible(&mut self, value: i8) {
        self.possible = self.possible & !(1 << value);
    }
}

/// A 9x9 sudoku grid
#[derive(Copy)]
pub struct Grid {
    /// Values in the grid in row-major order
    values: [Cell; 81],
    /// Whether or not this is a valid puzzle
    valid: bool
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
        g.update_possible();
        g
    }

    /// Is the puzzle grid valid?
    pub fn is_valid(&self) -> bool {
        self.valid
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
    pub fn solve_mut(&mut self) -> bool {
        self.filter_unique();
        self.valid
    }

    fn update_possible(&mut self) {
        for (i, j) in RowIterator::new()
                .chain(ColumnIterator::new()) {
            if let Some((c, v)) = self.get_remove_target(i ,j) {
                self.values[c].remove_possible(v);
            }
        }
    }
    fn get_remove_target(&mut self, i: usize, j: usize) -> Option<(usize, i8)> {
        let cell: &Cell = &self.values[i];
        let adj: &Cell = &self.values[j];
        if cell.value != -1 {
            if cell.value == adj.value {
                self.valid = false;
            } else if adj.value == -1 {
                return Some((j, cell.value))
            }
        } else if adj.value != -1 {
            return Some((i, adj.value))
        }
        None
    }

    fn filter_unique(&mut self) {
        for cell in self.values.as_mut().into_iter() {
            if cell.value == -1 {
                let ones = cell.possible.count_ones();
                if ones == 1 {
                    cell.value = match cell.possible {
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
                } else if ones == 0 {
                    self.valid = false;
                }
            }
        }
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

