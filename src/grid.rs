use std::char;
use std::mem::size_of;

type Group = [u8; 9];

pub struct Grid {
    rows: [Group; 9],
    columns: [Group; 9],
    boxes: [Group; 9]
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
            .collect::<Vec<u8>>();
        if vals.len() != 81 {
            None
        } else {
            let mut g = Grid {
                rows: [[0; 9]; 9],
                columns: [[0; 9]; 9],
                boxes: [[0; 9]; 9],
            };
            for i in (0..9) {
                for j in (0..9) {
                    //g.rows[i][j] = vals[];
                }
            }
            Some(g)
        }
    }
    pub fn to_string(&self) -> String {
        let mut buf = String::with_capacity(241);
        /*for (i, v) in self.rows.iter().enumerate() {
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
        }*/
        println!("{}", size_of::<Grid>() + (81*9));
        buf
    }
}

