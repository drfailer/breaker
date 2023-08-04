mod ui;
mod ball;
mod pad;
mod brick;
mod drawable;
mod breaker;

extern crate sdl2;

use ball::BallState;
use breaker::{Breaker, BreakerState};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

const MAP_HIGHT: u32 = 600;
const MAP_WIDTH: u32 = 800;
const BRICK_SIZE: u32 = 40;

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", MAP_WIDTH, MAP_HIGHT)?;
    let mut breaker = Breaker::new();

    ui.draw(vec![&breaker]);
    // TODO: the game should start when the player press space
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
                } => breaker.pad_left(),
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => breaker.pad_right(),
                Event::KeyUp {
                    keycode: Option::Some(Keycode::Right | Keycode::Left),
                    ..
                } => breaker.pad_stay(),
                _ => {}
            }
        }
        ui.draw(vec![&breaker]);
        match breaker.update() {
            BreakerState::GAME_OVER => break 'mainloop,
            _ => {},
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
