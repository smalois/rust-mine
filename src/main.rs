mod grid;

static WIDTH: i32 = 10;
static HEIGHT: i32 = 8;
static NUM_MINES: usize = 10;

fn main() {
    // vec2d exists
    let board : grid::Board =  grid::Board::new(WIDTH, HEIGHT, NUM_MINES);
    board.print_board();
    board.print_mines();
}
