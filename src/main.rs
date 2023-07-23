mod ui;
mod ball;
mod pad;
mod brick;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

const MAP_HIGHT: u32 = 600;
const MAP_WIDTH: u32 = 800;
const BRICK_SIZE: u32 = 40;

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", MAP_WIDTH, MAP_HIGHT)?;
    let mut left = false;
    let mut right = false;
    let mut ball = ball::Ball::new(230, 570, 10, 0.707, -0.707, 3.5);
    let mut pad = pad::Pad::new(200, 580, 70, 10, 8);
    let mut bricks: Vec<brick::Brick> = Vec::new();
    let row_size = (MAP_WIDTH / BRICK_SIZE) as i32;
    let columns_number = 10;
    let bricks_number = row_size * columns_number;

    // create the bricks
    for i in 0..bricks_number {
        bricks.push(brick::Brick {
            x: (i % row_size) * BRICK_SIZE as i32,
            y: (i / row_size) * BRICK_SIZE as i32,
            score: 1,
            size: BRICK_SIZE,
            index: i as usize,
        });
    }

    ui.draw(&ball, &pad, &bricks);
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
        ui.draw(&ball, &pad, &bricks);
        ball.update(&pad, &mut bricks);
        std::thread::sleep(std::time::Duration::from_millis(10));
        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }
    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
