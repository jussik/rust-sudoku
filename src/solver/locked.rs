use super::*;

use std::thread;

pub fn rows(args: SolverArgs) {
    run(args, box_loc, box_loc, row_loc, row_root);
}
fn row_root(outer_maj: usize, outer_min: usize, inner_maj: usize) -> (usize, usize) {
    (outer_maj * 3 + inner_maj, outer_min * 3)
}

pub fn columns(args: SolverArgs) {
    run(args, box_loc, box_loc, col_loc, col_root);
}
fn col_root(outer_maj: usize, outer_min: usize, inner_maj: usize) -> (usize, usize) {
    (outer_min * 3 + inner_maj, outer_maj * 3)
}

pub fn box_rows(args: SolverArgs) {
    run(args, row_loc, col_loc, box_loc, row_root);
}
pub fn col_rows(args: SolverArgs) {
    run(args, col_loc, row_loc, inv_box_loc, col_root);
}
fn inv_box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 27
        + (major / 3) * 3
        + (minor % 3) * 9
        + (minor / 3)
}

#[test]
fn inv_box_test() {
    for maj in 0..9 {
        for min in 0..9 {
            print!(" {:2}",box_loc(maj, min));
        }
        println!("");
    }
    for maj in 0..9 {
        for min in 0..9 {
            print!(" {:2}",inv_box_loc(maj, min));
        }
        println!("");
    }
}

/// (outer major, outer minor, inner major) -> (root major, root minor)
type LockRootFn = fn(usize, usize, usize) -> (usize, usize);

fn run(args: SolverArgs,
       outer_func: LocFn,
       inv_outer_func: LocFn,
       inner_func: LocFn,
       root_func: LockRootFn) {
    let grid = args.cells;
    let tx = args.tx;
    let is_done = args.is_done;
    let mut changed = false;
    loop {
        for outer_maj in 0..3 {
            for outer_min in 0..3 {
                println!("\n");
                let outer = outer_maj * 3 + outer_min;
                // find any values that are only in one row/column of this box
                let mut poss: [u16; 3] = [0; 3]; // all possibles in each triple
                let mut values: u16 = 0;
                for major in 0..3 {
                    for minor in 0..3 {
                        let i = outer_func(outer, 0) + inner_func(major, minor);
                        println!("i:{}", i);
                        let cell = *grid[i].read().unwrap();
                        if cell.value == -1 {
                            poss[major] |= cell.possible;
                        } else {
                            values |= 1 << cell.value as u16;
                        }
                    }
                }
                for major in 0..3 {
                    // uniq is bits only in poss[major]
                    let uniq = poss[major] & !values
                        & !(poss[(major + 1) % 3] | poss[(major + 2) % 3]);
                    if uniq != 0 {
                        println!("");
                        println!("outer {} ({}, {})", outer, outer_maj, outer_min);
                        println!("uniq {:09b} in inner {}", uniq, major);
                        println!("outer origin {} i:{}", outer, outer_func(outer, 0));
                        println!("inner origin {} i:{}", major, inner_func(major, 0));
                        let (maj_root, min_root) = root_func(outer_maj, outer_min, major);
                        println!("root ({},{})", maj_root, min_root);
                        for minor in 3..9 {
                            //println!("inner ({}, {})", major, minor);
                            let i = inner_func(maj_root, (min_root + minor) % 9);
                            let mut cell = grid[i].write().unwrap();
                            if cell.value == -1 && cell.possible & uniq != 0 {
                                //cell.possible &= !uniq;
                                //cell.check_possible();
                                changed = true;
                                println!(
                                    "----- changed ({}, {}) i:{}",
                                    maj_root, (min_root + minor) % 9, i);
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
