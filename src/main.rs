mod ui;
mod ball;
mod pad;
mod brick;
mod drawable;
mod breaker;

extern crate sdl2;

use breaker::Breaker;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

const MAP_HIGHT: u32 = 600;
const MAP_WIDTH: u32 = 800;
const BRICK_SIZE: u32 = 40;

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", MAP_WIDTH, MAP_HIGHT)?;
    let mut ball = ball::Ball::new(230, 570, 10, 0.707, -0.707, 3.5);
    let mut pad = pad::Pad::new(200, 580, 70, 10, 8);
    let row_size = (MAP_WIDTH / BRICK_SIZE) as i32;
    let columns_number = 10;
    let bricks_number = row_size * columns_number;
    let mut bricks = brick::Brick::generate_bricks(bricks_number, BRICK_SIZE, row_size);
    let mut breaker = Breaker::new();

    ui.draw(vec![&ball, &pad, &bricks]);
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
                } => pad.state = pad::PadState::LEFT,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => pad.state = pad::PadState::RIGHT,
                Event::KeyUp {
                    keycode: Option::Some(Keycode::Right | Keycode::Left),
                    ..
                } => pad.state = pad::PadState::STAY,
                _ => {}
            }
        }
        ui.draw(vec![&ball, &pad, &bricks]);
        pad.update();
        if ball.update(&pad, &mut bricks) == false {
            // TODO: have three balls and display a end screen.
            break 'mainloop;
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
