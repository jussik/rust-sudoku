use super::base::IteratorBase;

/// Iterates each row's cell adjacencies
pub struct Rows;
impl Rows {
    pub fn iter() -> RowIterator {
        RowIterator {
            iter: IteratorBase::new()
        }
    }
}
pub struct RowIterator {
    iter: IteratorBase
}
impl Iterator for RowIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let it = &mut self.iter;
        if it.step() {
            Some((it.cell, it.major * 9 + it.minor))
        } else {
            None
        }
    }
}
