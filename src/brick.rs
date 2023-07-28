use crate::drawable::Drawable;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Brick {
    pub x: i32,
    pub y: i32,
    pub score: u32,
    pub size: u32,
    pub index: usize,
}

impl Brick {
    pub fn collision(&self, x: i32, y: i32) -> bool {
        let mut output = false;
        if self.score > 0 {
            output = x >= self.x
                && x <= self.x + self.size as i32
                && y >= self.y
                && y <= self.y + self.size as i32;
        }
        output
    }

    pub fn generate_bricks(number: i32, brick_size: u32, row_size: i32) -> Vec<Self> {
        let mut bricks: Vec<Self> = Vec::new();
        let row_number: i32 = number / row_size;
        for i in 0..number {
            bricks.push(Self {
                x: (i % row_size) * brick_size as i32,
                y: (i / row_size) * brick_size as i32,
                score: (row_number - (i / row_size)) as u32,
                size: brick_size,
                index: i as usize,
            });
        }
        bricks
    }
}

impl Drawable for Brick {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        if self.score > 0 {
            match self.score {
                1 => canvas.set_draw_color(Color::RGB(39, 174, 96)),
                2 => canvas.set_draw_color(Color::RGB(52, 152, 219)),
                3 => canvas.set_draw_color(Color::RGB(241, 196, 15)),
                4 => canvas.set_draw_color(Color::RGB(155, 89, 182)),
                5 => canvas.set_draw_color(Color::RGB(54, 174, 96)),
                6 => canvas.set_draw_color(Color::RGB(67, 152, 219)),
                7 => canvas.set_draw_color(Color::RGB(255, 196, 15)),
                8 => canvas.set_draw_color(Color::RGB(170, 89, 182)),
                _ => canvas.set_draw_color(Color::RGB(243, 76, 60)),
            }
            canvas
                .fill_rect(Rect::new(
                    self.x + 1,
                    self.y + 1,
                    self.size - 2,
                    self.size - 2,
                ))
                .expect("impossible to draw the ball");
        }
    }
}

impl Drawable for Vec<Brick> {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        for brick in self {
            brick.draw(canvas);
        }
    }
}
