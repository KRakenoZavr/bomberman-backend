use std::ops::{Add, Sub};

use crate::game::power::PowerUp;

pub trait TPoint<T: Sized + Add + Sub> {
    fn get_x(&self) -> T;
    fn get_y(&self) -> T;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[allow(dead_code)]
impl Point {
    pub fn set_x(&mut self, x: u16) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: u16) {
        self.y = y;
    }

    pub fn add_x(&mut self, x: u16) {
        self.x += x;
    }

    pub fn add_y(&mut self, y: u16) {
        self.y += y;
    }

    pub fn sub_x(&mut self, x: u16) {
        self.x = self.x.checked_sub(x).unwrap_or_default();
    }

    pub fn sub_y(&mut self, y: u16) {
        self.y = self.y.checked_sub(y).unwrap_or_default();
    }
}

impl TPoint<u16> for Point {
    fn get_x(&self) -> u16 {
        self.x
    }

    fn get_y(&self) -> u16 {
        self.y
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BombType {
    Standard,
    Flame,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapItem {
    Bomb(BombType),
    Empty,
    Player(u8),
    Wall,
    WallTwo,
    PowerUp(PowerUp),
}

// TODO pass MapItem as T
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Map(pub Vec<Vec<MapItem>>);

pub const DEFAULT_POWER_TIME: u8 = 20;
pub const DEFAULT_ADD_SPEED: u8 = 2;
pub const NUM_TO_MAP_ITEM: [MapItem; 8] = [
    MapItem::Bomb(BombType::Standard),
    MapItem::Empty,
    MapItem::Wall,
    MapItem::WallTwo,
    MapItem::PowerUp(PowerUp::OneUp),
    MapItem::PowerUp(PowerUp::Bomb(DEFAULT_POWER_TIME)),
    MapItem::PowerUp(PowerUp::Flame(DEFAULT_POWER_TIME)),
    MapItem::PowerUp(PowerUp::Speed(DEFAULT_POWER_TIME, DEFAULT_ADD_SPEED)),
];

#[allow(dead_code)]
impl Map {
    pub fn new(map: Vec<Vec<MapItem>>) -> Self {
        Self(map)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0.first().unwrap().len()
    }

    // TODO move to engine
    // pub fn change_player_pos(&mut self, player: &mut Player, cmd: Key) {
    //     //let new_point = self.at_mut(coord.x, coord.y).unwrap();
    //     //let old_pos = self.at_mut(player.coord.x, player.coord.y).unwrap();

    //     let old_pos = Point {
    //         x: player.coord.x,
    //         y: player.coord.y,
    //     };

    //     self.remove_item(old_pos);

    //     match cmd {
    //         Key::Up => player.up(),
    //         Key::Down => player.down(),
    //         Key::Left => player.left(),
    //         Key::Right => player.right(),

    //         _ => (),
    //     }

    //     *self.at_mut_point(player.coord).unwrap() = MapItem::Player(player.id);
    //     //match player.last_action {
    //     //Key::Up =>
    //     //}
    //     // * new_point = MapItem::Player(player_id);
    // }

    pub fn at_point(&self, c: &Point) -> Option<&MapItem> {
        self.at(c.x, c.y)
    }

    pub fn at(&self, x: u16, y: u16) -> Option<&MapItem> {
        self.0.get(x as usize)?.get(y as usize)
    }

    fn at_mut_point(&mut self, c: &Point) -> Option<&mut MapItem> {
        self.at_mut(c.x, c.y)
    }

    fn at_mut(&mut self, x: u16, y: u16) -> Option<&mut MapItem> {
        if let Some(line) = self.0.get_mut(x as usize) {
            return line.get_mut(y as usize);
        }

        None
    }

    pub fn set_item(&mut self, coord: &Point, item: MapItem) {
        *self.at_mut_point(coord).unwrap() = item
    }

    // TODO
    // return RESULT
    pub fn remove_item(&mut self, coord: &Point) {
        if let Some(line) = self.0.get_mut(coord.x as usize) {
            if let Some(v) = line.get_mut(coord.y as usize) {
                *v = MapItem::Empty
            }
        }
    }

    pub fn from_arr(map: Vec<Vec<i32>>) -> Self {
        Map(map
            .iter()
            .map(|r| {
                r.iter()
                    .map(|v| NUM_TO_MAP_ITEM[*v as usize])
                    .collect::<Vec<MapItem>>()
            })
            .collect::<Vec<Vec<MapItem>>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_arr() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let m = Map::from_arr(v);

        let v2 = vec![vec![
            MapItem::Bomb(BombType::Standard),
            MapItem::Empty,
            MapItem::Wall,
            MapItem::WallTwo,
            MapItem::PowerUp(PowerUp::OneUp),
            MapItem::PowerUp(PowerUp::Bomb(DEFAULT_POWER_TIME)),
            MapItem::PowerUp(PowerUp::Flame(DEFAULT_POWER_TIME)),
            MapItem::PowerUp(PowerUp::Speed(DEFAULT_POWER_TIME, DEFAULT_ADD_SPEED)),
        ]];
        let m2 = Map::new(v2);

        assert!(m.eq(&m2));
    }

    #[test]
    fn rows() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let m = Map::from_arr(v);

        let rows_len = m.rows();

        assert_eq!(rows_len, 1);
    }

    #[test]
    fn cols() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let m = Map::from_arr(v);

        let cols_len = m.cols();

        assert_eq!(cols_len, 8);
    }

    #[test]
    fn at() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let m = Map::from_arr(v);

        let wall_two = m.at(0, 3).unwrap();
        let no_item = m.at(1, 0);

        assert_eq!(*wall_two, MapItem::WallTwo);
        assert!(no_item.is_none());
    }

    #[test]
    fn at_hard() {
        let m = Map::new(vec![vec![MapItem::PowerUp(PowerUp::Speed(10, 10))]]);

        let item = m.at(0, 0).unwrap();

        assert_eq!(*item, MapItem::PowerUp(PowerUp::Speed(10, 10)));
    }

    #[test]
    fn set_item() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let mut m = Map::from_arr(v);
        let coord = Point { x: 0, y: 2 };

        let item = m.at_point(&coord).unwrap();

        assert_eq!(*item, MapItem::Wall);

        m.set_item(&coord, MapItem::WallTwo);

        let item = m.at_point(&coord).unwrap();

        assert_eq!(*item, MapItem::WallTwo);
    }

