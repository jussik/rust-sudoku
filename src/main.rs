extern crate sudoku;

fn main() {
    let g = sudoku::Grid::parse("\
        123 200 300\
        456 500 600\
        789 800 900\

        100 200 300\
        400 500 600\
        700 800 900\

        100 200 300\
        400 500 600\
        700 800 900");
    println!("{}", g.unwrap().to_string());
}
