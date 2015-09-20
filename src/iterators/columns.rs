use super::base::IteratorBase;

/// Iterates each row's cell adjacencies
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
            Some(((it.cell % 9) * 9 + it.major, it.minor * 9 + it.major))
        } else {
            None
        }
    }
}
