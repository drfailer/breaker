mod ui;
mod ball;
mod pad;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

const MAP_HIGHT: u32 = 600;
const MAP_WIDTH: u32 = 800;

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", MAP_WIDTH, MAP_HIGHT)?;
    let mut left = false;
    let mut right = false;
    let vector = ball::Direction {
        x: 0.707,
        y: 0.707,
    };
    let mut ball = ball::Ball {
        x: 200,
        y: 200,
        size: 10,
        direction: vector,
        speed: 3.5
    };
    let mut pad = pad::Pad {
        x: 200,
        y: 580,
        width: 70,
        height: 10,
        speed: 8,
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
                } => left = true,
                Event::KeyUp {
                    keycode: Option::Some(Keycode::Left),
                    ..
                } => left = false,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => right = true,
                Event::KeyUp {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => right = false,
                _ => {}
            }
        }
        if left {
            pad.go_left()
        }
        if right {
            pad.go_right()
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
