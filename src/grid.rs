use std::char;

pub struct Grid {
    /// values in row-major order
    row_order: Vec<u8>,
    /// values in column-major order
    col_order: Vec<u8>,
    /// values ordered in groups of 3x3 row major boxes
    box_order: Vec<u8>
}

impl Grid {
    fn row_of(i: usize) -> usize { i / 9 }
    fn col_of(i: usize) -> usize { i % 9 }
    fn box_of(i: usize) -> usize { (i % 9 ) / 3 + 3 * (i / 27) }

    /// Parse a string containing a list of numbers into a Grid
    ///
    /// Values are read in row-major order ignoring all non-digit characters
    /// A value of 0 implies the value is unknown
    /// Returns `None` if there are not exactly 81 digits in the string
    pub fn parse(input: &str) -> Option<Grid> {
        let vals = input.chars()
            .filter_map(move |c| c.to_digit(10).map(|v| v as u8))
            .collect::<Vec<_>>();
        if vals.len() != 81 {
            None
        } else {
            let cols = (0..81)
                .map(|i| vals[(i / 9)  + (i % 9) * 9])
                .collect::<Vec<_>>();
            let boxes = (0..81)
                .map(|i| vals[i])
                .collect::<Vec<_>>();
            Some(Grid {
                row_order: vals,
                col_order: cols,
                box_order: boxes
            })
        }
    }
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(241);
        for (i, v) in self.box_order.iter().enumerate() {
            if i % 9 == 0 {
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

            if *v == 0  {
                buf.push(' ');
            } else {
                buf.push(char::from_digit(*v as u32, 10).unwrap());
            }
        }
        buf
    }
}

