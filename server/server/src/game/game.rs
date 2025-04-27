use shared::game::player::Player;
use shared::game::vector::Vector2;

use crate::storage::mem_db::MemDB;

use super::physics::PhysicsManager;

pub struct Game {
    mem_db: MemDB,
    pm: PhysicsManager,
}

impl Game {
    pub fn new(mem_db: MemDB, pm: PhysicsManager) -> Self {
        Game { mem_db, pm }
    }
}

impl Game {
    pub fn create_player(&mut self, username: String) -> Player {
        let new_player = Player::new(self.mem_db.get_next_id("player"), username);
        self.pm.create_player_rb(&new_player.id);
        self.mem_db.upsert_player(new_player).unwrap()
    }

    pub fn game_tick(&mut self, delta_time: f32) {
        let players = self.mem_db.get_all_players();

        self.pm.step(delta_time);

        for player in players {
            let new_position = self.pm.move_player(&player, delta_time).unwrap();
            self.mem_db.set_player_position(player.id, new_position);
        }
    }

    pub fn set_player_direction(&mut self, id: String, dir: Vector2) -> Player {
        let mut player = self.mem_db.get_player(&id).unwrap().unwrap();
        player.input_direction = dir;
        self.mem_db.upsert_player(player).unwrap()
    }

    pub fn set_player_rotation(&mut self, id: String, rotation: f32) -> Player {
        let mut player = self.mem_db.get_player(&id).unwrap().unwrap();
        player.rotation = rotation;
        self.mem_db.upsert_player(player).unwrap()
    }
}
