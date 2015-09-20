pub struct IteratorBase {
    pub cell: usize,
    pub major: usize,
    pub minor: usize,
    new_cell: bool
}
impl IteratorBase {
    pub fn new() -> IteratorBase {
        IteratorBase {
            cell: 0,
            major: 0,
            minor: 0,
            new_cell: true
        }
    }
    pub fn step(&mut self) -> bool {
        loop {
            if self.new_cell {
                if self.cell > 80 {
                    return false;
                } else {
                    self.major = self.cell / 9;
                    self.minor = self.cell % 9;
                    self.new_cell = false;
                }
            } else {
                if self.minor >= 8 {
                    self.cell += 1;
                    self.new_cell = true;
                } else {
                    self.minor += 1;
                    return true;
                }
            }
        }
    }
}
