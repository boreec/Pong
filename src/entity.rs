use crate::SCREEN_MARGIN;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

// Speed of the ball in terms of pixels/frame.
const BALL_INIT_SPEED: i32 = 10;
const BALL_MAX_SPEED: i32 = 15;

// Set the racket speed as 90% of the ball speed.
const RACKET_SPEED: i32 = ((BALL_INIT_SPEED as f32) * 0.9) as i32;
const RACKET_WIDTH: u32 = 10;
const RACKET_HEIGHT: u32 = WINDOW_HEIGHT / 8;

use rand::random;
use sdl2::pixels::Color;

/// Enum variants for every direction the ball can take.
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    NORTHWEST,
    NORTHEAST,
    SOUTHWEST,
    SOUTHEAST,
}

/// Data structure representing the state of the game at any point.
pub struct GameState {
    pub ball: Ball,
    pub racket_1: Racket,
    pub racket_2: Racket,
    pub is_game_over: bool,
    pub is_game_restarted: bool,
    pub score_p1: u32,
    pub score_p2: u32,
}

impl GameState {

    /// Create a new GameState object.
    pub fn new() -> GameState {
        return GameState {
            ball: Ball::new(
                WINDOW_WIDTH as i32 / 2,
                WINDOW_HEIGHT as i32 / 2,
                10,
                Direction::EAST,
                BALL_INIT_SPEED,
                Color::RGB(255, 140, 0),
            ),
            racket_1: Racket::new(
                SCREEN_MARGIN,
                (WINDOW_HEIGHT / 2 - RACKET_HEIGHT / 2) as i32,
                RACKET_HEIGHT,
                RACKET_WIDTH,
                RACKET_SPEED,
                Color::WHITE,
            ),
            racket_2: Racket::new(
                WINDOW_WIDTH as i32 - SCREEN_MARGIN - RACKET_WIDTH as i32,
                (WINDOW_HEIGHT / 2 - RACKET_HEIGHT / 2) as i32,
                RACKET_HEIGHT,
                RACKET_WIDTH,
                RACKET_SPEED,
                Color::WHITE,
            ),
            is_game_over: false,
            is_game_restarted: false,
            score_p1: 0,
            score_p2: 0,
        };
    }

    /// Set the positions and speeds of all objects for a new game.
    pub fn reset_positions(&mut self) {
        self.ball.pos_x = (WINDOW_WIDTH / 2) as i32;
        self.ball.pos_y = (WINDOW_HEIGHT / 2) as i32;
        self.ball.speed = BALL_INIT_SPEED;
        self.racket_1.pos_y = (WINDOW_HEIGHT / 2 - self.racket_1.height / 2) as i32;
        self.racket_2.pos_y = (WINDOW_HEIGHT / 2 - self.racket_2.height / 2) as i32;

        if random::<u8>() % 2 == 0 {
            self.ball.direction = Direction::EAST;
        } else {
            self.ball.direction = Direction::WEST;
        }
    }
}

/// Data structure representing the ball.
#[derive(Copy, Clone)]
pub struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub radius: i32,
    pub direction: Direction,
    pub speed: i32, // the number of pixels/frame
    pub color: sdl2::pixels::Color,
}

impl Ball {

    /// Create a new ball object.
    pub fn new(x: i32, y: i32, r: i32, dir: Direction, speed: i32, color: Color) -> Ball {
        return Ball {
            pos_x: x,
            pos_y: y,
            radius: r,
            direction: dir,
            speed: speed,
            color: color,
        };
    }

    /// Update the ball's coordinates depending on its direction.
    pub fn update_position(&mut self) {
        match self.direction {
            Direction::NORTH => {
                self.pos_y -= self.speed;
            }
            Direction::SOUTH => {
                self.pos_y += self.speed;
            }
            Direction::WEST => {
                self.pos_x -= self.speed;
            }
            Direction::EAST => {
                self.pos_x += self.speed;
            }
            Direction::NORTHEAST => {
                self.pos_x += self.speed;
                self.pos_y -= self.speed;
            }
            Direction::NORTHWEST => {
                self.pos_x -= self.speed;
                self.pos_y -= self.speed;
            }
            Direction::SOUTHEAST => {
                self.pos_x += self.speed;
                self.pos_y += self.speed;
            }
            Direction::SOUTHWEST => {
                self.pos_x -= self.speed;
                self.pos_y += self.speed;
            }
        }
    }

    /// Check if the ball collides with a racket.
    pub fn has_collision_with(self, racket: &Racket) -> bool {
        let y_collision = self.pos_y + self.radius >= racket.pos_y
            && self.pos_y - self.radius <= racket.pos_y + racket.height as i32;
        let x_left_collision = self.pos_x + self.radius >= racket.pos_x
            && self.pos_x + self.radius <= racket.pos_x + racket.width as i32;
        let x_right_collision = self.pos_x - self.radius <= racket.pos_x + racket.width as i32
            && self.pos_x - self.radius >= racket.pos_x;

        return x_left_collision && y_collision || x_right_collision && y_collision;
    }

    /// Give which part of aracket is hit by the ball.
    pub fn collision_point_with(self, racket: &Racket) -> i32 {
        if self.pos_y == racket.pos_y + racket.height as i32 / 2 {
            return 0;
        }
        return if self.pos_y > racket.pos_y + racket.height as i32 / 2 {
            1
        } else {
            -1
        };
    }

    /// Check if the ball intersects the top of the screen.
    pub fn has_collision_with_ceiling(self) -> bool {
        return self.pos_y - self.radius <= 0;
    }

    /// Check if the ball intersects the bottom of the screen.
    pub fn has_collision_with_floor(self) -> bool {
        return self.pos_y + self.radius >= WINDOW_HEIGHT as i32;
    }

    /// Increase the ball's speed by 1 as long as it does not exceed a limit.
    pub fn increase_speed(&mut self) {
        if self.speed < BALL_MAX_SPEED {
            self.speed += 1;
        }
    }
}

/// Data structure representing the rackets (left or right).
pub struct Racket {
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: u32,
    pub width: u32,
    pub speed: i32,
    pub color: sdl2::pixels::Color,
}

impl Racket {

    /// Create a new racket object.
    pub fn new(x: i32, y: i32, h: u32, w: u32, speed: i32, color: Color) -> Racket {
        return Racket {
            pos_x: x,
            pos_y: y,
            height: h,
            width: w,
            speed: speed,
            color: color,
        };
    }

    /// Move the racket upward (on the screen).
    pub fn move_up(&mut self) {
        if self.pos_y > 0 {
            self.pos_y = self.pos_y - self.speed;
        }
    }

    /// Move the racket downward (on the screen).
    pub fn move_down(&mut self) {
        if self.pos_y < (WINDOW_HEIGHT - self.height) as i32 {
            self.pos_y = self.pos_y + self.speed;
        }
    }
}
