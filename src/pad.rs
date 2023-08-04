use crate::drawable::Drawable;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub enum PadState {
    LEFT,
    RIGHT,
    STAY,
}

pub struct Pad {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub speed: i32,
    pub state: PadState,
}

impl Pad {
    pub fn new(x: i32, y: i32, width: u32, height: u32, speed: i32) -> Self {
        Pad {
            x,
            y,
            width,
            height,
            speed,
            state: PadState::STAY,
        }
    }

    fn go_left(&mut self) {
        self.x -= self.speed;
        if self.x < 0 {
            self.x = 0;
        }
    }

    // TODO: change types
    fn go_right(&mut self) {
        self.x += self.speed;
        if self.x + (self.width as i32) > crate::MAP_WIDTH as i32 {
            self.x = (crate::MAP_WIDTH as i32) - (self.width as i32);
        }
    }

    pub fn left(&mut self) {
        self.state = PadState::LEFT;
    }

    pub fn right(&mut self) {
        self.state = PadState::RIGHT;
    }

    pub fn stay(&mut self) {
        self.state = PadState::STAY;
    }

    pub fn update(&mut self) {
        match self.state {
            PadState::LEFT => self.go_left(),
            PadState::RIGHT => self.go_right(),
            PadState::STAY => {},
        }
    }
}

impl Drawable for Pad {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.width, self.height))
            .expect("impossible to draw the ball");
    }
}
