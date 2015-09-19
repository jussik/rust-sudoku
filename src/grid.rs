use std::char;

/// A single cell in a `Grid`
#[derive(Copy, Clone)]
pub struct Cell {
    /// Value of the cell, 0 if unknown
    value: u8,
    /// Bit field of possible values
    possible: u16
}

/// A 9x9 sudoku grid
pub struct Grid {
    /// Values in the grid in row-major order
    values: [Cell; 81]
}

impl Grid {
    fn row_of(i: usize) -> usize { i / 9 }
    fn col_of(i: usize) -> usize { i % 9 }
    fn box_of(i: usize) -> usize { (i % 9 ) / 3 + 3 * (i / 27) }

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
                value: 0,
                possible: (1 << 9) - 1
            }; 81]
        };
        for (i, v) in input.chars()
            .filter_map(move |c| c.to_digit(10))
            .take(81)
            .enumerate() {
                if v != 0 {
                    g.values[i].value = v as u8;
                    g.values[i].possible = 1 << v;
                }
            }
        g
    }

    /// Solve the sudoku puzzle
    ///
    /// Returns a filled out `Some(Grid)` if successful, `None` otherwise
    pub fn solve(&self) -> Option<Grid> {
        None
    }

    /// Generate 21x11 ascii table representing the `Grid`
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(241);
        for (i, v) in self.values.iter().enumerate() {
            if Grid::col_of(i) == 0 {
                if i > 0 {
                    if Grid::row_of(i) % 3 == 0 {
                        buf.push_str("\n------+-------+------\n");
                    } else {
                        buf.push('\n');
                    }
                }
            } else if Grid::col_of(i) % 3 == 0 {
                buf.push_str(" | ");
            } else {
                buf.push(' ');
            }

            if v.value == 0  {
                buf.push(' ');
            } else {
                buf.push(char::from_digit(v.value as u32, 10).unwrap());
            }
        }
        buf
    }
}

