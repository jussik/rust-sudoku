use ::grid::Grid;
use ::solver::Solver;

use rand;
use rand::{XorShiftRng,Rng};
use rand::distributions::{Range,Sample};

pub struct Generator {
    rng: XorShiftRng
}

static BASE_GRID: [i8; 81] = [
    0,1,2, 3,4,5, 6,7,8,
    3,4,5, 6,7,8, 0,1,2,
    6,7,8, 0,1,2, 3,4,5,

    1,2,3, 4,5,6, 7,8,0,
    4,5,6, 7,8,0, 1,2,3,
    7,8,0, 1,2,3, 4,5,6,

    2,3,4, 5,6,7, 8,0,1,
    5,6,7, 8,0,1, 2,3,4,
    8,0,1, 2,3,4, 5,6,7
];

impl Generator {
    pub fn random() -> Generator {
        Generator {
            rng: rand::random()
        }
    }

    pub fn generate(&mut self) -> Grid {
        let mut vals = self.gen_values();
        let mut range = Range::new(0,81);
        let mut prev = vals;
        loop {
            let i = range.sample(&mut self.rng);
            if vals[i] == -1 {
                continue;
            }
            vals[i] = -1;
            let solver = Solver { allow_guessing: false };
            if !solver.solve_values(&vals) {
                return Grid::load(&prev);
            }
            prev = vals;
        }
    }

    fn gen_values(&mut self) -> [i8; 81] {
        let mut order: [usize; 9] = [0,1,2,3,4,5,6,7,8];

        self.shuffle(&mut order);
        let mut vals: [i8; 81] = [0; 81];
        // shuffle columns within their groups
        for x in 0..9 {
            let xr = order[x] * 9;
            let xo = x * 9;
            for y in 0..9 {
                vals[xo + y] = BASE_GRID[xr + y];
            }
        }

        self.shuffle(&mut order);
        let mut vals2: [i8; 81] = [0; 81];
        // shuffle rows within their groups
        for x in 0..9 {
            let xr = order[x];
            for y in 0..9 {
                let yo = y * 9;
                vals2[x + yo] = vals[xr + yo];
            }
        }
        vals2
    }

    fn shuffle(&mut self, order: &mut [usize; 9]) {
        for i in 0..3 {
            let j = i * 3;
            self.rng.shuffle(&mut order[j..j + 3]);
        }
    }
}
