use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;
use crate::SCREEN_MARGIN;

// Racket's general speed in pixels per frame.
const RACKET_SPEED: i32 = 10;
const RACKET_WIDTH: u32 = 10;
const RACKET_HEIGHT: u32 = WINDOW_HEIGHT / 8;

const BALL_SPEED: f64 = 10.0;

use sdl2::pixels::Color;
use vector2d::Vector2D;

pub struct GameState {
    pub ball: Ball,
    pub racket_1: Racket,
    pub racket_2: Racket,
    pub is_game_over: bool,
}

#[derive(Copy, Clone)]
pub struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub radius: i32,
    pub direction: Vector2D<f64>,
    pub speed: f64, // the number of pixels/frame
    pub color: sdl2::pixels::Color,
}

impl ToString for Ball {
    fn to_string(&self) -> String {
        return format!(
            "({},{}), radius: {}, speed: {}, direction: ({},{})",
            self.pos_x,
            self.pos_y,
            self.radius,
            self.speed,
            self.direction.x,
            self.direction.y
        );
    }
}

impl Ball {
    pub fn update_position(&mut self) {
        self.pos_x = self.pos_x +(self.speed * self.direction.x) as i32;
        self.pos_y = self.pos_y + (self.speed * self.direction.y) as i32;
    }

    pub fn inverse_direction(&mut self, angle: f64){
        self.direction.x = self.direction.x * -1.0;
        self.direction.y = self.direction.y * -1.0;
        if angle >= 0.0 {
            let old_dx = self.direction.x;
            let old_dy = self.direction.y;
            self.direction.x = (old_dx * angle.cos() - old_dy * angle.sin()) * -1.0;
            self.direction.y = (old_dx * angle.sin() + old_dx * angle.cos()) * -1.0;
        }
    }

    /*
    * Algorithm based on the answer of e.James on Stackoverflow.
    */
    pub fn has_collision_with(self, racket: &Racket) -> bool {
        let circle_distance_x: u32 = (self.pos_x - racket.pos_x).abs() as u32;
        let circle_distance_y: u32 = (self.pos_y - racket.pos_y).abs() as u32;

        if circle_distance_x > (racket.width / 2 + self.radius as u32) {
            return false;
        }

        if circle_distance_y > (racket.height / 2 + self.radius as u32) {
            return false;
        }

        if circle_distance_x <= racket.width / 2 {
            return true;
        }

        if circle_distance_y <= racket.height / 2 {
            return true;
        }

        let corner_distance_sq =
            (circle_distance_x - racket.width / 2)^2 +
            (circle_distance_y - racket.height / 2)^2;

        return corner_distance_sq <= (self.radius^2) as u32;
    }
    
    pub fn has_collision_with_ceiling(self) -> bool {
        return self.pos_y - self.radius <= 0;
    }
    
    pub fn get_angle_from_collision_with(self, racket: &Racket) -> f64 {

        // distance between the ball and the middle of the racket.
        let d = racket.pos_y + (racket.height / 2) as i32 - self.pos_y;

        return (d as f64 / (racket.height as f64 / 2.0)) * 45.0;
    }
}

pub struct Racket {
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: u32,
    pub width: u32,
    pub speed: i32,
    pub color: sdl2::pixels::Color,
}

impl Racket {
    pub fn move_up(&mut self){
        if self.pos_y > 0 {
            self.pos_y = self.pos_y - self.speed;
        }
    }

    pub fn move_down(&mut self){
        if self.pos_y < (WINDOW_HEIGHT - self.height) as i32 {
            self.pos_y = self.pos_y + self.speed;
        }
    }
}

pub fn initialize_game_state() -> GameState {
    return GameState {
        ball: initialize_ball(
            WINDOW_WIDTH as i32 / 2,
            WINDOW_HEIGHT as i32 / 2,
            10,
            Vector2D::new(1.0, 0.0),
            BALL_SPEED,
            Color::RGB(255,140,0),
        ),
        racket_1: initialize_racket(
            SCREEN_MARGIN,
            (WINDOW_HEIGHT / 2  - RACKET_HEIGHT / 2) as i32,
            RACKET_HEIGHT,
            RACKET_WIDTH,
            RACKET_SPEED,
            Color::WHITE
        ),
        racket_2: initialize_racket(
            WINDOW_WIDTH as i32 - SCREEN_MARGIN - RACKET_WIDTH as i32,
            (WINDOW_HEIGHT / 2  - RACKET_HEIGHT / 2) as i32,
            RACKET_HEIGHT,
            RACKET_WIDTH,
            RACKET_SPEED,
            Color::WHITE
        ),
        is_game_over: false,
    };
}

pub fn initialize_racket(x: i32, y: i32, h: u32, w: u32, s: i32, c: Color) -> Racket {
    return Racket {
        pos_x: x,
        pos_y: y,
        height: h,
        width: w,
        speed: s,
        color: c,
    };
}

pub fn initialize_ball(x: i32, y: i32, r: i32, d: Vector2D<f64>, s: f64, c: Color) -> Ball {
    return Ball {
        pos_x: x,
        pos_y: y,
        radius: r,
        direction: d,
        speed: s,
        color: c,
    };
}

