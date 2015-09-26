use super::*;

use std::thread;

//use rand;
//use rand::{XorShiftRng,Rng};

/// Remove possible values based on cells in the same row
pub fn rows(args: SolverArgs) {
    run(args, row_loc);
}

/// Remove possible values based on cells in the same column
pub fn columns(args: SolverArgs) {
    run(args, col_loc);
}

/// Remove possible values based on cells in the same 3x3 box
pub fn boxes(args: SolverArgs) {
    run(args, box_loc);
}

/// Remove possibilities based on adjacent values
fn run(args: SolverArgs, func: LocFn) {
    // randomise walk order to minimise successive waits for other threads
    /*let mut rng: XorShiftRng = rand::random();

    let mut ix_major: [usize; 9] = [0; 9];
    for i in 0..9 { ix_major[i] = i; }
    rng.shuffle(&mut ix_major);

    let mut ix_minor: [usize; 8] = [0; 8];
    for i in 0..8 { ix_minor[i] = i; }
    rng.shuffle(&mut ix_minor);*/
    let grid = args.cells;
    let tx = args.tx;
    let is_done = args.is_done;

    loop {
        let mut changed = false;
        for major in 0..9 {
            //let major = ix_major[x]; // iterating arrays emits refs, need value
            for minor in 0..8 {
                //let minor = ix_minor[y];
                let i = func(major, minor);
                let ci;
                {
                    ci = *grid[i].read().unwrap();
                }
                for minor_adj in minor + 1..9 {
                    let j = func(major, minor_adj);
                    let cj;
                    {
                        cj = *grid[j].read().unwrap();
                    }
                    if ci.value != -1 {
                        if cj.value == -1 && cj.is_possible(ci.value) {
                            let mut cell = grid[j].write().unwrap();
                            changed |= cell.remove_possible(ci.value);
                        }
                    } else if cj.value != -1 && ci.is_possible(cj.value) {
                        let mut cell = grid[i].write().unwrap();
                        changed |= cell.remove_possible(cj.value);
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
