use super::base::IteratorBase;

/// Iterates each column's cell adjacencies
pub struct Columns;
impl Columns {
    pub fn iter() -> ColumnIterator {
        ColumnIterator {
            iter: IteratorBase::new()
        }
    }
}
pub struct ColumnIterator {
    iter: IteratorBase
}
impl Iterator for ColumnIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let it = &mut self.iter;
        if it.step() {
            let cell = it.minor * 9 + it.major;
            let other = it.minor_adj * 9 + it.major;
            Some((cell, other))
        } else {
            None
        }
    }
}
