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
        /*- Create a clone of all cells, to prevent a bug where one
            cell can be modified multiple times if moved downwards -*/
        let mut _grid = grid.clone();

        /*- Iterate -*/
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let this_cell = grid.get(x, y).unwrap_or(&Cell::Dead);
                match this_cell {
                    Cell::Predator => {

                        /*- If predator has found person -*/
                        if let Some(move_to) = Grid::prey_jump(&_grid, (x, y)) {
                            _grid._move(Cell::Predator, (x, y), move_to)
                        }

                        /*- Else move to random spot around -*/
                        else {
                            let move_to = _grid._move_random(&mut rng, x, y);
                            _grid._move(Cell::Predator, (x, y), move_to)
                        }
                    },
                    Cell::Female | Cell::Male => {
                        /*- If cell can reproduce -*/
                        if let Some(move_to) = Grid::can_reproduce(&_grid, (x, y)) {
                            _grid._move(this_cell.clone(), (x, y), move_to)
                        }

                        /*- Move to random spot around -*/
                        else {
                            let move_to = _grid._move_random(&mut rng, x, y);
                            _grid._move(this_cell.clone(), (x, y), move_to)
                        }
                    },
                    _ => ()
                }
            }
        }

        /*- Set grid to new grid -*/
        grid = _grid;

        std::thread::sleep(std::time::Duration::from_millis(100));

        /*- Display -*/
        grid.display();
    }
}
