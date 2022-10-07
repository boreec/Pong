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
        handle_game_events(&mut gs, &mut event_pump, canvas); 
    }
}

fn handle_game_events(gs: &mut GameState, event_pump: &mut EventPump, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    let event = event_pump.wait_event();
    if event.is_user_event() {
        handle_collisions(gs);

        gs.ball.update_position();

        update_cpu_racket(gs);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        draw_game(&gs, canvas);
        canvas.present();

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

fn handle_collisions(gs: &mut GameState){
    if gs.ball.has_collision_with(&gs.racket_1) {
        let cp = gs.ball.collision_point_with(&gs.racket_1);
        if cp == 0 { gs.ball.direction = Direction::EAST; }
        else if cp > 0 { gs.ball.direction = Direction::SOUTHEAST; }
        else {gs.ball.direction = Direction::NORTHEAST; }
    }

    if gs.ball.has_collision_with(&gs.racket_2) {
        let cp = gs.ball.collision_point_with(&gs.racket_2);
        if cp == 0 { gs.ball.direction = Direction::WEST; }
        else if cp > 0 { gs.ball.direction = Direction::SOUTHWEST; }
        else { gs.ball.direction = Direction::NORTHWEST; }
    }

    if gs.ball.has_collision_with_ceiling() {
        if gs.ball.direction == Direction::NORTHWEST {
            gs.ball.direction = Direction::SOUTHWEST;
        }else {
            gs.ball.direction = Direction::SOUTHEAST;
        }
    }

    if gs.ball.has_collision_with_floor() {
        if gs.ball.direction == Direction::SOUTHWEST {
            gs.ball.direction = Direction::NORTHWEST;
        }else {
            gs.ball.direction = Direction::NORTHEAST;
        }
    }
}

fn update_cpu_racket(gs: &mut GameState) {
    if gs.ball.direction == Direction::SOUTH ||
        gs.ball.direction == Direction::SOUTHWEST ||
        gs.ball.direction == Direction::SOUTHEAST {
            gs.racket_2.pos_y += gs.racket_2.speed;
        }
    if gs.ball.direction == Direction::NORTH ||
        gs.ball.direction == Direction::NORTHEAST ||
        gs.ball.direction == Direction::NORTHWEST {
            gs.racket_2.pos_y -= gs.racket_2.speed;
        }
}
