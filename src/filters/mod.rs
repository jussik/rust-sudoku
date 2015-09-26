pub mod simple;
pub mod hidden;
pub mod locked;

use std::thread;
use std::vec::Vec;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;

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

impl Solver {
    /// Solve the puzzle in place, returns `true` if successful
    pub fn solve_mut(&self, grid: &mut [Cell; 81]) -> bool {
        let cells = grid.iter()
            .map(|cell| Arc::new(RwLock::new(cell.clone())))
            .collect::<Vec<_>>();
        let is_done = Arc::new(RwLock::new(false));
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
        let (tx, rx) = channel::<bool>();

        let mut done = false;
        if self.parallel {
            start_solvers!([
                simple::rows,
                simple::columns,
                simple::boxes,
                hidden::rows,
                hidden::columns,
                hidden::boxes
            ](cells, tx, is_done) -> handles);

            for _ in 0..1000 {
                rx.recv().unwrap();
                for i in 0..81 {
                    if cells[i].read().unwrap().value == -1 {
                        done = false;
                        break;
                    }
                }
                if done {
                    let mut d = is_done.write().unwrap();
                    *d = true;
                    break;
                }
                while let Ok(_) = rx.try_recv() {
                    // swallow pings received during count
                }
            }
            if !done {
                // tell threads to stop
                let mut d = is_done.write().unwrap();
                *d = true;
            }

            for i in 0..81 {
                let cell = cells[i].read().unwrap();
                grid[i] = *cell;
            }
            for h in handles {
                h.join().unwrap();
            }
        } else {
            {
                let mut d = is_done.write().unwrap();
                *d = true;
            }
            for i in 1..100 {
                simple::rows(cells.clone(), tx.clone(), is_done.clone());
                simple::columns(cells.clone(), tx.clone(), is_done.clone());
                simple::boxes(cells.clone(), tx.clone(), is_done.clone());
                //hidden::rows(cells.clone(), tx.clone(), is_done.clone());
                //hidden::columns(cells.clone(), tx.clone(), is_done.clone());
                //hidden::boxes(cells.clone(), tx.clone(), is_done.clone());
                locked::rows(cells.clone(), tx.clone(), is_done.clone());

                done = true;
                for i in 0..81 {
                    if cells[i].read().unwrap().value == -1 {
                        done = false;
                        break;
                    }
                }
                if done {
                    println!("solved after {} iterations", i);
                    break;
                }

                let mut changed = false;
                for _ in 0..4 {
                    changed |= rx.recv().unwrap();
                }
                if !changed {
                    println!("gave up after {} iterations", i);
                    break;
                }
            }
            for i in 0..81 {
                let cell = cells[i].read().unwrap();
                grid[i] = *cell;
            }
        }
        done
    }
}
