use crate::pad::Pad;

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
    pub x: i32,
    pub y: i32,
    pub size: u32,
    pub direction: Direction,
    pub speed: f32,
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

    /* compute the next position of the ball
     * TODO: take the pad in count
     */
    pub fn update(&mut self, pad: &Pad) {
        let map_w = (crate::MAP_WIDTH as i32) - (self.size as i32);
        let map_h = (crate::MAP_HIGHT as i32) - (self.size as i32);
        let pad_xmin = pad.x;
        let pad_xmax = pad.x + (pad.width as i32);
        let pad_y = 580 - self.size as i32;

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
            self.y = map_h;
            self.direction.y = -self.direction.y;
        }
    }
}
