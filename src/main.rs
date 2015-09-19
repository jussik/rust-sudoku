extern crate sudoku;

fn main() {
    let grid = sudoku::Grid::parse("\
        003|020|600\
        900|305|001\
        001|806|400\
        ---+---+---\
        008|102|900\
        700|000|008\
        006|708|200\
        ---+---+---\
        002|609|500\
        800|203|009\
        005|010|300");
    let solved = grid.solve();
    println!("Start:\n{}", grid.to_string());
    println!("Valid? {}", grid.is_valid());
    if solved.is_some() {
        println!("Finished:\n{}", solved.unwrap().to_string());
    } else {
        println!("Not solved!");
    }
}
