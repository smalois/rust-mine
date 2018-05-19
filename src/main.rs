mod grid;
mod view;
use std::io;

static WIDTH: i32 = 10;
static HEIGHT: i32 = 8;
static NUM_MINES: usize = 10;

fn main() {
    let mut game = grid::Board::new(WIDTH, HEIGHT, NUM_MINES);
    game.print_board();

    while !game.is_lost() {
        let mut row_guess = String::new();
        let mut col_guess = String::new();
        println!("Row: ");
        io::stdin().read_line(&mut row_guess).expect("Failed to read line");
        let row_guess : i32 = row_guess.trim().parse().expect("Numbers only");
        println!("Col: ");
        io::stdin().read_line(&mut col_guess).expect("Failed to read line");
        let col_guess : i32 = col_guess.trim().parse().expect("Numbers only");

        game.reveal(row_guess, col_guess);
        game.print_board();
    }
}
