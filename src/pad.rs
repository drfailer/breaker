pub struct Pad {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub speed: i32,
}

impl Pad {
    pub fn go_left(&mut self) {
        self.x -= self.speed;
        if self.x < 0 {
            self.x = 0;
        }
    }

    // TODO: change types
    pub fn go_right(&mut self) {
        self.x += self.speed;
        if self.x + (self.width as i32) > crate::MAP_WIDTH as i32 {
            self.x = (crate::MAP_WIDTH as i32) - (self.width as i32);
        }
    }
}
