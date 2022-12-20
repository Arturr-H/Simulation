/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
)]

/*- Imports -*/
mod grid;
use rand::{self, Rng};
use grid::{ Grid, Cell };

/*- Constants -*/
const GRID_SIZE: usize = 12;

/*- Initialize -*/
fn main() -> () {
    /*- Create grid -*/
    let mut grid = Grid::new(GRID_SIZE, 0.1, 0.1);
    let mut rng = rand::thread_rng();

    loop {
        grid.display();

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match grid.get(x, y).unwrap_or(&Cell::Dead) {
                    Cell::Predator => {

                        /*- If predator has found person -*/
                        if let Some(move_to) = Grid::prey_jump(&grid, (x, y)) {
                            grid._move(Cell::Predator, (x, y), move_to)
                        }

                        /*- Else move to random spot around -*/
                        else {
                            let min_x = x.checked_sub(1).unwrap_or(0);
                            let min_y = y.checked_sub(1).unwrap_or(0);
                            let max_x = x.min(GRID_SIZE - 2);
                            let max_y = y.min(GRID_SIZE - 2);
                            let move_to = (
                                rng.gen_range(min_x..=max_x + 1),
                                rng.gen_range(min_y..=max_y + 1)
                            );
                            grid._move(Cell::Predator, (x, y), move_to)
                        }
                    },
                    _ => ()
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(100));

        /*- Display -*/
        grid.display();
    }
}
