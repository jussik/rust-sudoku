/// A single cell in a `Grid`
#[derive(Copy, Clone)]
pub struct Cell {
    /// Value of the cell (0-8), -1 if unknown
    pub value: i8,
    /// Bit field of possible values
    pub possible: u16
}
impl Cell {
    /// Sets the current value, returns `true` if changed
    pub fn set(&mut self, value: i8) -> bool {
        if self.value != -1 {
            false
        } else {
            self.value = value;
            true
        }
    }
    /// Removes a value from its possibilities
    /// Sets value if only one possibility remains
    /// Returns `true` if possibilities or value have changed
    pub fn remove_possible(&mut self, value: i8) -> bool {
        let bit = 1 << value;
        if self.possible & bit == 0 || self.value != -1 {
            false
        } else {
            self.possible &= !bit;
            self.check_possible();
            true
        }
    }
    pub fn is_possible(&self, value: i8) -> bool {
        self.possible & (1 << value) != 0
    }

    /// Checks if only one possibility remains and applies that value
    pub fn check_possible(&mut self) -> bool {
        if self.possible.count_ones() == 1 && self.value == -1 {
            self.value = match self.possible {
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
            true
        } else {
            false
        }
    }
}
