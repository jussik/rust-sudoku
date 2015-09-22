# rust-sudoku
Sudoku solver and generator implemented in Rust

# TODO

* Multithreading
  * Send `Vec<RwLock<&mut Cell>>` (or similar) to filters to ensure cell-level locking granularity
  * https://gist.github.com/jussik/e15ab7343c9585cebd79
* Implement solvers based on http://angusj.com/sudoku/hints.php
* Puzzle generator
