extern crate sdl2;

mod grid;
mod view;
use std::io;

use self::sdl2::keyboard::Keycode;
use self::sdl2::event::Event;
use sdl2::mouse::MouseButton;
use std::time::Duration;
use std::collections::HashSet;

static WIDTH: i32 = 10;
static HEIGHT: i32 = 8;
static NUM_MINES: usize = 10;

fn main() {
    let mut game = grid::Board::new(WIDTH, HEIGHT, NUM_MINES);
    let mut display = view::View::init();

    display.draw_grid(&game);

    let mut prev_buttons = HashSet::new();

    while !game.is_lost() {
        for event in display.event_pump.poll_iter() {
            match event {
                Event::Quit {..} | 
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    game.set_lost();
                },
                _ => {}
            }
        }

        // get a mouse state
        let state = display.event_pump.mouse_state();

        // Create a set of pressed Keys.
        let buttons = state.pressed_mouse_buttons().collect();

        // Get the difference between the new and old sets.
        let new_buttons = &buttons - &prev_buttons;
        let old_buttons = &prev_buttons - &buttons;

        if !old_buttons.is_empty() {
            let (guess_x, guess_y) = 
                display.get_cell_coord(state.x(), state.y(), &game);
            game.reveal(guess_x, guess_y);
            game.print_board();
        }
        prev_buttons = buttons;

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
