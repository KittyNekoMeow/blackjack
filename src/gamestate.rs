use crate::def::Shoe;

pub struct Gamestate {
    pub shoe: Shoe,
    pub player_chips: i32
}

impl Gamestate {
    pub fn new() -> Self {
        Gamestate {
            shoe: Shoe::new(4),
            player_chips: 100
        }
    }
}