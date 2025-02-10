use crate::game::{map::Point, movement::Movement, power::PowerUp};

use super::{key_handler::Key, movement::NextPos};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub lives: u8,
    pub power_ups: Vec<PowerUp>,
    pub speed: u8,
    pub bombs_count: u8,
    pub coord: Point,
    pub last_action: Option<Key>,
}

impl Player {
    pub fn new(id: u8, lives: u8, speed: u8, bombs_count: u8, coord: Point) -> Player {
        Player {
            id,
            lives,
            speed,
            bombs_count,
            power_ups: vec![],
            coord,
            last_action: None,
        }
    }

    pub fn get_speed(&self) -> u16 {
        self.speed as u16
    }
}

impl Movement<u16> for Player {
    fn get_speed(&self) -> u16 {
        self.speed as u16
    }

    fn up(&mut self) {
        self.coord.sub_y(self.get_speed());
    }

    fn left(&mut self) {
        self.coord.sub_x(self.get_speed());
    }

    fn down(&mut self) {
        self.coord.add_x(self.get_speed());
    }

    fn right(&mut self) {
        self.coord.add_y(self.get_speed());
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn simple_movement() {
//        let speed = 1;
//        let mut player = Player::new(0, 1, speed, 1, Point { x: 0, y: 0 });
//        player.right();
//        player.down();
//
//        assert_eq!(player.coord.x, 1);
//        assert_eq!(player.coord.y, 1);
//    }
//
//    #[test]
//    fn overflow_movement() {
//        let speed = 1;
//        let mut player = Player::new(0, 1, speed, 1, Point { x: 0, y: 0 });
//        player.left();
//        player.up();
//
//        assert_eq!(player.coord.x, 0);
//        assert_eq!(player.coord.y, 0);
//    }
//}
