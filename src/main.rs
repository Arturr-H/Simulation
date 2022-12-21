/*- Global allowings -*/
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
)]

/*- Imports -*/
mod grid;
use rand::{self, Rng};
use grid::{ Grid, Cell, GridConfig };

/*- Constants -*/
const GRID_SIZE: usize = 20;

/*- Initialize -*/
fn main() -> () {
    /*- Create grid -*/
    let mut grid = Grid::new(GRID_SIZE, GridConfig {
        predator_death_chance:0.1,
        predator_reproduce_chance:1.,

        reproduce_chance:0.1,

        spawn_chance:0.3,
        predator_spawn_chance:0.03
    });
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
                        /*- 10% chance to die -*/
                        match rng.gen_bool(grid.config.predator_death_chance) {
                            true => {
                                _grid.set(x, y, Cell::Dead);
                                continue;
                            },
                            false => ()
                        }

                        /*- If predator has found person -*/
                        if let Some(move_to) = Grid::prey_jump(&_grid, (x, y)) {
                            _grid._move(Cell::Predator, (x, y), move_to);

                            /*- 10% chance to reproduce -*/
                            match rng.gen_bool(grid.config.predator_reproduce_chance) {
                                true => _grid.set(x, y, Cell::Predator),
                                false => ()
                            }
                        }

                        /*- Else move to random spot around -*/
                        else {
                            let move_to = _grid._move_random(&mut rng, x, y);
                            _grid._move(Cell::Predator, (x, y), move_to)
                        }
                    },
                    Cell::Female | Cell::Male => {
                        /*- If cell can reproduce -*/
                        if let Some(_) = Grid::can_reproduce(&_grid, (x, y)) {
                            /*- Random 10% chance -*/
                            match rng.gen_bool(grid.config.reproduce_chance) {
                                true => {
                                    /*- Try 10 times -*/
                                    for _ in 0..10 {
                                        let move_to = _grid._move_random(&mut rng, x, y);

                                        let cell = grid.get(move_to.0, move_to.1);
                                        if  cell != Some(&Cell::Dead) { continue; };
                                        
                                        /*- Spawn either female or male at random spot -*/
                                        match rng.gen_bool(0.5) {
                                            true => _grid.set(move_to.0, move_to.1, Cell::Female),
                                            false => _grid.set(move_to.0, move_to.1, Cell::Male)
                                        };

                                        break;
                                    }
                                }
                                false => ()
                            }
                        }

                        /*- Move to random spot around -*/
                        else {
                            let move_to = _grid._move_random(&mut rng, x, y);
                            let cell = grid.get(move_to.0, move_to.1);
                            if  cell == Some(&Cell::Dead) {
                                _grid._move(this_cell.clone(), (x, y), move_to)
                            };
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
