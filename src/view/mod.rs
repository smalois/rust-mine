extern crate sdl2;

use self::sdl2::pixels::Color;
use self::sdl2::rect::Rect;
use view::sdl2::EventPump;

use grid;

static WINDOW_WIDTH: i32 = 800;
static WINDOW_HEIGHT: i32 = 600;

pub struct View { 
    pub event_pump: EventPump,
    side_margin_width: i32,
    top_margin_height: i32,
    foot_margin_height: i32,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl View {
    pub fn init() -> View {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("rust-sdl2 demo: Video", WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();

        let mut v : View = View {
            event_pump: event_pump,
            side_margin_width: 0,
            top_margin_height: 0,
            foot_margin_height: 0,
            canvas: canvas,
        };
        v
    }
    
    pub fn draw_grid(&mut self, board: &grid::Board) {
        self.canvas.set_draw_color(Color::RGB(0 ,255, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255 , 0, 0));
        let cell_w : i32 = (WINDOW_WIDTH - (2*self.side_margin_width)) / board.width;
        let cell_h : i32 = 
            (WINDOW_HEIGHT - (self.foot_margin_height + self.top_margin_height))
            / board.height;
        println!("Cell w {:?}", cell_w);
        println!("Cell h {:?}", cell_h);
        for x in 0..board.width {
            for y in 0..board.height {
                self.canvas.fill_rect(Rect::new(
                    /* x */ (self.side_margin_width + (x * cell_w)),
                    /* y */ (self.top_margin_height + (y * cell_h)), 
                    /* w */ (cell_w - 5) as u32, 
                    /* h */ (cell_h - 5) as u32));
            }
        }
        self.canvas.present();
    }

    pub fn get_cell_coord(&mut self, x_coord: i32, y_coord: i32, board: &grid::Board) -> 
        (i32, i32) {
        let cell_w : i32 = (WINDOW_WIDTH - (2*self.side_margin_width)) / board.width;
        let cell_h : i32 = 
            (WINDOW_HEIGHT - (self.foot_margin_height + self.top_margin_height))
            / board.height;
        let x = (x_coord - self.side_margin_width) / cell_w;
        let y = (y_coord - self.top_margin_height) / cell_h;
        (x, y)        
    }
}