    #[test]
    fn remove_item() {
        let v = vec![vec![0, 1, 2, 3, 4, 5, 6, 7]];
        let mut m = Map::from_arr(v);
        let coord = Point { x: 0, y: 2 };

        m.remove_item(&coord);

        let item = m.at_point(&coord).unwrap();

        assert_eq!(*item, MapItem::Empty);
    }

    // TODO move to engine
    // #[test]
    // fn change_player_pos_right() {
    //     let v = vec![vec![0, 0, 0, 0, 0]];
    //     let mut m = Map::from_arr(v);

    //     let (id, lives, speed, bombs_count, coord) = (1, 3, 1, 0, Point { x: 0, y: 0 });
    //     let mut p = Player::new(id, lives, speed, bombs_count, coord);

    //     m.set_item(p.coord, MapItem::Player(p.id));

    //     m.change_player_pos(&mut p, Key::Right);

    //     assert_eq!(*m.at(0, 0).unwrap(), MapItem::Empty);
    //     assert_eq!(*m.at(0, 1).unwrap(), MapItem::Player(p.id));
    // }

    // #[test]
    // fn change_player_pos_up() {
    //     let v = vec![vec![0, 0, 0, 0, 0], vec![0, 0]];
    //     let mut m = Map::from_arr(v);

    //     let (id, lives, speed, bombs_count, coord) = (1, 3, 1, 0, Point { x: 0, y: 1 });
    //     let mut p = Player::new(id, lives, speed, bombs_count, coord);

    //     m.set_item(p.coord, MapItem::Player(p.id));

    //     m.change_player_pos(&mut p, Key::Up);

    //     assert_eq!(*m.at(0, 1).unwrap(), MapItem::Empty);
    //     assert_eq!(*m.at(0, 0).unwrap(), MapItem::Player(p.id));
    // }
}
