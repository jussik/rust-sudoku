use super::*;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::vec::Vec;
use std::thread;

use rand;
use rand::{XorShiftRng,Rng};

use ::grid::Cell;

/// Remove possible values based on cells in the same row
pub fn rows(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<()>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, row_loc, tx, is_done);
}

/// Remove possible values based on cells in the same column
pub fn columns(grid: Vec<Arc<RwLock<Cell>>>,
               tx: Sender<()>,
               is_done: Arc<RwLock<bool>>) {
    run(grid, col_loc, tx, is_done);
}

/// Remove possible values based on cells in the same 3x3 box
pub fn boxes(grid: Vec<Arc<RwLock<Cell>>>,
             tx: Sender<()>,
             is_done: Arc<RwLock<bool>>) {
    run(grid, box_loc, tx, is_done);
}

/// Remove possibilities based on adjacent values
fn run(grid: Vec<Arc<RwLock<Cell>>>,
       func: LocFn,
       tx: Sender<()>,
       is_done: Arc<RwLock<bool>>) {
    // randomise walk order to minimise successive waits for other threads
    let mut rng: XorShiftRng = rand::random();

    let mut ix_major: [usize; 9] = [0; 9];
    for i in 0..9 { ix_major[i] = i; }
    //rng.shuffle(&mut ix_major);

    let mut ix_minor: [usize; 8] = [0; 8];
    for i in 0..8 { ix_minor[i] = i; }
    //rng.shuffle(&mut ix_minor);
    let id = rng.next_u32() as u16;

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
