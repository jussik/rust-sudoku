pub mod simple;
pub mod hidden;
pub mod locked;
pub mod naked;

use ::grid::Cell;

pub type LocFn = fn(usize, usize) -> usize;

pub fn row_loc(major: usize, minor: usize) -> usize {
    major * 9 + minor
}
pub fn col_loc(major: usize, minor: usize) -> usize {
    minor * 9 + major
}
pub fn box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 3
        + (major / 3) * 27
        + (minor % 3)
        + (minor / 3) * 9
}
pub fn inv_box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 27
        + (major / 3) * 3
        + (minor % 3) * 9
        + (minor / 3)
}

macro_rules! check_pass {
    (
        $cells:ident,
        [ $($func:ident:$id:expr),+ ]
        ( $major:ident, $minor:ident )
        -> $vals:ident, $done:ident
    ) => {{
        $(
            let i = $func($major, $minor);
            let cell = $cells[i];
            if cell.value == -1 {
                if cell.possible == 0 {
                    // no possible values
                    return false;
                }
                $done = false;
            } else {
                let val_bit = 1 << cell.value;
                if ($vals[$id] & val_bit) != 0 {
                    // duplicate value
                    return false;
                }
                $vals[$id] |= val_bit;
            }
        )+
    }}
}

pub struct Solver;

impl Solver {
    /// Solve the puzzle in place, returns `true` if successful
    pub fn solve_mut(&self, cells: &mut [Cell; 81]) -> bool {
        loop {
            let mut changed = false;
            changed |= simple::rows(cells);
            changed |= simple::columns(cells);
            changed |= simple::boxes(cells);
            changed |= hidden::rows(cells);
            changed |= hidden::columns(cells);
            changed |= hidden::boxes(cells);
            changed |= locked::rows(cells);
            changed |= locked::columns(cells);
            changed |= locked::box_rows(cells);
            changed |= locked::box_cols(cells);
            changed |= naked::rows(cells);
            changed |= naked::columns(cells);
            changed |= naked::boxes(cells);

            // check if grid is solved or invalid
            let mut done = true;
            for major in 0..9 {
                let mut vals: [u16; 3] = [0; 3];
                for minor in 0..9 {
                    check_pass!(cells,
                        [row_loc:0, col_loc:1, box_loc:2](major, minor)
                        -> vals, done);
                }
            }
            if done {
                return true;
            } else if !changed {
                return self.guess(cells);
            }
        }
    }

    fn guess(&self, cells: &mut [Cell; 81]) -> bool  {
        for poss in 2..10 { // find cells with least possibles first
            for c in 0..81 {
                let cell = cells[c];
                if cell.value == -1 {
                    let mut p = cell.possible;
                    let mut v = 0;
                    if p.count_ones() == poss {
                        while p != 0 {
                            if p & 1 != 0 {
                                let mut new_cells = *cells;
                                {
                                    let mut cell = &mut new_cells[c];
                                    cell.value = v as i8;
                                    cell.possible = 1 << v;
                                }
                                if self.solve_mut(&mut new_cells) {
                                    *cells = new_cells;
                                    return true;
                                }
                            }
                            p = p >> 1;
                            v += 1;
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}
