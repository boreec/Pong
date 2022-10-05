use sdl2::pixels::Color;

pub struct Ball {
    pub pos_x: i32,
    pub pos_y: i32,
    pub radius: i32,
    pub color: sdl2::pixels::Color,
}

pub struct Racket {
    pub pos_x: i32,
    pub pos_y: i32,
    pub height: u32,
    pub width: u32,
    pub color: sdl2::pixels::Color,
}

pub fn initialize_racket(x: i32, y: i32, h: u32, w: u32, c: Color) -> Racket {
    return Racket {
        pos_x: x,
        pos_y: y,
        height: h,
        width: w,
        color: c,
    };
}

pub fn initialize_ball(x: i32, y: i32, r: i32, c: Color) -> Ball {
    return Ball {
        pos_x: x,
        pos_y: y,
        radius: r,
        color: c,
    };
}

