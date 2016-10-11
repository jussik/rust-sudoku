# rust-sudoku
Sudoku solver and generator implemented in Rust

* Performs no heap allocations during the solving process, and only allocates on the stack when no logical reduction possible (i.e. a guess is required)
* Implement solvers based on http://angusj.com/sudoku/hints.php
* Seems to be fairly fast, solves the ["World's hardest sudo puzzle"](http://www.telegraph.co.uk/news/science/science-news/9359579/Worlds-hardest-sudoku-can-you-crack-it.html) in under one millisecond

# TODO

* Different difficulty levels for generator
