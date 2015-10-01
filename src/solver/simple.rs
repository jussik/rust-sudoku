use super::*;
use ::grid::Cell;

/// Remove possibilities based on adjacent values
macro_rules! run {
    ($cells:ident, $func:ident) => {{
        let mut changed = false;
        for major in 0..9 {
            for minor in 0..8 {
                let i = $func(major, minor);
                let ci = $cells[i];
                for minor_adj in minor + 1..9 {
                    let j = $func(major, minor_adj);
                    let cj = $cells[j];
                    if ci.value != -1 {
                        if cj.value == -1 && cj.is_possible(ci.value) {
                            let mut cell = &mut $cells[j];
                            changed |= cell.remove_possible(ci.value);
                        }
                    } else if cj.value != -1 && ci.is_possible(cj.value) {
                        let mut cell = &mut $cells[i];
                        changed |= cell.remove_possible(cj.value);
                    }
                }
            }
        }
        changed
    }}
}

/// Remove possible values based on cells in the same row
#[inline]
pub fn rows(cells: &mut [Cell; 81]) -> bool {
    run!(cells, row_loc)
}

/// Remove possible values based on cells in the same column
#[inline]
pub fn columns(cells: &mut [Cell; 81]) -> bool {
    run!(cells, col_loc)
}

/// Remove possible values based on cells in the same 3x3 box
#[inline]
pub fn boxes(cells: &mut [Cell; 81]) -> bool {
    run!(cells, box_loc)
}
