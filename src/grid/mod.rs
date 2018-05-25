extern crate rand;

use self::rand::Rng;
use std::cmp;

mod cell;

pub struct Board {
    pub width: i32,
    pub height: i32,
    num_mines: usize,
    lost: bool,
    cells : Vec<cell::Cell>,
}

impl Board {
   pub fn new(width: i32, height: i32, num_mines: usize) -> Board {
        let mut b : Board = Board {
            width: width,
            height: height,
            num_mines: num_mines,
            lost: false,
            cells: Vec::new(),
        };
        b.init_board();
        b.init_mines();
        b.init_cell_values();
        b
    }

    pub fn reveal(&mut self, row: i32, col: i32) {
        self.cells[(col + (self.width * row)) as usize].reveal();

        // Found a safe square. Reveal all surrounding.
        if self.cells[(col + (self.width * row)) as usize].num_adjacent_mines() == 0 {
            let surrounding_grid_y_bounds = 
                (cmp::max(row - 1, 0), cmp::min(row + 1, self.height - 1));
            let surrounding_grid_x_bounds = 
                (cmp::max(col - 1, 0), cmp::min(col + 1, self.width - 1));
            for y in surrounding_grid_y_bounds.0..surrounding_grid_y_bounds.1 + 1 {
                for x in surrounding_grid_x_bounds.0..surrounding_grid_x_bounds.1 + 1 {
                    self.cells[(x + y * self.width) as usize].reveal();
                }
            }
        }

        // Found a bomb.
        if self.cells[(col + (self.width * row)) as usize].is_mine() {
            self.lost = true;
        }
    }

    fn init_board(&mut self) {
        for _ in 0..self.height {
            for _ in 0..self.width {
                self.cells.push( cell::Cell::new() ); 
            }
        }
    }

    fn init_mines(&mut self) {
        // Generate a vector of mines (tuples) without repeats. This approach is not ideal.
        let mut mines : Vec<(i32, i32)> = Vec::new();
        while mines.len() < self.num_mines {
            let x_loc : i32 = rand::thread_rng().gen_range(0, self.width);
            let y_loc : i32 = rand::thread_rng().gen_range(0, self.height);
            // Only add if non-mine cell.
            if !mines.contains( &(x_loc, y_loc) ) {
                mines.push( (x_loc, y_loc) );
            }
        }

        // Set the mine values in the board
        for mine in mines {
            self.cells[(mine.0 + (self.width * mine.1)) as usize].set_mine();
        }
    }

    fn init_cell_values(&mut self) {
        // Iterate through entire grid
        for row in 0..self.height {
            for col in 0..self.width {
                if self.cells[(col + (self.width * row)) as usize].is_mine() {
                    let surrounding_grid_y_bounds = 
                        (cmp::max(row - 1, 0), cmp::min(row + 1, self.height - 1));
                    let surrounding_grid_x_bounds = 
                        (cmp::max(col - 1, 0), cmp::min(col + 1, self.width - 1));
                    // Iterate through the 3x3 cell block surrounding the current cell
                    for y in surrounding_grid_y_bounds.0..surrounding_grid_y_bounds.1 + 1 {
                        for x in surrounding_grid_x_bounds.0..surrounding_grid_x_bounds.1 + 1 {
                            self.cells[(x + y * self.width) as usize].add_one();
                        }
                    }
                }
            }
        }
    }

    pub fn is_lost(&self) -> bool {
        self.lost
    }

    // This is temporary
    pub fn set_lost(&mut self) {
        self.lost = true;
    }

    pub fn print_board(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.is_revealed() {
                if cell.is_mine() {
                    print!("B");
                } else {
                    print!("{}", cell.num_adjacent_mines());
                }
            } else {
                print!("X");
            }
            if (i + 1) % self.width as usize == 0 {
                print!("\n");
            }
        }
    }
}
