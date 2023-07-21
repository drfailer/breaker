mod ui;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

const MAP_HIGHT: u32 = 600;
const MAP_WIDTH: u32 = 800;

pub struct Pad {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub speed: i32,
}

impl Pad {
    pub fn go_left(&mut self) {
        self.x -= self.speed;
        if self.x < 0 {
            self.x = 0;
        }
    }

    // TODO: change types
    pub fn go_right(&mut self) {
        self.x += self.speed;
        if self.x + (self.width as i32) > MAP_WIDTH as i32 {
            self.x = (MAP_WIDTH as i32) - (self.width as i32);
        }
    }
}

// NOTE: it would be better if the values where between 0 and 1
struct Direction {
    pub x: i32,
    pub y: i32,
}

pub struct Ball {
    pub x: i32,
    pub y: i32,
    pub size: u32,
    direction: Direction,
}

impl Ball {
    /* compute the next position of the ball
     * TODO: take the pad in count
     */
    pub fn update(&mut self, pad: &Pad) {
        let map_w = (MAP_WIDTH as i32) - (self.size as i32);
        let map_h = (MAP_HIGHT as i32) - (self.size as i32);
        let pad_xmin = pad.x;
        let pad_xmax = pad.x + (pad.width as i32);
        let pad_y = 580 - self.size as i32;

        self.x += self.direction.x;
        self.y += self.direction.y;

        // update x
        if self.x <= 0 {
            self.x = 0;
            self.direction.x = -self.direction.x;
        } else if self.x >= map_w {
            self.x = map_w;
            self.direction.x = -self.direction.x;
        }

        // update y
        if self.y <= 0 {
            self.y = 0;
            self.direction.y = -self.direction.y;
        } else if self.y >= pad_y && self.x >= pad_xmin && self.x <= pad_xmax {
            self.y = pad_y;
            self.direction.y = -self.direction.y;
        } else if self.y >= map_h {
            self.y = map_h;
            self.direction.y = -self.direction.y;
        }
    }
}

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", MAP_WIDTH, MAP_HIGHT)?;
    let vector = Direction {
        x: 1,
        y: 2,
    };
    let mut ball = Ball {
        x: 200,
        y: 200,
        size: 10,
        direction: vector,
    };
    let mut pad = Pad {
        x: 200,
        y: 580,
        width: 70,
        height: 10,
        speed: 10,
    };
    ui.draw(&ball, &pad);

    'mainloop: loop {
        for event in ui.get_events().poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Option::Some(Keycode::Escape | Keycode::Q),
                    ..
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Left),
                    ..
                } => pad.go_left(),
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => pad.go_right(),
                _ => {}
            }
        }
        ui.draw(&ball, &pad);
        ball.update(&pad);
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
