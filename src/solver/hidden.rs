use super::*;

use std::thread;

//use rand;
//use rand::{XorShiftRng,Rng};

pub fn rows(args: &SolverArgs) {
    run(args, row_loc);
}
pub fn columns(args: &SolverArgs) {
    run(args, col_loc);
}
pub fn boxes(args: &SolverArgs) {
    run(args, box_loc);
}

fn run(args: &SolverArgs, func: LocFn) {
    /*let mut rng: XorShiftRng = rand::random();
    let mut ix_major: [usize; 9] = [0; 9];
    for i in 0..9 { ix_major[i] = i; }
    rng.shuffle(&mut ix_major);*/
    let grid = &args.cells;
    let tx = &args.tx;
    let is_done = &args.is_done;

    let mut poss: [u16; 9] = [0; 9];
    loop {
        let mut changed = false;
        for major in 0..9 {
            //let major = ix_major[x];
            let mut any = 0; // bit is 1 if any cells have that possibility
            let mut ovr = 0; // bit is 1 once a second cell has it
            for minor in 0..9 {
                let i = func(major, minor);
                let cell;
                {
                    cell = *grid[i].read().unwrap();
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
                    let p = poss[minor] & uniqs;
                    if p != 0 {
                        // cell has unique possible
                        let i = func(major, minor);
                        let mut cell = grid[i].write().unwrap();
                        cell.possible = p;
                        changed |= cell.check_possible();
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
