pub struct IteratorBase {
    pub major: usize,
    pub minor: usize,
    pub minor_adj: usize
}
impl IteratorBase {
    pub fn new() -> IteratorBase {
        IteratorBase {
            major: 0,
            minor: 0,
            minor_adj: 0
        }
    }
    pub fn step(&mut self) -> bool {
        /*  As generator:
         *  for major in (0..9) {
         *      for minor in (0..8) {
         *          for minor_adj in (minor..9) {
         *              yield;
         *          }
         *      }
         *  }
        */
        loop {
            if self.minor_adj >= 8 {
                if self.minor >= 7 {
                    if self.major >= 8 {
                        return false;
                    }
                    self.major += 1;
                    self.minor = 0;
                    self.minor_adj = 0;
                } else {
                    self.minor += 1;
                    self.minor_adj = self.minor;
                }
            } else {
                self.minor_adj += 1;
                return true;
            }
        }
    }
}
