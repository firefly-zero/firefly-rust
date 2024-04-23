pub enum Player {
    P0,
    P1,
    P2,
    P3,
}

impl From<Player> for u32 {
    fn from(value: Player) -> Self {
        match value {
            Player::P0 => 0,
            Player::P1 => 1,
            Player::P2 => 2,
            Player::P3 => 3,
        }
    }
}

impl TryFrom<usize> for Player {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Player::P0),
            1 => Ok(Player::P1),
            2 => Ok(Player::P2),
            3 => Ok(Player::P3),
            _ => Err(()),
        }
    }
}
