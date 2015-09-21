extern crate sudoku;
mod ascii_grid;

const SIMPLE: &'static str = "\
0 0 3 | 0 2 0 | 6 0 0\
9 0 0 | 3 0 5 | 0 0 1\
0 0 1 | 8 0 6 | 4 0 0\
------+-------+------\
0 0 8 | 1 0 2 | 9 0 0\
7 0 0 | 0 0 0 | 0 0 8\
0 0 6 | 7 0 8 | 2 0 0\
------+-------+------\
0 0 2 | 6 0 9 | 5 0 0\
8 0 0 | 2 0 3 | 0 0 9\
0 0 5 | 0 1 0 | 3 0 0";


fn main() {
    let mut grid = sudoku::Grid::parse(SIMPLE);
    println!("Start:");
    println!("{}", ascii_grid::create(&grid));
    let mut i = 1;
    loop {
        match grid.step() {
            Some((step, false)) => {
                grid = step;
                println!("");
                println!("Iteration {}:", i);
                println!("{}", ascii_grid::create_large(&grid));
            },
            Some((step, true)) => {
                grid = step;
                println!("");
                if grid.is_solved() {
                    println!("Solved in {} iterations :)", i);
                    println!("{}", ascii_grid::create(&grid));
                } else {
                    println!("Could not solve in {} iterations :|", i);
                    println!("{}", ascii_grid::create_large(&grid));
                }
                break;
            },
            None => {
                println!("Invalid grid :(");
                break;
            }
        }
        i += 1;
    }
}
