use ::grid::{Grid, Op};
use ::iterators::{Rows, Columns, Boxes};
use std::sync::mpsc::Sender;

/// Remove possibilities based on adjacent values
pub fn remove_possibles(grid: &Grid, tx: Sender<Option<Op>>) {
    for (i, j) in Rows::iter()
            .chain(Columns::iter())
            .chain(Boxes::iter()) {
        let cell = grid.values[i].value;
        let adj = grid.values[j].value;
        if cell != -1 {
            if cell == adj {
                send(&tx, Op::Invalidate("duplicate_adjacent"));
            } else if adj == -1 {
                send(&tx, Op::RemovePossible(j, cell));
            }
        } else if adj != -1 {
            send(&tx, Op::RemovePossible(i, adj));
        }
    }
    end(&tx);
}
/// Set values if no other possibilities exist
pub fn set_uniques(grid: &Grid, tx: Sender<Option<Op>>) {
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
    end(&tx);
}
fn send(tx: &Sender<Option<Op>>, op: Op) {
    tx.send(Some(op)).unwrap();
}
fn end(tx: &Sender<Option<Op>>) {
    tx.send(None).unwrap();
}
