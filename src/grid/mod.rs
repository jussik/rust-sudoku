mod cell;
pub use self::cell::Cell;

use std::thread;
use std::vec::Vec;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::channel;

use filters::simple;

/// A 9x9 sudoku grid
#[derive(Copy)]
pub struct Grid {
    /// Values in the grid in row-major order
    pub values: [Cell; 81],
    /// Whether or not this is a valid puzzle
    valid: bool,
    /// Whether or not this puzzle is solved
    solved: bool
}
impl Clone for Grid {
    fn clone(&self) -> Self {
        //println!("Grid copy");
        *self
    }
}

// $data is data to pass to solver
// $handlers is vector of JoinHandles to add new threads to
// $func is a solver function
macro_rules! start_solvers {
    ($data:ident, $handles:ident, $tx:ident, $st:ident, $($func:expr),*) => {{
        $(
            let data = $data.clone();
            let tx = $tx.clone();
            let st = $st.clone();
            $handles.push(thread::spawn(move || {
                $func(data, tx, st);
            }));
        )*
    }}
}

impl Grid {
    /// Parse a string containing a list of numbers into a `Grid`
    ///
    /// Values are read in row-major order
    /// All non-digit characters are ignored
    /// 0 implies the value is unknown
    /// If less than 81 values are found, the remainder are unknown
    /// Values after the 81st are ignored
    pub fn parse(input: &str) -> Grid {
        let mut g = Grid {
            values: [Cell {
                value: -1,
                possible: (1 << 9) - 1
            }; 81],
            valid: true,
            solved: false
        };
        for (i, d) in input.chars()
            .filter_map(move |c| c.to_digit(10))
            .take(81)
            .enumerate() {
                if d != 0 {
                    let v = d as i8 - 1;
                    g.values[i].value = v;
                    g.values[i].possible = 1 << v;
                }
            }
        g
    }

    /// Attempts to solve the puzzle
    ///
    /// Returns a filled out `Grid` when the solver has finished
    /// Returns `None` if the puzzle is invalid
    pub fn solve(&self) -> Option<Grid> {
        if self.valid {
            let mut g2 = self.clone();
            g2.solve_mut();
            if self.valid {
                return Some(g2);
            }
        }
        None
    }

    /// Solve the sudoku puzzle in the current `Grid`
    fn solve_mut(&mut self) {
        let cells = self.values.iter()
            .map(|cell| Arc::new(RwLock::new(cell.clone())))
            .collect::<Vec<_>>();
        let is_done = Arc::new(RwLock::new(false));
        let mut handles: Vec<thread::JoinHandle<()>> = Vec::new();
        let (tx, rx) = channel::<()>();

        start_solvers!(cells, handles, tx, is_done,
            simple::rows,
            simple::columns,
            simple::boxes
        );

        let mut done = false;
        for _ in 0..100 {
            rx.recv().unwrap();
            done = true;
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
        self.solved = done;
        if !done {
            // tell threads to stop
            let mut d = is_done.write().unwrap();
            *d = true;
        }

        for i in 0..81 {
            let cell = cells[i].read().unwrap();
            self.values[i] = *cell;
        }
        for h in handles {
            h.join().unwrap();
        }
    }
}
