mod ascii_grid;

use time;
use clap::{Arg, App};
use std::io::{self,BufReader,BufRead};
use std::fs::File;

use sudoku::grid::Grid;
use sudoku::gen::Generator;

const EASY: &'static str = "\
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

type Formatter = fn(grid: &Grid) -> String;

pub fn run() {
    let args = App::new("Sudoku")
        .arg(Arg::with_name("demo")
            .short("d")
            .long("demo")
            .help("Run a demo instead of using an input"))
        .arg(Arg::with_name("input")
            .help("Input a file with multiple puzzles (one per line), reads from STDIN if no file specified")
            .value_name("FILE")
            .index(1))
        .arg(Arg::with_name("format")
            .short("f")
            .long("format")
            .help("Specify an output format")
            .takes_value(true)
            .value_name("FORMAT")
            .possible_values(&["oneline", "small", "large", "quiet"])
            .default_value("small"))
        .get_matches();

    let formatter: Option<Formatter> = match args.value_of("format") {
        Some("oneline") => Some(ascii_grid::create_oneline),
        Some("small") => Some(ascii_grid::create),
        Some("large") => Some(ascii_grid::create_large),
        _ => None,
    };

    let duration = if args.is_present("demo") {
        let grids = [
            Grid::parse(EASY),
            Grid::parse(MEDIUM),
            Grid::parse(HARD),
            Grid::parse(HARDEST),
            Generator::random().generate()
        ];
        let start = time::precise_time_ns();
        for grid in grids.into_iter() {
            solve(grid, formatter);
        }
        time::precise_time_ns() - start
    } else if args.is_present("input") {
        let f = File::open(args.value_of("input").unwrap()).unwrap();
        let mut file = BufReader::new(&f);
        solve_all(&mut file, formatter)
    } else {
        let stdin = io::stdin();
        let dur = solve_all(&mut stdin.lock(), formatter);
        dur
    };
    println!("{} ms", duration / 1000000);
}

fn solve_all<R: BufRead>(input: &mut R, formatter: Option<Formatter>) -> u64 {
    let mut duration: u64 = 0;
    for line in input.lines() {
        let grid = &Grid::parse(line.unwrap().as_str());
        let start = time::precise_time_ns();
        solve(grid, formatter);
        duration += time::precise_time_ns() - start;
    }
    duration
}

fn solve(grid: &Grid, formatter: Option<Formatter>) {
    match formatter {
        Some(fmt) => {
            println!("{}\n", fmt(&grid));
            match grid.solve() {
                Some(grid) => println!("{}\n", fmt(&grid)),
                None => println!("Invalid grid")
            };
        },
        None => { grid.solve(); }
    };
}
