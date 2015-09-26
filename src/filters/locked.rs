use super::*;

use std::thread;

pub fn rows(args: SolverArgs) {
    run(args);
}

pub fn columns(args: SolverArgs) {
    run(args);
}

fn run(args: SolverArgs) {
    let tx = args.tx;
    let is_done = args.is_done;
    let mut changed = false;
    /*for b in 0..9 {
        // find any values that are only in one row/column of this box
        for major in 0..3 {
            for minor in 0..3 {
            }
        }
    }*/
    tx.send(changed).unwrap();
    thread::yield_now();
    if *is_done.read().unwrap() {
        return;
    }
}
