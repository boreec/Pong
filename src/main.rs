extern crate sdl2;

use crate::entity::*;
use crate::view::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod entity;
mod view;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "pong";

const RACKET_WIDTH: u32 = 10;
const RACKET_HEIGHT: u32 = WINDOW_HEIGHT / 10;

const SCREEN_MARGIN: i32 = 10;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    game_loop(&sdl_context, &mut canvas);

}

fn game_loop(context: &sdl2::Sdl,
             canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    let mut gs: GameState = initialize_game_state();

    let mut event_pump = context.event_pump().unwrap();
    while !gs.is_game_over {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    gs.is_game_over = true;
                },
                _ => {}
            }
        }
        gs.ball.update_position();
        draw_ball(&gs.ball, canvas);
        draw_racket(&gs.racket_1, canvas);
        draw_racket(&gs.racket_2, canvas);
        canvas.present();
    }
}
