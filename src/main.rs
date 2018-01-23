extern crate rand;

use rand::Rng;
use std::cmp;

struct Cell {
    x_coord: i32,
    y_coord: i32,
    is_mine: bool,
    adjacent_mines: i32,
}

impl Cell {
    fn set_mine(&mut self) {
        self.is_mine = true;
    }

    fn add_one(&mut self) {
        self.adjacent_mines += 1;
    }
}

struct Board {
}

impl Board {

}

static WIDTH: i32 = 10;
static HEIGHT: i32 = 8;
static NUM_MINES: usize = 10;

fn init_board(board : &mut Vec<Cell>) {
    for i in 0..(WIDTH * HEIGHT) {
        board.push( Cell { x_coord: i % WIDTH, y_coord: i % HEIGHT, is_mine: false, adjacent_mines: 0} );
    }
}

fn init_mines(board : &mut Vec<Cell>) {
    // Generate a vector of mines (tuples) without repeats.
    let mut mines : Vec<(i32, i32)> = Vec::new();
    while mines.len() <= NUM_MINES {
        let x_loc : i32 = rand::thread_rng().gen_range(0, WIDTH);
        let y_loc : i32 = rand::thread_rng().gen_range(0, HEIGHT);
        if !mines.contains( &(x_loc, y_loc) ) {
            mines.push( (x_loc, y_loc) );
        }
    }

    // Set the mine values in the board
    for mine in mines {
        board[(mine.0 * mine.1) as usize].set_mine();
    }
}

fn init_cell_values(board : &mut Vec<Cell>) {
    for cell in board.iter() {
        if cell.is_mine {
            let nine_grid_x_bounds = (cmp::max(cell.x_coord - 1, 0), cmp::min(cell.x_coord + 1, WIDTH));
            let nine_grid_y_bounds = (cmp::max(cell.y_coord - 1, 0), cmp::min(cell.y_coord + 1, HEIGHT));
            for y in nine_grid_y_bounds.0..nine_grid_y_bounds.1 {
                for x in nine_grid_x_bounds.0..nine_grid_x_bounds.1 {
                    board[(x * y) as usize].add_one();
              }
            }
        }
    }
}

fn print_board(board : &mut Vec<Cell>) {
    for i in board {
        print!("{} ", i.is_mine);
        if i.x_coord == WIDTH - 1 {
            print!("\n");
        }
    }
}

fn main() {
    // vec2d exist
    let mut board : Vec<Cell> = Vec::new();
    init_board(&mut board);
    init_mines(&mut board);
    print_board(&mut board);
}
