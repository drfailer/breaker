use sdl2::{video::Window, render::Canvas};

use crate::{ball::{Ball, BallState}, pad::Pad, brick::{Brick, self}, drawable::Drawable};

pub enum BreakerState {
    OK,
    PAUSE,
    VICTORY,
    GAME_OVER,
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

    pub fn pad_left(&mut self) {
        self.pad.left();
    }

    pub fn pad_right(&mut self) {
        self.pad.right();
    }

    pub fn pad_stay(&mut self) {
        self.pad.stay();
    }

    fn update_ok(&mut self) -> BreakerState {
        self.pad.update();
        match self.ball.update(&self.pad, &mut self.bricks) {
            BallState::OK => {},
            _ => {
                self.lifes -= 1;
                // TODO: reset the game
            },
        }
        // output game over if lifes are null
        // TODO: detect victory
        if self.lifes == 0 {
            BreakerState::GAME_OVER
        } else {
            BreakerState::OK
        }
    }

    // NOTE: this may be removed depending on the implementation of the pause
    pub fn update(&mut self) -> BreakerState {
        match self.state {
            BreakerState::OK => self.update_ok(),
            _ => BreakerState::OK,
        }
    }
}

impl Drawable for Breaker {
    fn draw(&self, canvas: &mut Canvas<Window>) {
        // todo: draw ui
        self.ball.draw(canvas);
        self.pad.draw(canvas);
        self.bricks.draw(canvas);
    }
}
