use std::char;
use sudoku::grid::{Grid, Cell};

/// Generate 21x11 ascii table representing the `Grid`
pub fn create(grid: &Grid) -> String {
    let mut buf = String::with_capacity(241);
    for (i, v) in grid.values.iter().enumerate() {
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

        buf.push(number(v.value));
    }
    buf
}
fn number(value: i8) -> char {
    if value == -1 {
        ' '
    } else {
        char::from_digit(value as u32 + 1, 10).unwrap()
    }
}

/// Generate large ascii grid which displays cells' possible values
pub fn create_large(grid: &Grid) -> String {
    let mut printer = GridPrinter {
        grid: grid,
        buf: String::with_capacity(2811)
    };
    for i in 0..9 {
        printer.divider(i);
        printer.row(i);
    }
    printer.divider(0);
    printer.buf
}

pub fn create_oneline(grid: &Grid) -> String {
    let mut buf = String::with_capacity(82);
    for v in grid.values.iter() {
        buf.push(number(v.value));
    }
    buf
}

struct GridPrinter<'a> {
    grid: &'a Grid,
    buf: String
}
impl<'a> GridPrinter<'a> {
    fn row(&mut self, row: usize) {
        self.buf.push('\n');
        let rx = row * 9;
        for r in 0..3 {
            for b in 0..3 {
                let bx = b * 3 + rx;
                self.buf.push('|');
                for c in 0..3 {
                    let cell = self.grid.values[bx + c];
                    self.cell_row(&cell, r as u32);
                }
            }
            self.buf.push('\n');
        }
    }
    fn cell_row(&mut self, cell: &Cell, r: u32) {
        if cell.value == -1 {
            let x = r * 3;
            for i in 0..3 {
                self.buf.push(' ');
                let mut num = i + x as i8;
                num = if cell.is_possible(num) { num } else { -1 };
                self.buf.push(number(num));
            }
        } else {
            self.buf.push_str(&FIG_CACHE[(r + cell.value as u32 * 3) as usize]);
        }
        self.buf.push_str(" |");
    }
    fn divider(&mut self, row: usize) {
        if row % 3 == 0 {
            self.buf.push_str("\
                |=======|=======|=======|\
                |=======|=======|=======|\
                |=======|=======|=======|");
        } else {
            self.buf.push_str("\
                |-------|-------|-------|\
                |-------|-------|-------|\
                |-------|-------|-------|");
        }
    }
}
static FIG_CACHE: [&'static str; 27] = [
    "   ,  ",
    "  /|  ",
    "   |  ",
    "  __  ",
    "   _) ",
    "  /__ ",
    "  __  ",
    "   _) ",
    "  __) ",
    "      ",
    " |__| ",
    "    | ",
    "   __ ",
    "  |_  ",
    "  __) ",
    "  __  ",
    " /__  ",
    " \\__) ",
    "  ___ ",
    "    / ",
    "   /  ",
    "  __  ",
    " (__) ",
    " (__) ",
    "  __  ",
    " (__\\ ",
    "  __/ "
];
