mod ascii_grid;

use sudoku::grid::Grid;
use sudoku::gen::Generator;

const SIMPLE: &'static str = "\
003|020|600\
900|305|001\
001|806|400\

008|102|900\
700|000|008\
006|708|200\

002|609|500\
800|203|009\
005|010|300";

const MEDIUM: &'static str = "\
043|080|250\
600|000|000\
000|001|094\

900|004|070\
000|608|000\
010|200|003\

820|500|000\
000|000|005\
034|090|710";

const HARD: &'static str = "\
000|700|000\
100|000|000\
000|430|200\

000|000|006\
000|509|000\
000|000|418\

000|081|000\
002|000|050\
040|000|300";

const HARDEST: &'static str = "\
800|000|000\
003|600|000\
070|090|200\

050|007|000\
000|045|700\
000|100|030\

001|000|068\
008|500|010\
090|000|400";

const EMPTY: &'static str = "\
000|000|000\
000|000|000\
000|000|000\

000|000|000\
000|000|000\
000|000|000\

000|000|000\
000|000|000\
000|000|000";

pub fn run() {
    //let grid = Grid::parse(HARDEST);
    let grid = Generator::random().generate();
    println!("{}\n", ascii_grid::create(&grid));
    match grid.solve() {
        Some(grid) => {
            println!("{}", if grid.solved {
                ascii_grid::create(&grid)
            } else {
                ascii_grid::create_large(&grid)
            })
        },
        None => println!("Invalid grid")
    }
}
