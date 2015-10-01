pub mod simple;
pub mod hidden;
pub mod locked;
pub mod naked;

use std::thread;
use std::vec::Vec;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender, Receiver};

use ::grid::Cell;

pub type LocFn = fn(usize, usize) -> usize;

pub fn row_loc(major: usize, minor: usize) -> usize {
    major * 9 + minor
}
pub fn col_loc(major: usize, minor: usize) -> usize {
    minor * 9 + major
}
pub fn box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 3
        + (major / 3) * 27
        + (minor % 3)
        + (minor / 3) * 9
}
pub fn inv_box_loc(major: usize, minor: usize) -> usize {
    (major % 3) * 27
        + (major / 3) * 3
        + (minor % 3) * 9
        + (minor / 3)
}

/// Start a number of solvers in threads and pass them cloned arguments
///
/// $func is a solver function
/// $arg is clonable data to pass to solver
/// $handles is vector of JoinHandles to add new threads to
///
/// e.g. `start_solvers!([fn1, fn2, fn3](arg1, arg2) -> handles);`
macro_rules! start_solvers {
    (
        [ $func:expr, $($rest:expr),+ ]
        ( $($arg:ident),* )
        -> $handles:ident
     ) => {
        start_solvers!([$func]($($arg),*) -> $handles);
        start_solvers!([$($rest),+]($($arg),*) -> $handles);
    };
    (
        [ $func:expr ]
        ( $($arg:ident),* )
        -> $handles:ident
     ) => {{
        $( let $arg = $arg.clone(); )*
        $handles.push(thread::spawn(move || {
            $func(&$($arg),*);
        }));
    }};
}

pub struct Solver {
    pub parallel: bool
}

#[derive(Clone)]
pub struct SolverArgs {
    cells: Vec<Arc<RwLock<Cell>>>,
    tx: Sender<bool>,
    is_done: Arc<RwLock<bool>>
}

impl Solver {
    pub fn new(parallel: bool) -> Solver {
        Solver {
            parallel: parallel
        }
    }
    /// Solve the puzzle in place, returns `true` if successful
    pub fn solve_mut(&self, grid: &mut [Cell; 81]) -> bool {
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
        let (tx, rx) = channel::<bool>();
        let args = SolverArgs {
            cells: grid.iter()
                .map(|cell| Arc::new(RwLock::new(cell.clone())))
                .collect::<Vec<_>>(),
            tx: tx,
            is_done: Arc::new(RwLock::new(false))
        };

        let mut done = false;
        if self.parallel {
            start_solvers!([
                simple::rows,
                simple::columns,
                simple::boxes,
                hidden::rows,
                hidden::columns,
                hidden::boxes
            ](args) -> handles);

            for _ in 0..1000 {
                rx.recv().unwrap();
                for i in 0..81 {
                    if args.cells[i].read().unwrap().value == -1 {
                        done = false;
                        break;
                    }
                }
                if done {
                    let mut d = args.is_done.write().unwrap();
                    *d = true;
                    break;
                }
                while let Ok(_) = rx.try_recv() {
                    // swallow pings received during count
                }
            }
            if !done {
                // tell threads to stop
                let mut d = args.is_done.write().unwrap();
                *d = true;
            }

            for h in handles {
                h.join().unwrap();
            }
        } else {
            {
                let mut d = args.is_done.write().unwrap();
                *d = true;
            }
            done = self.solve_seq(&args, &rx, 0);
        }
        for i in 0..81 {
            let cell = args.cells[i].read().unwrap();
            grid[i] = *cell;
        }
        done
    }

    fn solve_seq(&self, args: &SolverArgs, rx: &Receiver<bool>, depth: u32) -> bool {
        loop {
            simple::rows(&args);
            simple::columns(&args);
            simple::boxes(&args);
            hidden::rows(&args);
            hidden::columns(&args);
            hidden::boxes(&args);
            locked::rows(&args);
            locked::columns(&args);
            locked::box_rows(&args);
            locked::box_cols(&args);
            naked::rows(&args);
            naked::columns(&args);
            naked::boxes(&args);

            let mut done = true;
            for major in 0..9 {
                let mut vals: [u16; 3] = [0; 3];
                for minor in 0..9 {
                    for f in 0..3 {
                        let i = CHECK_FUNCS[f](major, minor);
                        let cell = *args.cells[i].read().unwrap();
                        if cell.value == -1 {
                            if cell.possible == 0 {
                                return false;
                            }
                            done = false;
                        } else {
                            let val_bit = 1 << cell.value;
                            if (vals[f] & val_bit) != 0 {
                                return false;
                            }
                            vals[f] |= val_bit;
                        }
                    }
                }
            }
            if done {
                return true;
            }

            let mut changed = false;
            for _ in 0..13 {
                changed |= rx.recv().unwrap();
            }
            if !changed {
                return self.guess(&args, &rx, depth);
            }
        }
    }

    fn guess(&self, args: &SolverArgs, rx: &Receiver<bool>, depth: u32) -> bool  {
        for poss in 2..10 { // find cells with least possibles first
            for c in 0..81 {
                let cell: Cell;
                {
                    cell = *args.cells[c].read().unwrap();
                }
                if cell.value == -1 {
                    let count = cell.possible.count_ones() as u16;
                    if count == poss {
                        for v in 0..9 {
                            // check each value in bitfield, is there a better way?
                            if cell.possible & VALS[v] != 0 {
                                let new_args = SolverArgs {
                                    cells: args.cells.iter()
                                        .map(|arc| {
                                            // need to copy cell data
                                            let cell = *arc.read().unwrap();
                                            Arc::new(RwLock::new(cell))
                                        })
                                    .collect::<Vec<_>>(),
                                    tx: args.tx.clone(),
                                    is_done: args.is_done.clone()
                                };
                                {
                                    let mut cell = new_args.cells[c]
                                        .write().unwrap();
                                    cell.value = v as i8;
                                    cell.possible = 1 << v;
                                }
                                if self.solve_seq(&new_args, &rx, depth + 1) {
                                    for i in 0..81 {
                                        let src = *new_args.cells[i].read().unwrap();
                                        let mut dst = args.cells[i].write().unwrap();
                                        dst.value = src.value;
                                        dst.possible = src.possible;
                                    }
                                    return true;
                                }
                            }
                        }
                        return false;
                    }
                }
            }
        }
        false
    }
}

static CHECK_FUNCS: [LocFn; 3] = [row_loc, col_loc, box_loc];

static VALS: [u16; 9] = [
    0x001,
    0x002,
    0x004,
    0x008,
    0x010,
    0x020,
    0x040,
    0x080,
    0x100,
];
