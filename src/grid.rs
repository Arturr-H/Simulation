use std::slice::SliceIndex;

/*- Imports -*/
use rand::{self, Rng};

/*- Main -*/
pub struct Grid {
    /// All cells are stored here in 2d vec
    cells:Vec<Vec<Cell>>,

    /// grid size (square).
    grid_size:usize
}

/*- Cell -*/
#[derive(Debug, PartialEq)]
pub enum Cell {
    Dead = 0,

    // Will pair with eachother
    Male = 1,
    Female = 2,

    // Predator
    Predator = 3
}

/*- Method implementations -*/
impl Grid {
    /// `size` will determine the grid size (square).
    /// `spawn_chance` determines the chance of spawning an
    /// alive cell on each position, which ranges 0.0 - 1.0
    pub fn new(grid_size:usize, spawn_chance:f64, prey_chance:f64) -> Self {
        let mut cells:Vec<Vec<Cell>> = Vec::new();
        let mut rng = rand::thread_rng();

        /*- Iterate -*/
        for y in 0..grid_size {
            let mut inner:Vec<Cell> = Vec::new();

            /*- Create cells -*/
            for x in 0..grid_size {

                /*- If will spawn -*/
                match rng.gen_bool(spawn_chance) {
                    true => {
                        /*- If will be prey -*/
                        match rng.gen_bool(prey_chance) {
                            true => inner.push(Cell::Predator),
                            false => {

                                /*- Female or male -*/
                                match rng.gen_bool(0.5) {
                                    true => inner.push(Cell::Male),
                                    false => inner.push(Cell::Female)
                                }
                            }
                        }
                    },
                    false => inner.push(Cell::Dead)
                }
            };

            /*- Push -*/
            cells.push(inner)
        };

        /*- Return -*/
        Self { cells, grid_size }
    }

    /// Get tile at coordinate
    pub fn get(&self, x:usize, y:usize) -> Option<&Cell> {
        self.cells.get(y)?.get(x)
    }

    /// Set tile at coordinate to any `Cell`
    pub fn set(&mut self, x:usize, y:usize, to:Cell) -> () {
        match self.cells.get_mut(y) {
            Some(row) => {
                match row.get_mut(x) {
                    Some(cell) => {
                        *cell = to
                    },
                    None => ()
                }
            }
            None => ()
        }
    }

    /// Move a tile
    pub fn _move(&mut self, cell:Cell, from:(usize, usize), to:(usize, usize)) -> () {
        if self.get(to.0, to.1) == Some(&Cell::Predator) { return; };

        /*- Remove current -*/
        self.set(from.0, from.1, Cell::Dead);
        self.set(to.0, to.1, cell);
    }

    /// Display the grid to stdout
    pub fn display(&self) -> () {
        println!("\n\n\n");
        for row in &self.cells {
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        Cell::Dead => "⬜️",
                        Cell::Male => "🟦",
                        Cell::Female => "🟥",
                        Cell::Predator => "⬛️"
                    }
                )
            };
            println!()
        }
    }

    /// Get neighbouring tiles and return an
    /// array, containing a tuple of the
    /// neighbours coordinates and their cell
    pub fn get_neighbours(&self, x:usize, y:usize) -> Vec<((usize, usize), &Cell)> {
        let mut end = Vec::new();

        /*- Get bounds -*/
        let top = y.checked_sub(1).unwrap_or(0);
        let left = x.checked_sub(1).unwrap_or(0);
        let right = (x + 2).min(self.grid_size);
        let bottom = (y + 2).min(self.grid_size);

        /*- Iterate -*/
        for _y in top..bottom {
            for _x in left..right {
                if _x == x && _y == y { continue; };

                /*- We unwrap here, I am sure this won't break -*/
                end.push(((_x, _y), self.get(_x, _y).unwrap()))
            };
        };

        end
    }

    /// Neighbour contains will iterate over neighbours
    /// and check if any of these match the input, if so
    /// return their coordinates
    pub fn neighbours_contain(neighbours:&Vec<((usize, usize), &Cell)>, input:Cell) -> Option<(usize, usize)> {
        for n in neighbours {
            if n.1 == &input {
                return Some(n.0)
            }
        };

        None
    }

    /*- Asociated neighbouring functions for diffrent cells -*/
    /// Return coordinates of where prey should jump to
    /// (attack) and return None if there is nothing to attack
    pub fn prey_jump(grid:&Self, prey:(usize, usize)) -> Option<(usize, usize)> {
        /*- Get neighbouring cells -*/
        let neighbours = grid.get_neighbours(prey.0, prey.1);

        /*- Check if has jump location -*/
        if let Some(female) = Self::neighbours_contain(&neighbours, Cell::Female) { Some(female) }
        else if let Some(male) = Self::neighbours_contain(&neighbours, Cell::Male) {Some(male)   }
        else { None }
    }
}