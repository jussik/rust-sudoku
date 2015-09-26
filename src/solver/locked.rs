use super::*;

use std::thread;

pub fn rows(args: SolverArgs) {
    run(args, row_loc, row_box);
}
fn row_box(b: usize, po: usize) -> usize {
    (b + po) % 3
}

pub fn columns(args: SolverArgs) {
    run(args, col_loc, col_box);
}
fn col_box(b: usize, po: usize) -> usize {
    ((b % 3) + po * 3)
}

fn run(args: SolverArgs, func: LocFn, box_func: LocFn) {
    let grid = args.cells;
    let tx = args.tx;
    let is_done = args.is_done;
    let mut changed = false;
    for b in 0..9 {
        // find any values that are only in one row/column of this box
        let box_offset = box_loc(b, 0); // left top cell of box
        let mut poss: [u16; 3] = [0; 3]; // all possibles in each row/column
        for major in 0..3 {
            for minor in 0..3 {
                let i = box_offset + func(major, minor);
                let cell = *grid[i].read().unwrap();
                if cell.value == -1 {
                    poss[major] |= cell.possible;
                }
            }
        }
        for p in 0..3 {
            // uniq is bits only in poss[p]
            let uniq = poss[p] & !(poss[(p + 1) % 3] | poss[(p + 2) % 3]);
            if uniq != 0 {
                //println!("uniq: {:09b} in minor {}", uniq, p);
                let b_maj = (b / 3) * 3;
                //println!("box {} (in maj {})", b, b_maj/3);
                for po in 1..3 {
                    // each boxes to remove possibles from
                    let bo = (b_maj + box_func(b, po)) % 9;
                    //println!("others: {}", bo);
                    let box_offset = box_loc(bo, 0);
                    for minor in 0..3 {
                        let i = box_offset + func(p, minor);
                        //println!("remove from {}", i);
                        let mut cell = grid[i].write().unwrap();
                        cell.possible &= !uniq;
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
