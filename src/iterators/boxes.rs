use super::base::IteratorBase;

/// Iterates each 3x3 box's cell adjacencies
pub struct Boxes;
impl Boxes {
    pub fn iter() -> BoxIterator {
        BoxIterator {
            iter: IteratorBase::new()
        }
    }
}
pub struct BoxIterator {
    iter: IteratorBase
}
impl Iterator for BoxIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        let it = &mut self.iter;
        if it.step() {
            let major_offset = (it.major % 3) * 3 + (it.major / 3) * 27;
            let cell = it.minor % 3
                + (it.minor / 3) * 9
                + major_offset;
            let other = it.minor_adj % 3
                + (it.minor_adj / 3) * 9
                + major_offset;
            Some((cell, other))
        } else {
            None
        }
    }
}
