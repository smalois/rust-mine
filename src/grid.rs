extern crate rand;

use self::rand::Rng;
use std::cmp;


static WIDTH: i32 = 10;
static HEIGHT: i32 = 8;
static NUM_MINES: usize = 10;

mod cell {
    pub struct Cell {
        is_mine: bool,
        adjacent_mines: i32,
    }

    impl Cell {
        pub fn new() -> Cell {
            let new_cell : Cell = Cell {
                is_mine : false,
                adjacent_mines : 0,
            };
            new_cell
        }
        pub fn set_mine(&mut self) {
            self.is_mine = true;
        }

        pub fn is_mine(&self) -> bool {
            self.is_mine
        }

        pub fn num_adjacent_mines(&self) -> i32 {
            self.adjacent_mines
        }

        pub fn add_one(&mut self) {
            self.adjacent_mines += 1;
        }
    }
}

pub struct Board {
    cells : Vec<cell::Cell>,
}

impl Board {
    // TODO receive board dimensions and number of mines
   pub fn new() -> Board {
        let mut b : Board = Board {
            cells: Vec::new(),
        };
        b.init_board();
        b.init_mines();
        b.init_cell_values();
        b
    }

    fn init_board(&mut self) {
        for _ in 0..HEIGHT {
            for _ in 0..WIDTH {
                self.cells.push( cell::Cell::new() ); 
            }
        }
    }

    fn init_mines(&mut self) {
        // Generate a vector of mines (tuples) without repeats.
        let mut mines : Vec<(i32, i32)> = Vec::new();
        while mines.len() < NUM_MINES {
            let x_loc : i32 = rand::thread_rng().gen_range(0, WIDTH);
            let y_loc : i32 = rand::thread_rng().gen_range(0, HEIGHT);
            if !mines.contains( &(x_loc, y_loc) ) {
                mines.push( (x_loc, y_loc) );
            }
        }

        // Set the mine values in the board
        for mine in mines {
            self.cells[(mine.0 + (WIDTH * mine.1)) as usize].set_mine();
        }
    }

    fn init_cell_values(&mut self) {
        // Iterate through entire grid
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if self.cells[(col + (WIDTH * row)) as usize].is_mine() {
                    let surrounding_grid_y_bounds = (cmp::max(row - 1, 0), cmp::min(row + 1, HEIGHT - 1));
                    let surrounding_grid_x_bounds = (cmp::max(col - 1, 0), cmp::min(col + 1, WIDTH - 1));
                    // Iterate through the 3x3 cell block surrounding the current cell
                    for y in surrounding_grid_y_bounds.0..surrounding_grid_y_bounds.1 + 1 {
                        for x in surrounding_grid_x_bounds.0..surrounding_grid_x_bounds.1 + 1 {
                            self.cells[(x + y * WIDTH) as usize].add_one();
                        }
                    }
                }
            }
        }
    }

    pub fn print_mines(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_mine() {
                print!("1 ");
            } else {
                print!("0 ");
            }
            if (i + 1) % WIDTH as usize == 0 {
                print!("\n");
            }
        }
    }

    pub fn print_board(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            print!("{} ", cell.num_adjacent_mines());
            if (i + 1) % WIDTH as usize == 0 {
                print!("\n");
            }
        }
    }
}
