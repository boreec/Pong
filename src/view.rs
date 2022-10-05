use crate::sdl2::gfx::primitives::DrawRenderer;

use crate::entity::GameState;
use crate::entity::Ball;
use crate::entity::Racket;

use sdl2::rect::Rect;

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

pub fn draw_game(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    draw_ball(&gs.ball, canvas);
    draw_racket(&gs.racket_1, canvas);
    draw_racket(&gs.racket_2, canvas);
}
