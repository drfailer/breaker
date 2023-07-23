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

    pub fn generate_bricks(number: i32, brick_size: u32, row_size: i32) -> Vec<Self> {
        let mut bricks: Vec<Self> = Vec::new();
        for i in 0..number {
            bricks.push(Self {
                x: (i % row_size) * brick_size as i32,
                y: (i / row_size) * brick_size as i32,
                score: 1,
                size: brick_size,
                index: i as usize,
            });
        }
        bricks
    }
}
