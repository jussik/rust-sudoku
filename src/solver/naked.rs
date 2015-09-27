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
    // store index and possibles for each non-value cell in each segment
    let mut poss: [(usize, u16); 9] = [(0, 0); 9];
    loop {
        let mut changed = false;
        for major in 0..9 {
            let mut count = 0;
            for minor in 0..9 {
                let i = func(major, minor);
                let cell = *grid[i].read().unwrap();
                if cell.value == -1 {
                    poss[count] = (i, cell.possible);
                    count += 1;
                }
            }
            if count < 3 {
                break;
            }
            'outer: for a in 0..count - 1 {
                for b in a + 1..count {
                    let (_, a_poss) = poss[a];
                    // pair with the same 2-value possibles
                    if a_poss == poss[b].1 && a_poss.count_ones() == 2 {
                        for o in 0..count {
                            let (i, o_poss) = poss[o];
                            if o_poss != 0
                                && o != a && o != b
                                && (o_poss & a_poss) != 0 {
                                    let mut cell = grid[i].write().unwrap();
                                    cell.possible &= !a_poss;
                                    cell.check_possible();
                                    changed = true;
                            }
                        }
                        // aleady found the pair, check next outer cell
                        continue 'outer;
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
