use super::*;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::vec::Vec;
use std::thread;

use rand;
use rand::{XorShiftRng,Rng};

use ::grid::Cell;

pub fn rows(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<()>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, row_loc, tx, is_done);
}
pub fn columns(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<()>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, col_loc, tx, is_done);
}
pub fn boxes(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<()>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, box_loc, tx, is_done);
}

fn run(grid: Vec<Arc<RwLock<Cell>>>,
       func: LocFn,
       tx: Sender<()>,
       is_done: Arc<RwLock<bool>>) {
    let mut rng: XorShiftRng = rand::random();
    let mut ix_major: [usize; 9] = [0; 9];
    for i in 0..9 { ix_major[i] = i; }
    //rng.shuffle(&mut ix_major);

    let mut poss: [u16; 9] = [0; 9];
    loop {
        for x in 0..9 {
            //let major = ix_major[x];
            let major = x;
            let mut any = 0; // bit is 1 if any cells have that possibility
            let mut ovr = 0; // bit is 1 once a second cell has it
            for minor in 0..9 {
                let i = func(major, minor);
                let mut cell;
                {
                    cell = grid[i].read().unwrap();
                }
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
                // second pass, find cells with unique possibles
                for minor in 0..9 {
                    let i = func(major, minor);
                    let p = poss[minor] & uniqs;
                    if p != 0 {
                        // cell has unique possible
                        let mut cell = grid[i].write().unwrap();
                        cell.possible = p;
                        cell.check_possible();
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
