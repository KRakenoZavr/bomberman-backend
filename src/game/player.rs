use crate::game::{
    movement::{Coord, Movement},
    power::PowerUp,
};

use super::key_handler::Key;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Player {
    pub id: u8,
    pub lives: u8,
    pub power_ups: Vec<PowerUp>,
    pub speed: u8,
    pub bombs_count: u8,
    pub coord: Coord,
    pub last_action: Option<Key>,
}

impl Player {
    pub fn new(id: u8, lives: u8, speed: u8, bombs_count: u8, coord: Coord) -> Player {
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
}

impl Movement for Player {
    fn up(&mut self) {
        self.coord.sub_y(self.speed);
    }

    fn left(&mut self) {
        self.coord.sub_x(self.speed);
    }

    fn down(&mut self) {
        self.coord.add_x(self.speed);
    }

    fn right(&mut self) {
        self.coord.add_y(self.speed);
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn simple_movement() {
//        let speed = 1;
//        let mut player = Player::new(0, 1, speed, 1, Coord { x: 0, y: 0 });
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
//        let mut player = Player::new(0, 1, speed, 1, Coord { x: 0, y: 0 });
//        player.left();
//        player.up();
//
//        assert_eq!(player.coord.x, 0);
//        assert_eq!(player.coord.y, 0);
//    }
//}
