use sdl2::{render::Canvas, video::Window};

use crate::{
    ball::{Ball, BallState},
    brick::{self, Brick},
    drawable::Drawable,
    pad::Pad,
    MAP_HIGHT, MAP_WIDTH, ROW_NUMBER,
};

pub enum BreakerState {
    OK,
    PAUSE,
    START,
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
    /*------------------------------------------------------------------------------------------*/
    /* constructor */

    pub fn new() -> Self {
        // compute the number of rows and columns
        let row_size = (crate::MAP_WIDTH / crate::BRICK_SIZE) as i32;
        let bricks_number = row_size * ROW_NUMBER as i32;

        Breaker {
            ball: Ball::new(
                ((MAP_WIDTH - 10) / 2) as i32,
                (MAP_HIGHT - 30) as i32,
                10,
                0.707,
                -0.707,
                3.5,
            ),
            pad: Pad::new(
                ((MAP_WIDTH - 70) / 2) as i32,
                (MAP_HIGHT - 20) as i32,
                70,
                10,
                8,
            ),
            bricks: brick::Brick::generate_bricks(bricks_number, crate::BRICK_SIZE, row_size),
            lifes: 3,
            state: BreakerState::START,
        }
    }

    /*------------------------------------------------------------------------------------------*/
    /* accessors */

    pub fn get_lifes(&self) -> u8 {
        self.lifes
    }

    /*------------------------------------------------------------------------------------------*/
    /* pad methodes */

    pub fn pad_left(&mut self) {
        self.pad.left();
    }

    pub fn pad_right(&mut self) {
        self.pad.right();
    }

    pub fn pad_stay(&mut self) {
        self.pad.stay();
    }

    /*------------------------------------------------------------------------------------------*/
    pub fn start(&mut self) {
        match self.state {
            BreakerState::START => self.state = BreakerState::OK,
            _ => {}
        }
    }

    /*------------------------------------------------------------------------------------------*/
    fn update_ok(&mut self) -> BreakerState {
        self.pad.update();
        match self.ball.update(&self.pad, &mut self.bricks) {
            BallState::OK => {}
            BallState::FALLEN => {
                self.lifes -= 1;
                self.pad.reset();
                self.ball.reset();
                self.state = BreakerState::START;
            }
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
