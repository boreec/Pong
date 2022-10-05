use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;
use crate::SCREEN_MARGIN;
use crate::RACKET_HEIGHT;
use crate::RACKET_WIDTH;

use sdl2::pixels::Color;
use vector2d::Vector2D;

pub struct GameState {
    pub ball: Ball,
    pub racket_1: Racket,
    pub racket_2: Racket,
    pub is_game_over: bool,
}

pub struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub radius: i32,
    pub direction: Vector2D<i32>,
    pub speed: i32, // the number of pixels/frame
    pub color: sdl2::pixels::Color,
}

impl Ball {
    pub fn update_position(&mut self) {
        self.pos_x = self.pos_x + self.speed * self.direction.x;
        self.pos_y = self.pos_y + self.speed * self.direction.y;
    }
}

pub struct Racket {
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: u32,
    pub width: u32,
    pub color: sdl2::pixels::Color,
}

pub fn initialize_game_state() -> GameState {
    return GameState {
        ball: initialize_ball(
            WINDOW_WIDTH as i32 / 2,
            WINDOW_HEIGHT as i32 / 2,
            10,
            Vector2D::new(1, 0),
            10,
            Color::RGB(255,140,0),
        ),
        racket_1: initialize_racket(
            SCREEN_MARGIN,
            (WINDOW_HEIGHT / 2  - RACKET_HEIGHT / 2) as i32,
            RACKET_HEIGHT,
            RACKET_WIDTH,
            Color::WHITE
        ),
        racket_2: initialize_racket(
            WINDOW_WIDTH as i32 - SCREEN_MARGIN - RACKET_WIDTH as i32,
            (WINDOW_HEIGHT / 2  - RACKET_HEIGHT / 2) as i32,
            RACKET_HEIGHT,
            RACKET_WIDTH,
            Color::WHITE
        ),
        is_game_over: false,
    };
}

pub fn initialize_racket(x: i32, y: i32, h: u32, w: u32, c: Color) -> Racket {
    return Racket {
        pos_x: x,
        pos_y: y,
        height: h,
        width: w,
        color: c,
    };
}

pub fn initialize_ball(x: i32, y: i32, r: i32, d: Vector2D<i32>, s: i32, c: Color) -> Ball {
    return Ball {
        pos_x: x,
        pos_y: y,
        radius: r,
        direction: d,
        speed: s,
        color: c,
    };
}

