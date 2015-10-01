use super::*;
use ::grid::Cell;

macro_rules! run {
    ($cells:ident, $func:ident) => {{
        let mut poss: [u16; 9] = [0; 9];
        let mut changed = false;
        for major in 0..9 {
            let mut any = 0; // bit is 1 if any $cells have that possibility
            let mut ovr = 0; // bit is 1 once a second cell has it
            for minor in 0..9 {
                let i = $func(major, minor);
                let cell = $cells[i];
                let pos = cell.possible;
                if cell.value != -1 {
                    // value already exists, take it out of contention
                    poss[minor] = 0;
                    ovr |= pos;
                } else {
                    poss[minor] = pos;
                    ovr |= any & pos;
                }
                any |= pos;
            }
            let uniqs = any ^ ovr;
            if uniqs != 0 {
                // there are bits in any that are not in ovr
                // second pass, find $cells with unique possibles
                for minor in 0..9 {
                    let p = poss[minor] & uniqs;
                    if p != 0 {
                        // cell has unique possible
                        let i = $func(major, minor);
                        let mut cell = &mut $cells[i];
                        cell.possible = p;
                        changed |= cell.check_possible();
                    }
                }
            }
        }
        changed
    }}
}

pub fn rows(cells: &mut [Cell;  81]) -> bool {
    run!(cells, row_loc)
}
pub fn columns(cells: &mut [Cell;  81]) -> bool {
    run!(cells, col_loc)
}
pub fn boxes(cells: &mut [Cell;  81]) -> bool {
    run!(cells, box_loc)
}
