extern crate sdl2;
use crate::sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_TITLE: &str = "pong";

struct Ball {
    pos_x: i32,
    pos_y: i32,
    radius: i32,
    color: sdl2::pixels::Color,
}

struct Racket {
    pos_x: i32,
    pos_y: i32,
    height: u32,
    width: u32,
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
    let r_1: Racket = initialize_racket(10, WINDOW_WIDTH as i32/ 2);

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

        draw_ball(&ball, canvas);
        draw_racket(&r_1, canvas);
        canvas.present();
    }
}

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

fn initialize_racket(x: i32, y:i32) -> Racket {
    return Racket {
        pos_x: x,
        pos_y: y,
        height: WINDOW_HEIGHT / 10,
        width: 10,
        color: Color::WHITE,
    };
}

fn initialize_ball() -> Ball {
    return Ball {
        pos_x: WINDOW_WIDTH as i32/ 2,
        pos_y: WINDOW_HEIGHT as i32 / 2,
        radius: 10,
        color: Color::RGB(255, 140, 0),
    };
}
