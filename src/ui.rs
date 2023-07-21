use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{EventPump, Sdl};

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

    fn draw_ball(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        if let Err(_) = self.canvas.fill_rect(Rect::new(200, 200, 10, 10)) {
            println!("impossible to draw the ball");
        }
    }

    fn draw_pad(&mut self) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        if let Err(_) = self.canvas.fill_rect(Rect::new(200, 580, 70, 10)) {
            println!("impossible to draw the ball");
        }
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.draw_ball();
        self.draw_pad();
        self.canvas.present();
    }

    pub fn get_events(&mut self) -> EventPump {
        self.sdl
            .event_pump()
            .expect("error: can't access to events.")
    }
}
