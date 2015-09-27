pub mod simple;
pub mod hidden;
pub mod locked;
pub mod naked;

use std::thread;
use std::vec::Vec;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Sender};

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
            $func($($arg),*);
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
            for i in 1..100 {
                simple::rows(args.clone());
                simple::columns(args.clone());
                simple::boxes(args.clone());
                hidden::rows(args.clone());
                hidden::columns(args.clone());
                hidden::boxes(args.clone());
                locked::rows(args.clone());
                locked::columns(args.clone());
                locked::box_rows(args.clone());
                locked::col_rows(args.clone());
                naked::rows(args.clone());
                naked::columns(args.clone());
                naked::boxes(args.clone());

                done = true;
                for i in 0..81 {
                    if args.cells[i].read().unwrap().value == -1 {
                        done = false;
                        break;
                    }
                }
                if done {
                    println!("solved after {} iterations", i);
                    break;
                }

                let mut changed = false;
                for _ in 0..13 {
                    changed |= rx.recv().unwrap();
                }
                //break;
                if !changed {
                    println!("gave up after {} iterations", i);
                    break;
                }
            }
        }
        for i in 0..81 {
            let cell = args.cells[i].read().unwrap();
            grid[i] = *cell;
        }
        done
    }
}
