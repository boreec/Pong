extern crate sdl2;

use crate::entity::*;
use crate::view::*;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;

mod entity;
mod view;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "pong";

const RACKET_WIDTH: u32 = 10;
const RACKET_HEIGHT: u32 = WINDOW_HEIGHT / 10;

const SCREEN_MARGIN: i32 = 10;

const FRAME_DURATION: u32 = 50;

struct FrameEvent;

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

    let mut gs: GameState = initialize_game_state();
    let mut event_pump = context.event_pump().unwrap();
    let ev = context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();

    let timer_subsystem = context.timer().unwrap();
    let _timer = timer_subsystem.add_timer(
        FRAME_DURATION,
        Box::new(|| {
            ev.push_custom_event(FrameEvent).unwrap();
            FRAME_DURATION
        }),
    );
    while !gs.is_game_over {
        handle_game_events(&mut gs, &mut event_pump);

        gs.ball.update_position();

        if gs.ball.has_collision_with(&gs.racket_1) {
            println!("ball: {}", gs.ball.to_string());
            //println!("collision angle {}", gs.ball.get_angle_from_collision_with(&gs.racket_1));
            gs.ball.inverse_direction();
        }

        if gs.ball.has_collision_with(&gs.racket_2) {
            //println!("collision angle {}", gs.ball.get_angle_from_collision_with(&gs.racket_2));
            println!("ball: {}", gs.ball.to_string());
            gs.ball.inverse_direction();
        }

        // draw the game
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        draw_game(&gs, canvas);
        canvas.present();
    }
}

fn handle_game_events(gs: &mut GameState, event_pump: &mut EventPump){
    let event = event_pump.wait_event();
    if event.is_user_event() {
        //
    }else {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                gs.is_game_over = true;
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                gs.racket_1.move_up();
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                gs.racket_1.move_down();
            },
            _ => {}
        }
    }
}
