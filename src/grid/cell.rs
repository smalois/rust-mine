pub struct Cell {
    is_mine: bool,
    revealed: bool,
    adjacent_mines: i32,
}

impl Cell {
    pub fn new() -> Cell {
        let new_cell : Cell = Cell {
            is_mine : false,
            revealed : false,
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

    pub fn is_revealed(&self) -> bool {
        self.revealed
    }

    pub fn num_adjacent_mines(&self) -> i32 {
        self.adjacent_mines
    }

    pub fn add_one(&mut self) {
        self.adjacent_mines += 1;
    }

    pub fn reveal(&mut self) {
        self.revealed = true;
    }
}
