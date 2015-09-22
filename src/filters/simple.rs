use std::sync::{Arc, RwLock};
use std::vec::Vec;
use std::thread;

use ::grid::Cell;

type LocFn = fn(usize, usize) -> usize;

/// Remove possible values based on cells in the same row
pub fn rows(grid: Vec<Arc<RwLock<Cell>>>) {
    run(grid, row_loc);
}
fn row_loc(major: usize, minor: usize) -> usize {
    major * 9 + minor
}

/// Remove possible values based on cells in the same column
pub fn columns(grid: Vec<Arc<RwLock<Cell>>>) {
    run(grid, col_loc);
}
pub fn col_loc(major: usize, minor: usize) -> usize {
    minor * 9 + major
}

/// Remove possible values based on cells in the same 3x3 box
pub fn boxes(grid: Vec<Arc<RwLock<Cell>>>) {
    run(grid, box_loc);
}
fn box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 3
        + (major / 3) * 27
        + (minor % 3)
        + (minor / 3) * 9
}

/// Remove possibilities based on adjacent values
fn run(grid: Vec<Arc<RwLock<Cell>>>, func: LocFn) {
    loop {
        let mut changed = false;
        for major in 0..9 {
            for minor in 0..8 {
                let i = func(major, minor);
                let ival = grid[i].read().unwrap().value;
                for minor_adj in minor + 1..9 {
                    let j = func(major, minor_adj);
                    let jval = grid[j].read().unwrap().value;
                    if ival != -1 {
                        if jval == -1 {
                            let mut cell = grid[j].write().unwrap();
                            changed |= cell.remove_possible(ival);
                            //send(&tx, Op::RemovePossible(j, ival));
                        }
                    } else if jval != -1 {
                        let mut cell = grid[i].write().unwrap();
                        changed |= cell.remove_possible(jval);
                        //send(&tx, Op::RemovePossible(i, jval));
                    }
                }
            }
        }
        if !changed {
            return;
        }
        thread::yield_now();
    }
}
