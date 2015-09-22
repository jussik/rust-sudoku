use std::sync::{Arc, RwLock};
use std::vec::Vec;

use ::grid::{Cell};
use ::iterators::{Rows, Columns, Boxes};

/// Remove possibilities based on adjacent values
pub fn remove_possibles(grid: Vec<Arc<RwLock<Cell>>>) {
    loop {
        let mut changed = false;
        for (i, j) in Rows::iter()
                .chain(Columns::iter())
                .chain(Boxes::iter()) {
            let mut ival;
            let mut jval;
            {
                ival = grid[i].read().unwrap().value;
                jval = grid[j].read().unwrap().value;
            }
            if ival != -1 {
                if ival == jval {
                    //send(&tx, Op::Invalidate("duplicate_adjacent"));
                    return;
                } else if jval == -1 {
                    let mut cell = grid[j].write().unwrap();
                    changed |= cell.remove_possible(ival);
                    //send(&tx, Op::RemovePossible(j, ival));
                }
            } else if jval != -1 {
                let mut cell = grid[i].write().unwrap();
                changed |= cell.remove_possible(jval);
                //send(&tx, Op::RemovePossible(i, jval));
            }
        }
        if !changed {
            return;
        }
    }
}
