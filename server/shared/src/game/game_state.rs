use std::collections::HashMap;

use super::player::Player;
use bitcode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct GameState {
    pub players: HashMap<String, Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
        }
    }
}
