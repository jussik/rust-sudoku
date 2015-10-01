use super::*;
use ::grid::Cell;

macro_rules! run {
    ($cells:ident, $outer_func:ident, $inner_func:ident) => {{
        let mut changed = false;
        for outer_maj in 0..3 {
            for outer_min in 0..3 {
                let outer_root = $outer_func(outer_maj * 3 + outer_min, 0);
                // find any values that are only in one row/column of this box
                let mut poss: [u16; 3] = [0; 3]; // all possibles in each triple
                let mut values: u16 = 0;
                for major in 0..3 {
                    // loop through inner segments, each with 3 $cells
                    for minor in 0..3 {
                        let i = outer_root + $inner_func(major, minor);
                        let cell = $cells[i];
                        if cell.value == -1 {
                            poss[major] |= cell.possible;
                        } else {
                            values |= 1 << cell.value as u16;
                        }
                    }
                }
                for major in 0..3 {
                    let uniq = poss[major] & !values
                        & !(poss[(major + 1) % 3] | poss[(major + 2) % 3]);
                    if uniq != 0 {
                        // some unique possibles in this inner segment
                        for iter in 1..3 {
                            // go through the other two inner segments
                            let outer_offset = $outer_func(
                                outer_maj * 3 + ((outer_min + iter) % 3),
                                0);
                            for minor in 0..3 {
                                // 3 $cells in each inner segment
                                let i = outer_offset + $inner_func(major, minor);
                                let mut cell = &mut $cells[i];
                                if cell.value == -1
                                    && (cell.possible & uniq) != 0 {
                                        cell.possible &= !uniq;
                                        cell.check_possible();
                                        changed = true;
                                    }
                            }
                        }
                    }
                }
            }
        }
        changed
    }}
}

#[inline]
pub fn rows(cells: &mut [Cell;  81]) -> bool {
    run!(cells, box_loc, row_loc)
}
#[inline]
pub fn columns(cells: &mut [Cell;  81]) -> bool {
    run!(cells, inv_box_loc, col_loc)
}
#[inline]
pub fn box_rows(cells: &mut [Cell;  81]) -> bool {
    run!(cells, row_loc, box_loc)
}
#[inline]
pub fn box_cols(cells: &mut [Cell;  81]) -> bool {
    run!(cells, col_loc, inv_box_loc)
}
