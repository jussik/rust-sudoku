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
/*pub fn find_hidden(grid: &Grid, tx: Sender<Option<Op>>) {
    for seg in (0..1) { // 0..9
        let counts = [0; 9];
        for c in (0..9) {
            let cell = grid.values[c];
            if(cell.value == -1) {
                println!("{}", cell.possible);
            }
        }
    }
    end(&tx);
}*/
fn send(tx: &Sender<Option<Op>>, op: Op) {
    tx.send(Some(op)).unwrap();
}
fn end(tx: &Sender<Option<Op>>) {
    tx.send(None).unwrap();
}
