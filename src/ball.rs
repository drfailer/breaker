use crate::{MAP_WIDTH, MAP_HIGHT};
use crate::{pad::Pad, brick::Brick, drawable::Drawable};
use sdl2::video::Window;
use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

pub enum BallState {
    OK,
    FALLEN,
}

/* The direction is a 2D vector where the first point is always at (0, 0).
 * The second point is somewhere on the trigonometric circle around (0, 0). For
 * instance, if the second is at (sqrt(2)/2, sqrt(2)/2), the ball will bounce
 * with a 45° angle when it hits a wall.
 */
pub struct Direction {
    pub x: f32,
    pub y: f32,
}

pub struct Ball {
    x: i32,
    y: i32,
    size: u32,
    direction: Direction,
    speed: f32,
}

fn points_in_bound(x: i32, y: i32, size: u32, brick: &Brick) -> [bool; 4] {
    let i32_size = size as i32;
    let mid = i32_size / 2;
    [
        (x + mid, y),
        (x + i32_size, y + mid),
        (x + mid, y + i32_size),
        (x, y + mid),
    ]
    .map(|(a, b)| brick.collision(a, b))
}

impl Ball {
    pub fn new(x: i32, y: i32, size: u32, v1: f32, v2: f32, speed: f32) -> Self {
        Ball {
            x,
            y,
            size,
            direction: Direction { x: v1, y: v2 },
            speed,
        }
    }

    fn get_next(&self, direction_coord: f32) -> i32 {
        (direction_coord * self.speed) as i32
    }

    pub fn reset(&mut self) {
        self.x = ((MAP_WIDTH - 10) / 2) as i32;
        self.y = (MAP_HIGHT - 30) as i32;
    }

    /* compute the next position of the ball
     * TODO: take the pad in count
     */
    pub fn update(&mut self, pad: &Pad, bricks: &mut Vec<Brick>) -> BallState {
        let map_w = (crate::MAP_WIDTH as i32) - (self.size as i32);
        let map_h = (crate::MAP_HIGHT as i32) - (self.size as i32);
        let pad_xmin = pad.x;
        let pad_xmax = pad.x + (pad.width as i32);
        let pad_y = 580 - self.size as i32;
        let mut brick_index: Option<usize> = None;

        self.x += self.get_next(self.direction.x);
        self.y += self.get_next(self.direction.y);

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
            // early quit since the ball has hit the bottom
            return BallState::FALLEN;
        }

        for brick in bricks.into_iter() {
            let corners = points_in_bound(self.x, self.y, self.size, &brick);
            if corners.contains(&true) {
                brick_index = Some(brick.index);
                if corners[0] != corners[2] {
                    self.direction.y = -self.direction.y;
                }
                if corners[1] != corners[3] {
                    self.direction.x = -self.direction.x;
                }
                break;
            }
        }
        if let Some(index) = brick_index {
            bricks[index].score -= 1;
        }
        return BallState::OK;
    }
}

impl Drawable for Ball {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .fill_rect(Rect::new(self.x, self.y, self.size, self.size))
            .expect("impossible to draw the ball");
    }
}
