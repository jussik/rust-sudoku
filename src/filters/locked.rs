use super::*;

use std::sync::{Arc, RwLock};
use std::sync::mpsc::Sender;
use std::vec::Vec;
use std::thread;

use ::grid::Cell;

pub fn rows(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<bool>,
            is_done: Arc<RwLock<bool>>) {
    run(grid, tx, is_done);
}

pub fn columns(grid: Vec<Arc<RwLock<Cell>>>,
               tx: Sender<bool>,
               is_done: Arc<RwLock<bool>>) {
    run(grid, tx, is_done);
}

fn run(grid: Vec<Arc<RwLock<Cell>>>,
            tx: Sender<bool>,
            is_done: Arc<RwLock<bool>>) {
    let mut changed = false;
    for b in 0..9 {
        // find any values that are only in one row/column of this box
        for major in 0..3 {
            for minor in 0..3 {
            }
        }
    }
    tx.send(changed).unwrap();
    thread::yield_now();
    if *is_done.read().unwrap() {
        return;
    }
}
