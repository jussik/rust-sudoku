use super::*;
use ::grid::Cell;

macro_rules! run {
    ($cells:ident, $func:ident) => {{
        let mut poss: [(usize, u16); 9] = [(0, 0); 9];
        let mut changed = false;
        for major in 0..9 {
            let mut count = 0;
            for minor in 0..9 {
                let i = $func(major, minor);
                let cell = $cells[i];
                if cell.value == -1 {
                    poss[count] = (i, cell.possible);
                    count += 1;
                }
            }
            if count < 3 {
                break;
            }
            'outer: for a in 0..count - 1 {
                for b in a + 1..count {
                    let (_, a_poss) = poss[a];
                    // pair with the same 2-value possibles
                    if a_poss == poss[b].1 && a_poss.count_ones() == 2 {
                        for o in 0..count {
                            let (i, o_poss) = poss[o];
                            if o_poss != 0
                                && o != a && o != b
                                    && (o_poss & a_poss) != 0 {
                                        let mut cell = &mut $cells[i];
                                        cell.possible &= !a_poss;
                                        cell.check_possible();
                                        changed = true;
                                    }
                        }
                        // aleady found the pair, check next outer cell
                        continue 'outer;
                    }
                }
            }
        }
        changed
    }}
}

pub fn rows(cells: &mut [Cell;  81]) -> bool {
    run!(cells, row_loc)
}
pub fn columns(cells: &mut [Cell;  81]) -> bool {
    run!(cells, col_loc)
}
pub fn boxes(cells: &mut [Cell;  81]) -> bool {
    run!(cells, box_loc)
}
