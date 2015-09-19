/// Iterates each rows' cell adjacencies
///
/// Emits a tuple of two cell locations
/// Only emits each pair of cells once, i.e. (3,5) but not (5,3)
pub struct RowIterator {
    tgt: u32,
    row: u32,
    adj: u32,
    new_row: bool
}
impl RowIterator {
    pub fn new() -> RowIterator {
        RowIterator {
            tgt: 0,
            row: 0,
            adj: 0,
            new_row: true
        }
    }
}
impl Iterator for RowIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        /*
         *  for tgt in (0..81) {
         *      let row = tgt / 9 * 9;
         *      for adj in (tgt % 9 + 1 .. 9) {
         *          yield (tgt, row + adj);
         *      }
         *  }
         */
        loop {
            if self.new_row {
                if self.tgt > 80 {
                    // all rows done
                    return None;
                } else {
                    // check next row
                    self.row = self.tgt / 9 * 9;
                    self.adj = self.tgt % 9 + 1;
                    self.new_row = false;
                }
            } else {
                if self.adj > 8 {
                    // past last cell in row, check next row
                    self.tgt = self.tgt + 1;
                    self.new_row = true;
                } else {
                    // send cell
                    let tmp_adj = self.adj;
                    self.adj = self.adj + 1;
                    return Some((
                            self.tgt as usize,
                            (self.row + tmp_adj) as usize));
                }
            }
        }
    }
}
