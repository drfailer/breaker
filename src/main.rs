mod ui;

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use ui::UI;

fn run() -> Result<(), String> {
    let mut ui = UI::create_ui("breaker", 800, 600)?;
    ui.draw();

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
                } => break 'mainloop,
                Event::KeyDown {
                    keycode: Option::Some(Keycode::Right),
                    ..
                } => break 'mainloop,
                _ => {}
            }
        }
        ui.draw();
    }
    Ok(())
}

fn main() -> Result<(), String> {
    run()?;
    Ok(())
}
