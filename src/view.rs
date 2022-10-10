use crate::sdl2::gfx::primitives::DrawRenderer;

use crate::entity::Ball;
use crate::entity::GameState;
use crate::entity::Racket;
use crate::WINDOW_HEIGHT;
use crate::WINDOW_WIDTH;

use std::path::Path;

use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;

const HALFWAY_LINE_DASHES: i32 = 20;

fn draw_racket(racket: &Racket, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(racket.color);
    let rectangle: Rect = Rect::new(racket.pos_x, racket.pos_y, racket.width, racket.height);
    let r = canvas.fill_rect(rectangle);
    if r.is_err() {
        panic!("racket could not be drawn");
    }
}

fn draw_ball(ball: &Ball, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(ball.color);
    let r = canvas.filled_circle(
        ball.pos_x as i16,
        ball.pos_y as i16,
        ball.radius as i16,
        ball.color,
    );
    if r.is_err() {
        panic!("ball could not be drawn!");
    }
}

pub fn draw_halfway_line(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::WHITE);
    let middle_x = (WINDOW_WIDTH / 2) as i32 - 2;
    let dash_length = WINDOW_HEIGHT as i32 / (HALFWAY_LINE_DASHES * 2);
    let margin_top = dash_length / 2;
    for i in 0..(HALFWAY_LINE_DASHES * 2) {
        if i % 2 == 0 {
            let p1 = Point::new(middle_x, margin_top + i * dash_length);
            let p2 = Point::new(middle_x, margin_top + i * dash_length + dash_length);
            let r = canvas.draw_line(p1, p2);
            if r.is_err() {
                panic!(
                    "Attempted to draw halfway line dash from {} to {}.",
                    margin_top + i * dash_length,
                    margin_top + i * dash_length + dash_length
                );
            }
        }
    }
}

pub fn draw_score(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
    let texture_creator = canvas.texture_creator();

    let schluber_font_path: &Path = Path::new("font/schluber/Schluber.ttf");

    let result_load_font = ttf_context.load_font(schluber_font_path, 128);
    if result_load_font.is_err() {
        panic!("Problem loading font {}", schluber_font_path.display());
    }

    let font = result_load_font.unwrap();
    let surface_p1 = font
        .render(&format!("{}", gs.score_p1))
        .blended(Color::WHITE)
        .unwrap();
    let surface_p2 = font
        .render(&format!("{}", gs.score_p2))
        .blended(Color::WHITE)
        .unwrap();

    let rect_width: u32 = WINDOW_WIDTH / 12;
    let rect_height: u32 = WINDOW_HEIGHT / 10;
    let font_rect_p1 = Rect::new(
        (WINDOW_WIDTH / 4 - rect_width / 2) as i32,
        rect_height as i32,
        rect_width,
        rect_height,
    );
    let font_rect_p2 = Rect::new(
        (WINDOW_WIDTH * 3 / 4 - rect_width / 2) as i32,
        rect_height as i32,
        rect_width,
        rect_height,
    );
    let texture_p1 = texture_creator
        .create_texture_from_surface(&surface_p1)
        .unwrap();
    let texture_p2 = texture_creator
        .create_texture_from_surface(&surface_p2)
        .unwrap();

    canvas.copy(&texture_p1, None, font_rect_p1).unwrap();
    canvas.copy(&texture_p2, None, font_rect_p2).unwrap();
}

pub fn draw_game(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    draw_halfway_line(canvas);
    draw_score(&gs, canvas);
    draw_ball(&gs.ball, canvas);
    draw_racket(&gs.racket_1, canvas);
    draw_racket(&gs.racket_2, canvas);
}
