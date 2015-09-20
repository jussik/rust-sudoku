/// Iterates each column's cell adjacencies
///
/// Emits a tuple of two cell locations
/// Only emits each pair of cells once, i.e. (3,5) but not (5,3)
pub struct ColumnIterator {
    tgt: usize,
    col: usize,
    row: usize,
    new_col: bool
}
impl ColumnIterator {
    pub fn new() -> ColumnIterator {
        ColumnIterator {
            tgt: 0,
            col: 0,
            row: 0,
            new_col: true
        }
    }
}
impl Iterator for ColumnIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        /*
         *  for tgt in (0..81) {
         *      let col = tgt % 9;
         *      for row in (tgt / 9 + 1 ..9) {
         *          yield (tgt, row * 9 + col);
         *      }
         *  }
         */
        loop {
            if self.new_col {
                if self.tgt > 80 {
                    // all columns done
                    return None;
                } else {
                    // check next column
                    self.col = self.tgt % 9;
                    self.row = self.tgt / 9;
                    self.new_col = false;
                }
            } else {
                if self.row >= 8 {
                    // past last cell in column, go to next
                    self.tgt += 1;
                    self.new_col = true;
                } else {
                    // send cell
                    self.row += 1;
                    return Some((self.tgt, self.row * 9 + self.col));
                }
            }
        }
    }
}
