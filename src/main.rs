extern crate glutin_window;     
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate find_folder;

use glutin_window::GlutinWindow as Window;                              // The type that WindowSettings::new() is going to give back
use opengl_graphics::{GlGraphics, OpenGL, GlyphCache};                  // opengl graphics handling
use piston::event_loop::{EventSettings, Events};                        // events
use piston::input::{
    RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,                   // events & args
    PressEvent, Button, Key,
};  
use piston::window::WindowSettings;                                     // the actual window object
use texture::TextureSettings;                                           // for loading the font
use find_folder::Search;                                                // for finding the font file

use rand::Rng;                                                          // for random numbers (for food)
use rand::prelude::ThreadRng;

use std::collections::VecDeque;                                         // For the snake body                                     
use std::ops::Add;                                                      // for custom Point<N> struct arithmetic

const BODY_COLOR:  [f32; 4] = [0.8, 0.0, 0.0, 1.0];
const HEAD_COLOR:  [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const EMPTY_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const FOOD_COLOR:  [f32; 4] = [0.0, 0.8, 0.0, 1.0];
const BG_COLOR:    [f32; 4] = [0.2, 0.2, 0.2, 1.0];

const TILES_W: usize = 17;
const TILES_H: usize = 15;
const TILE_SZ: f64 = 30.0;
const BORDER_PX: f64 = 5.0;
const BORDER_TOP_PX: f64 = 25.0;

#[derive(Eq, PartialEq, Clone, Debug)]
struct Point<N> where N:Add {
    x: N,
    y: N
}

impl<N> Point<N> where N:Add{
    fn new(x: N, y: N) -> Self {
        Point{x, y}
    }
}

impl<N: Add<Output=N>> Add for Point<N>{
    type Output = Self;

    fn add(self, other: Point<N>) -> Point<N> {
        Point{
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

struct SnakeGame {
    size: Point<i32>,
    direction: Point<i32>,

    snake_head: Point<i32>,
    snake_body: VecDeque<Point<i32>>,
    snake_size: u32,

    food_pos: Point<i32>,
    rng: ThreadRng,
    is_alive: bool,
    board: Vec<Vec<[f32; 4]>>,
}

impl SnakeGame {

    fn new(width: i32, height: i32) -> Self {
        let mid_x = width/2;
        let mid_y = height/2;

        let mut snake_body: VecDeque<Point<i32>> = VecDeque::new();
        for cell in 0..2 {
            snake_body.push_back( Point::new(mid_x - (3 + cell), mid_y ) );
        }

        let mut board: Vec<Vec<[f32; 4]>> = Vec::new();
        for _col in 0..TILES_W {
            let mut row: Vec<[f32; 4]> = Vec::new();
            for _row in 0..TILES_H {
                row.push(EMPTY_COLOR);
            } board.push(row);
        }

        SnakeGame {
            size: Point::new(width, height),
            direction: Point::new(1,0),

            snake_head: Point::new(mid_x - 3, mid_y),
            snake_body,
            snake_size: 3,

            food_pos: Point::new(mid_x+4, mid_y),
            rng: rand::thread_rng(),
            is_alive: true,
            board,
        }
    }

    fn spawn_food(&mut self) {

        self.food_pos.x = self.rng.gen_range(0..self.size.x) as i32;
        self.food_pos.y = self.rng.gen_range(0..self.size.y) as i32;

        if self.food_pos == self.snake_head { self.spawn_food(); }
        else {
            let mut in_body: bool = false;          // whether the food's new location is inside the snake

            for cell in self.snake_body.iter() {    // checks this
                if &self.food_pos == cell { in_body = true; break; }
            } if in_body { self.spawn_food() }
        }
    }

    fn update(&mut self) {

        if !self.is_alive { return }

        self.snake_body.push_back(self.snake_head.clone());

        self.snake_head.x += self.direction.x;
        self.snake_head.y += self.direction.y;

        let hx = self.snake_head.x;
        let hy = self.snake_head.y;

        if (hx == -1 || hx == self.size.x) || (hy == -1 || hy == self.size.y) {
            self.is_alive = false; return;
        }

        for cell in self.snake_body.iter() {
            if hx == cell.x && hy == cell.y {
                self.is_alive = false; return;
            }
        }

        if self.snake_head == self.food_pos {
            self.snake_size += 1;
            self.spawn_food();
        }

        if self.snake_body.len() > (self.snake_size - 1) as usize {
            self.snake_body.pop_front();
        }

        let mut board: Vec<Vec<[f32; 4]>> = Vec::new();
        for c in 0..self.size.x {
            let mut col: Vec<[f32; 4]> = Vec::new();

            for r in 0..self.size.y {
                let place = Point::new(c,r);

                if self.snake_head == place { col.push( HEAD_COLOR ); }
                else if self.food_pos == place { col.push( FOOD_COLOR ); }
                else {
                    let mut done: bool = false;
                    for cell in self.snake_body.iter() {
                        if cell == &place {
                            col.push( BODY_COLOR );
                            done=true; break;
                        }
                    } if !done { col.push( EMPTY_COLOR ); }
                }
            } board.push(col);
        }
        self.board = board;
    }
}

struct App {
    gl: GlGraphics,
    game: SnakeGame,
}

impl App {
    fn update(&mut self, _args: &UpdateArgs) {
        self.game.update();
    }

    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*;

        let game_tile = rectangle::square(0.0, 0.0, TILE_SZ);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(BG_COLOR, gl);
            for cl in 0..TILES_W {
                let mut transform = c
                    .transform
                    .trans(BORDER_PX + TILE_SZ * (cl as f64), BORDER_TOP_PX + BORDER_PX);

                let col = self.game.board[cl].clone();
                for rw in 0..TILES_H {
                    
                    let cell_color = col[rw];
                    rectangle(cell_color, game_tile, transform, gl);
                    transform = transform.trans(0.0, TILE_SZ);
                }
            }
            let score_str: String =  format!("Score: {}", self.game.snake_size - 3); 
            let text_trans = c
                .transform
                .trans(6.0, 24.0)
                .zoom(0.25);
            text(
                [1.0,1.0,1.0,1.0],
                100,
                &score_str[..],
                glyphs,
                text_trans,
                gl,
            ).unwrap();

        });
    }

    fn new_game(&mut self) {
        self.game = SnakeGame::new(self.game.size.x, self.game.size.y);
    }
}

fn main() {

    let up: Point<i32> = Point::new(0, -1);
    let down: Point<i32> = Point::new(0, 1);
    let left: Point<i32> = Point::new(-1, 0);
    let right: Point<i32> = Point::new(1, 0);

    let mut last_dir: Point<i32> = right.clone();
    let mut last_pressed_dir: Point<i32> = right.clone();
    let mut input_buffer: VecDeque<Point<i32>> = VecDeque::new();

    const WIN_DIMS: [u32; 2] = [
        (2.0*BORDER_PX + TILE_SZ*TILES_W as f64) as u32,
        (BORDER_TOP_PX + 2.0*BORDER_PX + TILE_SZ*TILES_H as f64) as u32
        ];
    let opengl = OpenGL::V3_2;

    let wx = TILES_W as i32; let wy = TILES_H as i32;
    let mut window: Window = WindowSettings::new("CTSF Rust-Snake", WIN_DIMS)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        game: SnakeGame::new(wx, wy),
        gl: GlGraphics::new(opengl),
    };

    let mut settings = EventSettings::new();
    settings.ups = 5;
    let mut events = Events::new(settings);

    let mut exe_folder = std::env::current_exe().unwrap();
    exe_folder.pop(); // Remove the executable's name, leaving the path to the containing folder
    let resource_path = Search::Parents(2).of(exe_folder).for_folder("Resources").unwrap();

    let ref font = resource_path.join("FiraSans-Regular.ttf");

    let mut glyphs = GlyphCache::new(
        font, 
        (),
        TextureSettings::new()
    ).unwrap();

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &mut glyphs);
        }

        if let Some(args) = e.update_args() {
            if let Some(newdir) = input_buffer.pop_front(){
                // make sure the buffered input isn't invalid
                if ! ( last_dir.clone() + newdir.clone() == Point::new(0,0) ) {
                    last_dir = newdir.clone();
                    app.game.direction = newdir;
                }
            }
            app.update(&args);
        }

        // input handling
        if let Some(button) = e.press_args() {
            let ibl = input_buffer.len();
            if ibl < 2 {
                match button {
                    Button::Keyboard(Key::Up) | Button::Keyboard(Key::W) => {
                        if ( !(last_dir==down) || (ibl==1) ) && !(last_pressed_dir==down) {
                            input_buffer.push_back( up.clone() );
                        }
                    },
                    Button::Keyboard(Key::Down) | Button::Keyboard(Key::S) => {
                        if ( !(last_dir==up) || (ibl==1) ) && !(last_pressed_dir==up) {
                            input_buffer.push_back( down.clone() );
                        }
                    },
                    Button::Keyboard(Key::Left) | Button::Keyboard(Key::A) => {
                        if ( !(last_dir==right) || (ibl==1) ) && !(last_pressed_dir==right) {
                            input_buffer.push_back( left.clone() ); }
                    },
                    Button::Keyboard(Key::Right) | Button::Keyboard(Key::D) => {
                        if ( !(last_dir==left) || (ibl==1) ) && !(last_pressed_dir==left) {
                            input_buffer.push_back( right.clone() );
                        }
                    },
                    Button::Keyboard(Key::R) => {
                        if !(app.game.is_alive) { last_dir = right.clone(); app.new_game(); }
                    }
                    Button::Keyboard(Key::X) => {
                        app.game.is_alive = false;
                    }
                    Button::Keyboard(Key::Q) => {
                        break
                    }
                    _ => {}
                }
                if let Some(last_pressed) = input_buffer.back() {
                    last_pressed_dir = last_pressed.clone();
                }
            }
        }
    }
}
