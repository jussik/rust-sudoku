mod ascii_grid;

use sudoku::grid::Grid;
use sudoku::gen::Generator;

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

const MEDIUM: &'static str = "\
043080250\
600000000\
000001094\
900004070\
000608000\
010200003\
820500000\
000000005\
034090710";

const HARD: &'static str = "\
800|000|000\
003|600|000\
070|090|200\

050|007|000\
000|045|700\
000|100|030\

001|000|068\
008|500|010\
090|000|400";

pub fn run() {
    let grid = Grid::parse(MEDIUM);
    //let grid = Generator::random().generate();
    println!("{}\n", ascii_grid::create(&grid));
    match grid.solve() {
        Some(grid) => println!("{}", ascii_grid::create_large(&grid)),
        None => println!("Invalid grid")
    }
}
