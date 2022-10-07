use crate::sdl2::gfx::primitives::DrawRenderer;

use crate::entity::GameState;
use crate::entity::Ball;
use crate::entity::Racket;
use crate::WINDOW_WIDTH;
use crate::WINDOW_HEIGHT;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

const HALFWAY_LINE_DASHES: i32 = 10;

fn draw_racket(racket: &Racket, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(racket.color);
    let rectangle: Rect = Rect::new(racket.pos_x, racket.pos_y, racket.width, racket.height);
    let r = canvas.fill_rect(rectangle);
    if r.is_err() {
        panic!("racket could not be drawn");
    }
}

fn draw_ball(ball: &Ball, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(ball.color);
    let r = canvas.filled_circle(ball.pos_x as i16, ball.pos_y as i16, ball.radius as i16, ball.color);
    if r.is_err() {
        panic!("ball could not be drawn!");
    }
}

pub fn draw_halfway_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(Color::WHITE);
    let middle_x = (WINDOW_WIDTH / 2) as i32 - 2;
    let dash_length = WINDOW_HEIGHT as i32 / (HALFWAY_LINE_DASHES * 2);
    let margin_top = dash_length / 2;
    for i in 0..(HALFWAY_LINE_DASHES * 2) {
        if i % 2 == 0 {
            let p1 = Point::new(middle_x, margin_top + i * dash_length);
            let p2 = Point::new(middle_x, margin_top + i * dash_length + dash_length);
            canvas.draw_line(p1, p2);
        }
    }
}

pub fn draw_game(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    draw_halfway_line(canvas);
    draw_ball(&gs.ball, canvas);
    draw_racket(&gs.racket_1, canvas);
    draw_racket(&gs.racket_2, canvas);
}
