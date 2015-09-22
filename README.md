# rust-sudoku
Sudoku solver and generator implemented in Rust

# TODO

* Multithreading
  * Send `Vec<RwLock<&mut Cell>>` (or similar) to filters to ensure cell-level locking granularity
* Implement solvers based on http://angusj.com/sudoku/hints.php
* Puzzle generator
