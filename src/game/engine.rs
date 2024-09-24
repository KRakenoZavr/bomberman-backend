use std::{collections::HashMap, sync::Arc};

use parking_lot::Mutex;

use crate::game::{
    key_handler::KeyHandler,
    map::{Map, MapItem},
    movement::Movement,
    player::Player,
};

use super::key_handler::{self, Key};

pub struct Engine {
    pub players: Arc<Mutex<HashMap<u8, Player>>>,
    pub map: Arc<Mutex<Map>>,
    pub key_handlers: Arc<Mutex<HashMap<u8, KeyHandler>>>,
}

#[allow(dead_code)]
impl Engine {
    pub fn new() -> Self {
        Self {
            players: Arc::new(Mutex::new(HashMap::new())),
            map: Arc::new(Mutex::new(Map(vec![]))),
            key_handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        let mut players = self.players.lock();
        let id = player.id.clone();
        players.insert(id, player);

        let kh = KeyHandler::new();
        let mut khs = self.key_handlers.lock();
        khs.insert(id, kh);
    }

    pub fn set_map(&mut self, map: Map) {
        self.map = Arc::new(Mutex::new(map));
    }
}

// for API call
impl Engine {
    // MAP
    pub fn get_map(&self) -> Map {
        self.map.lock().clone()
    }

    //pub fn map_change_event(&mut self, )

    // KeyboardEvents
    pub fn handle_key(&mut self, player_id: u8, key: u8, event: &str) -> Option<()> {
        let mut kh = self.key_handlers.lock();
        let kh = kh.get_mut(&player_id).unwrap();
        kh.handle_key(key, event);
        Some(())
    }
}

#[allow(dead_code)]
impl Engine {
    //pub fn place_bomb(&self) {}

    // TODO
    // use debounce
    //pub fn handle_key(&mut self, key: u8, event: &str) {
    //self.key_handler.handle_key(key, event);
    //}
}

// impl Engine loop check key handler
impl Engine {
    pub fn key_handler_loop(&mut self) {
        loop {
            //let khs = self.key_handlers.lock();
            //for key in khs.keys() {
            //let kh =
            //}
            //for key in .keys() {
            //if
            //}
            //if self.key_handler.is_key(Key::Up) {
            //self.move_up(id)
            //}
        }
    }
}

// TODO
// impl Movement trait here, remove from player
// movement impl
#[allow(dead_code)]
impl Engine {
    pub fn move_left(&mut self, id: u8) -> Option<()> {
        let guard_player = self.players.lock();
        let player = guard_player.get(&id)?;

        //let x = player.coord.x.checked_sub(player.speed)?;
        //let y = player.coord.y;

        // TODO
        // ccheck_sub => move player till wall
        if self.check_if_wall(player.coord.x.checked_sub(player.speed)?, player.coord.y) {
            let mut players = self.players.lock();
            players.get_mut(&id)?.left();
            return Some(());
        }

        None
    }

    pub fn move_right(&mut self, id: u8) -> Option<()> {
        let guard_player = self.players.lock();
        let player = guard_player.get(&id)?;
        if self.check_if_wall(player.coord.x + player.speed, player.coord.y) {
            let mut players = self.players.lock();
            players.get_mut(&id)?.left();
            return Some(());
        }

        None
    }

    pub fn move_up(&mut self, id: u8) -> Option<()> {
        let guard_player = self.players.lock();
        let player = guard_player.get(&id)?;
        if self.check_if_wall(player.coord.x, player.coord.y.checked_sub(player.speed)?) {
            let mut players = self.players.lock();
            players.get_mut(&id)?.left();
            return Some(());
        }

        None
    }

    pub fn move_down(&mut self, id: u8) -> Option<()> {
        let guard_player = self.players.lock();
        let player = guard_player.get(&id)?;
        if self.check_if_wall(player.coord.x, player.coord.y + player.speed) {
            let mut players = self.players.lock();
            players.get_mut(&id)?.left();
            return Some(());
        }

        None
    }

    fn check_if_wall(&self, x: u8, y: u8) -> bool {
        match self.map.lock().at(x, y) {
            Some(item) => match item {
                MapItem::WallTwo => false,
                MapItem::Wall => false,
                _ => true,
            },
            None => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::game::movement::Coord;

    #[test]
    fn player_movement_end_of_map() {
        let mut engine = Engine::new();

        let v = vec![vec![0; 10]; 10];
        let map = Map::from_arr(v);
        engine.set_map(map);

        let player_id = 0_u8;
        let player = Player::new(player_id, 1, 1, 1, Coord { x: 0, y: 0 });

        engine.add_player(player);
        assert_eq!(engine.move_left(player_id), None);
    }

    #[test]
    fn player_movement_ok_and_coords() {
        let mut engine = Engine::new();

        let v = vec![vec![0; 10]; 10];
        let map = Map::from_arr(v);
        engine.set_map(map);

        let player_id = 0_u8;
        let player = Player::new(player_id, 1, 1, 1, Coord { x: 5, y: 5 });

        engine.add_player(player);
        assert_eq!(engine.move_left(player_id), Some(()));
        assert_eq!(
            engine
                .players
                .lock()
                .get(&player_id)
                .unwrap()
                .coord
                .eq(&Coord { x: 4, y: 5 }),
            true
        );
    }
    #[test]
    fn player_movement_wall() {
        let mut engine = Engine::new();

        let v = vec![vec![0; 10]; 10];
        let map = Map::from_arr(v);
        engine.set_map(map);

        let player_id = 0_u8;
        let player = Player::new(player_id, 1, 2, 1, Coord { x: 1, y: 1 });

        engine.add_player(player);
        assert_eq!(engine.move_left(player_id), None)
    }
}
