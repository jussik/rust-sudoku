use super::*;

use std::thread;

pub fn rows(args: &SolverArgs) {
    run(args, box_loc, row_loc);
}
pub fn columns(args: &SolverArgs) {
    run(args, inv_box_loc, col_loc);
}
pub fn box_rows(args: &SolverArgs) {
    run(args, row_loc, box_loc);
}
pub fn box_cols(args: &SolverArgs) {
    run(args, col_loc, inv_box_loc);
}

fn run(args: &SolverArgs, outer_func: LocFn, inner_func: LocFn) {
    let grid = &args.cells;
    let tx = &args.tx;
    let is_done = &args.is_done;
    let mut changed = false;
    loop {
        for outer_maj in 0..3 {
            for outer_min in 0..3 {
                let outer_root = outer_func(outer_maj * 3 + outer_min, 0);
                // find any values that are only in one row/column of this box
                let mut poss: [u16; 3] = [0; 3]; // all possibles in each triple
                let mut values: u16 = 0;
                for major in 0..3 {
                    // loop through inner segments, each with 3 cells
                    for minor in 0..3 {
                        let i = outer_root + inner_func(major, minor);
                        let cell = *grid[i].read().unwrap();
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
                            let outer_offset = outer_func(
                                    outer_maj * 3 + ((outer_min + iter) % 3),
                                    0);
                            for minor in 0..3 {
                                // 3 cells in each inner segment
                                let i = outer_offset + inner_func(major, minor);
                                let mut cell = grid[i].write().unwrap();
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
        tx.send(changed).unwrap();
        thread::yield_now();
        if *is_done.read().unwrap() {
            return;
        }
    }
}
