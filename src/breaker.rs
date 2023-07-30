use crate::{ball::Ball, pad::Pad, brick::{Brick, self}};

enum BreakerState {
    OK,
    END(bool),
}

pub struct Breaker {
    ball: Ball,
    pad: Pad,
    bricks: Vec<Brick>,
    lifes: u8,
    state: BreakerState,
}

impl Breaker {
    pub fn new() -> Self {
        // compute the number of rows and columns
        let row_size = (crate::MAP_WIDTH / crate::BRICK_SIZE) as i32;
        let columns_number = 10;
        let bricks_number = row_size * columns_number;

        Breaker {
            ball: Ball::new(230, 570, 10, 0.707, -0.707, 3.5),
            pad: Pad::new(200, 580, 70, 10, 8),
            bricks: brick::Brick::generate_bricks(bricks_number, crate::BRICK_SIZE, row_size),
            lifes: 3,
            state: BreakerState::OK,
        }
    }

    pub fn update(&mut self) -> BreakerState {
        BreakerState::OK
    }
}
