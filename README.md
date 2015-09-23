# rust-sudoku
Sudoku solver and generator implemented in Rust

# TODO

* Implement solvers based on http://angusj.com/sudoku/hints.php
* Puzzle generator
  1. Start with a hard coded filled out puzzle
  1. Shuffle values in a way that retains validity
  1. Remove random values one by one and return the first puzzle that can be solved using the specified difficulty.  
     e.g. easy means at least 10 missing values and only simple row, column and box solvers
