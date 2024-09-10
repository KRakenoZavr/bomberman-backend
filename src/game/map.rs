use crate::game::{movement::Coord, power::PowerUp};

use super::{key_handler::Key, player::Player};

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

#[derive(Debug, PartialEq, Eq)]
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

    pub fn change_player_pos(&mut self, player: &Player, coord: Coord) {
        let new_point = self.at_mut(coord.x, coord.y).unwrap();

        let new_point = self.at_mut(player.coord.x, player.coord.y).unwrap();
        //match player.last_action {
        //Key::Up =>
        //}
        // * new_point = MapItem::Player(player_id);
    }

    pub fn at(&self, x: u8, y: u8) -> Option<&MapItem> {
        self.0.get(x as usize)?.get(y as usize)
    }

    pub fn at_mut(&mut self, x: u8, y: u8) -> Option<&mut MapItem> {
        if let Some(line) = self.0.get_mut(x as usize) {
            return line.get_mut(y as usize);
        }

        None
    }

    // TODO
    // return RESULT
    pub fn remove_item(&mut self, coord: Coord) {
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

        assert_eq!(m.eq(&m2), true)
    }
}
