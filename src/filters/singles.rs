use ::grid::{Grid, Op};
use ::iterators::rows::Rows;
use ::iterators::columns::Columns;
use std::sync::mpsc::Sender;

pub fn remove_possibles(grid: &Grid, tx: Sender<Option<Op>>) {
    // remove possibilities based on adjacent values
    for (i, j) in Rows::iter().chain(Columns::iter()) {
        let cell = grid.values[i].value;
        let adj = grid.values[j].value;
        if cell != -1 {
            if cell == adj {
                send(&tx, Op::Invalidate("duplicate_adjacent"));
                return;
            } else if adj == -1 {
                send(&tx, Op::RemovePossible(j, cell));
            }
        } else if adj != -1 {
            send(&tx, Op::RemovePossible(i, adj));
        }
    }
}
pub fn set_uniques(grid: &Grid, tx: Sender<Option<Op>>) {
    // set values if no other possibilities exist
    for (i, cell) in grid.values.iter().enumerate() {
        if cell.value == -1 {
            let ones = cell.possible.count_ones();
            if ones == 1 {
                let val = match cell.possible {
                    0x001 => 0,
                    0x002 => 1,
                    0x004 => 2,
                    0x008 => 3,
                    0x010 => 4,
                    0x020 => 5,
                    0x040 => 6,
                    0x080 => 7,
                    0x100 => 8,
                    _ => -1
                };
                send(&tx, Op::SetValue(i, val));
            } else if ones == 0 {
                send(&tx, Op::Invalidate("no_possibles"));
            }
        }
    }
}
fn send(tx: &Sender<Option<Op>>, op: Op) {
    tx.send(Some(op)).unwrap();
}
