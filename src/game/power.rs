#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerUp {
    OneUp,
    Bomb(u8),
    Flame(u8),
    Speed(u8, u8),
}
