#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord {
    pub x: u8,
    pub y: u8,
}

#[allow(dead_code)]
impl Coord {
    pub fn set_x(&mut self, x: u8) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u8) {
        self.y = y;
    }

    pub fn add_x(&mut self, x: u8) {
        self.x = self.x + x;
    }

    pub fn add_y(&mut self, y: u8) {
        self.y = self.y + y;
    }

    pub fn sub_x(&mut self, x: u8) {
        self.x = self.x.checked_sub(x).unwrap_or_default();
    }

    pub fn sub_y(&mut self, y: u8) {
        self.y = self.y.checked_sub(y).unwrap_or_default();
    }
}

// fn check_sub(v1: u8, v2: u8) -> u8 {
//     match v1.checked_sub(v2) {
//         Some(v) => v,
//         None => 0,
//     }
// }

pub trait Movement {
    fn left(&mut self);
    fn right(&mut self);
    fn up(&mut self);
    fn down(&mut self);
}
