use sdl2::video::Window;
use sdl2::render::Canvas;

pub trait Drawable {
    fn draw(&self, canvas: &mut Canvas<Window>);
}
