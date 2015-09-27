use super::*;

use std::thread;

pub fn rows(args: SolverArgs) {
    run(args, row_loc);
}
pub fn columns(args: SolverArgs) {
    run(args, col_loc);
}
pub fn boxes(args: SolverArgs) {
    run(args, box_loc);
}

fn run(args: SolverArgs, func: LocFn) {
    let grid = args.cells;
    let tx = args.tx;
    let is_done = args.is_done;
    let mut poss: [u16; 9] = [0; 9];
    loop {
        let mut changed = false;
        for major in 0..9 {
            for minor in 0..9 {
                let i = func(major, minor);
                let cell = *grid[i].read().unwrap();
                poss[minor] =
                    if cell.value == -1 {
                        cell.possible
                    } else {
                        0
                    }
            }
            for a in 0..8 {
                for b in a + 1..9 {
                    let a_poss = poss[a];
                    if poss[a] == poss[b] && a_poss.count_ones() == 2 {
                        for o in 0..9 {
                            let o_poss = poss[o];
                            if o_poss != 0
                                && o != a && o != b
                                && (o_poss & a_poss) != 0 {
                                    let i = func(major, o);
                                    let mut cell = grid[i].write().unwrap();
                                    cell.possible &= !a_poss;
                                    cell.check_possible();
                                    changed = true;
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
