use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
}

impl TryFrom<u8> for Key {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            38 => Ok(Key::Up),
            37 => Ok(Key::Left),
            39 => Ok(Key::Right),
            40 => Ok(Key::Down),
            32 => Ok(Key::Space),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    KeyUp,
    KeyDown,
    KeyPress,
}

impl TryFrom<&str> for Event {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "key_up" => Ok(Event::KeyUp),
            "key_down" => Ok(Event::KeyDown),
            "key_press" => Ok(Event::KeyPress),
            _ => Err(()),
        }
    }
}

pub struct KeyHandler(HashMap<Key, bool>);

use Key::*;

#[allow(dead_code)]
impl KeyHandler {
    fn default_map() -> Self {
        let mut hm: HashMap<Key, bool> = HashMap::new();
        hm.insert(Left, false);
        hm.insert(Right, false);
        hm.insert(Up, false);
        hm.insert(Down, false);
        hm.insert(Space, false);
        hm
    }

    pub fn new() -> Self {
        KeyHandler::default_map()
    }

    fn key_down(&mut self, key: Key) {
        self.0.insert(key, true);
    }

    fn key_up(&mut self, key: Key) {
        self.0.insert(key, false);
    }

    fn key_press(&mut self, key: Key) {
        self.0.insert(key, true);
    }

    pub fn handle_key(&mut self, key: u8, event: &str) {
        let key = Key::try_from(key).unwrap();
        let event = Event::try_from(event).unwrap();

        match event {
            Event::KeyUp => self.key_up(key),
            Event::KeyDown => self.key_down(key),
            Event::KeyPress => self.key_press(key),
        }
    }

    pub fn is_space(&self) -> bool {
        *self.0.get(&Space).unwrap()
    }

    pub fn is_key(&self, key: Key) -> bool {
        *self.0.get(&key).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_event_handler() {
        let mut kh = KeyHandler::new();

        let events: [(u8, &str); 4] = [
            (38, "key_down"),
            (37, "key_down"),
            (38, "key_down"),
            (32, "key_press"),
        ];

        for (key, event) in events {
            kh.handle_key(key, event)
        }

        assert_eq!(true, kh.is_key(Key::Up));
        assert_eq!(true, kh.is_key(Key::Left));
        assert_eq!(true, kh.is_key(Key::Space));
        assert_eq!(true, kh.is_space());

        kh.handle_key(32, "key_up");

        assert_eq!(false, kh.is_key(Key::Space));
    }
}
