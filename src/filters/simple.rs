use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::vec::Vec;
use std::thread;

use rand;
use rand::{XorShiftRng,Rng};

use ::grid::Cell;

type LocFn = fn(usize, usize) -> usize;

/// Remove possible values based on cells in the same row
pub fn rows(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<()>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, row_loc, tx, is_done);
}
fn row_loc(major: usize, minor: usize) -> usize {
    major * 9 + minor
}

/// Remove possible values based on cells in the same column
pub fn columns(grid: Vec<Arc<RwLock<Cell>>>,
               tx: Sender<()>,
               is_done: Arc<RwLock<bool>>) {
    run(grid, col_loc, tx, is_done);
}
pub fn col_loc(major: usize, minor: usize) -> usize {
    minor * 9 + major
}

/// Remove possible values based on cells in the same 3x3 box
pub fn boxes(grid: Vec<Arc<RwLock<Cell>>>,
             tx: Sender<()>,
             is_done: Arc<RwLock<bool>>) {
    run(grid, box_loc, tx, is_done);
}
fn box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 3
        + (major / 3) * 27
        + (minor % 3)
        + (minor / 3) * 9
}

/// Remove possibilities based on adjacent values
fn run(grid: Vec<Arc<RwLock<Cell>>>,
       func: LocFn,
       tx: Sender<()>,
       is_done: Arc<RwLock<bool>>) {
    let mut rng: XorShiftRng = rand::random();

    let mut ix_major: [usize; 9] = [0; 9];
    for i in 0..9 { ix_major[i] = i; }
    rng.shuffle(&mut ix_major);

    let mut ix_minor: [usize; 8] = [0; 8];
    for i in 0..8 { ix_minor[i] = i; }
    rng.shuffle(&mut ix_minor);

    loop {
        for x in 0..9 {
            let major = ix_major[x]; // iterating arrays emits refs, need value
            for y in 0..8 {
                let minor = ix_minor[y];
                let i = func(major, minor);
                let ival = grid[i].read().unwrap().value;
                for minor_adj in minor + 1..9 {
                    let j = func(major, minor_adj);
                    let jval = grid[j].read().unwrap().value;
                    if ival != -1 {
                        if jval == -1 {
                            let mut cell = grid[j].write().unwrap();
                            cell.remove_possible(ival);
                            //send(&tx, Op::RemovePossible(j, ival));
                        }
                    } else if jval != -1 {
                        let mut cell = grid[i].write().unwrap();
                        cell.remove_possible(jval);
                        //send(&tx, Op::RemovePossible(i, jval));
                    }
                }
            }
        }
        tx.send(()).unwrap();
        thread::yield_now();
        if *is_done.read().unwrap() {
            return;
        }
    }
}
