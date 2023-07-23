pub struct Brick {
    pub x: i32,
    pub y: i32,
    pub score: u32,
    pub size: u32,
    pub index: usize,
}

impl Brick {
    pub fn collision(&self, x: i32, y: i32) -> bool {
        let mut output = false;
        if self.score > 0 {
            output = x >= self.x
                && x <= self.x + self.size as i32
                && y >= self.y
                && y <= self.y + self.size as i32;
        }
        output
    }
}
