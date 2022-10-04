extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "pong";

struct Ball {
    pos_x: i32,
    pos_y: i32,
    radius: i32,
    color: sdl2::pixels::Color,
}

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

    let ball: Ball = initialize_ball();

    let mut event_pump = context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
    }
}

fn initialize_ball() -> Ball {
    return Ball {
        pos_x: WINDOW_WIDTH as i32/ 2,
        pos_y: WINDOW_HEIGHT as i32 / 2,
        radius: 10,
        color: Color::WHITE,
    };
}
