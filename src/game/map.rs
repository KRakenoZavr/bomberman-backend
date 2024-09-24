use crate::game::{movement::Coord, power::PowerUp};

use super::{key_handler::Key, movement::Movement, player::Player};

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

    pub fn change_player_pos(&mut self, player: &mut Player, cmd: Key) {
        //let new_point = self.at_mut(coord.x, coord.y).unwrap();
        //let old_pos = self.at_mut(player.coord.x, player.coord.y).unwrap();

        let old_pos = Coord {
            x: player.coord.x,
            y: player.coord.y,
        };

        self.remove_item(old_pos);

        match cmd {
            Key::Up => player.up(),
            Key::Down => player.down(),
            Key::Left => player.left(),
            Key::Right => player.right(),

            _ => (),
        }

        *self.at_mut_point(player.coord).unwrap() = MapItem::Player(player.id);
        //match player.last_action {
        //Key::Up =>
        //}
        // * new_point = MapItem::Player(player_id);
    }

    pub fn at_point(&self, c: Coord) -> Option<&MapItem> {
        self.at(c.x, c.y)
        //self.0.get(c.x as usize)?.get(c.y as usize)
    }

    pub fn at(&self, x: u8, y: u8) -> Option<&MapItem> {
        self.0.get(x as usize)?.get(y as usize)
    }

    pub fn at_mut_point(&mut self, c: Coord) -> Option<&mut MapItem> {
        self.at_mut(c.x, c.y)
    }

    pub fn at_mut(&mut self, x: u8, y: u8) -> Option<&mut MapItem> {
        if let Some(line) = self.0.get_mut(x as usize) {
            return line.get_mut(y as usize);
        }

        None
    }

    pub fn set_item(&mut self, coord: Coord, item: MapItem) {
        *self.at_mut_point(coord).unwrap() = item
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

    #[test]
    fn change_player_pos_right() {
        let v = vec![vec![0, 0, 0, 0, 0]];
        let mut m = Map::from_arr(v);

        let (id, lives, speed, bombs_count, coord) = (1, 3, 1, 0, Coord { x: 0, y: 0 });
        let mut p = Player::new(id, lives, speed, bombs_count, coord);

        m.set_item(p.coord, MapItem::Player(p.id));

        m.change_player_pos(&mut p, Key::Right);

        assert_eq!(*m.at(0, 0).unwrap(), MapItem::Empty);
        assert_eq!(*m.at(0, 1).unwrap(), MapItem::Player(p.id));
    }

    #[test]
    fn change_player_pos_up() {
        let v = vec![vec![0, 0, 0, 0, 0], vec![0, 0]];
        let mut m = Map::from_arr(v);

        let (id, lives, speed, bombs_count, coord) = (1, 3, 1, 0, Coord { x: 0, y: 1 });
        let mut p = Player::new(id, lives, speed, bombs_count, coord);

        m.set_item(p.coord, MapItem::Player(p.id));

        m.change_player_pos(&mut p, Key::Up);

        assert_eq!(*m.at(0, 1).unwrap(), MapItem::Empty);
        assert_eq!(*m.at(0, 0).unwrap(), MapItem::Player(p.id));
    }
}
