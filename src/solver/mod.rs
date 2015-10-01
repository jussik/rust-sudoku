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

pub struct Solver;

impl Solver {
    /// Solve the puzzle in place, returns `true` if successful
    pub fn solve_mut(&self, cells: &mut [Cell; 81]) -> bool {
        loop {
            let mut changed = false;
            changed |= simple::rows(cells)
                || simple::columns(cells)
                || simple::boxes(cells)
                || hidden::rows(cells)
                || hidden::columns(cells)
                || hidden::boxes(cells)
                || locked::rows(cells)
                || locked::columns(cells)
                || locked::box_rows(cells)
                || locked::box_cols(cells)
                || naked::rows(cells)
                || naked::columns(cells)
                || naked::boxes(cells);

            let mut done = true;
            for major in 0..9 {
                let mut vals: [u16; 3] = [0; 3];
                for minor in 0..9 {
                    for f in 0..3 {
                        let i = CHECK_FUNCS[f](major, minor);
                        let cell = cells[i];
                        if cell.value == -1 {
                            if cell.possible == 0 {
                                return false;
                            }
                            done = false;
                        } else {
                            let val_bit = 1 << cell.value;
                            if (vals[f] & val_bit) != 0 {
                                return false;
                            }
                            vals[f] |= val_bit;
                        }
                    }
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
                    let count = cell.possible.count_ones() as u16;
                    if count == poss {
                        for v in 0..9 {
                            if cell.possible & VALS[v] != 0 {
                                let mut new_cells: [Cell; 81] = [Cell {
                                    value: 0,
                                    possible: 0
                                }; 81];
                                {
                                    for i in 0..81 {
                                        // TODO: faster copy
                                        new_cells[i] = cells[i];
                                    }
                                    let mut cell = &mut new_cells[c];
                                    cell.value = v as i8;
                                    cell.possible = 1 << v;
                                }
                                if self.solve_mut(&mut new_cells) {
                                    // TODO: faster copy
                                    for i in 0..81 {
                                        let src = new_cells[i];
                                        let mut dst = &mut cells[i];
                                        dst.value = src.value;
                                        dst.possible = src.possible;
                                    }
                                    return true;
                                }
                            }
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}

static CHECK_FUNCS: [LocFn; 3] = [row_loc, col_loc, box_loc];

static VALS: [u16; 9] = [
    0x001,
    0x002,
    0x004,
    0x008,
    0x010,
    0x020,
    0x040,
    0x080,
    0x100,
];
