use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

use crate::ball::Ball;
use crate::pad::Pad;
use crate::brick::Brick;

pub struct UI {
    sdl: Sdl,
    canvas: Canvas<Window>,
}

impl UI {
    pub fn create_ui(name: &str, width: u32, height: u32) -> Result<UI, String> {
        let sdl = sdl2::init()?;
        let window = sdl
            .video()?
            .window(name, width, height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(UI { sdl, canvas })
    }

    fn draw_ball(&mut self, ball: &Ball) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas
            .fill_rect(Rect::new(ball.x, ball.y, ball.size, ball.size))
            .expect("impossible to draw the ball");
    }

    fn draw_pad(&mut self, pad: &Pad) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas
            .fill_rect(Rect::new(pad.x, pad.y, pad.width, pad.height))
            .expect("impossible to draw the ball");
    }

    fn draw_bricks(&mut self, bricks: &Vec<Brick>) {
        for brick in bricks {
            if brick.score > 0 {
                self.canvas.set_draw_color(Color::RGB(70, 170, 250));
                self.canvas
                    .fill_rect(Rect::new(
                        brick.x + 1,
                        brick.y + 1,
                        brick.size - 2,
                        brick.size - 2,
                    ))
                    .expect("impossible to draw the ball");
            }
        }
    }

    pub fn draw(&mut self, ball: &Ball, pad: &Pad, bricks: &Vec<Brick>) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.draw_ball(ball);
        self.draw_pad(pad);
        self.draw_bricks(bricks);
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl
            .event_pump()
            .expect("error: can't access to events.")
    }
}
