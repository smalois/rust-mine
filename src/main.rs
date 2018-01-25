mod grid;

fn main() {
    // vec2d exists
    let board : grid::Board =  grid::Board::new();
    board.print_board();
    board.print_mines();
}
