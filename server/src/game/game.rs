use crate::game::vector::Vector2;
use crate::{game::player::Player, storage::mem_db::MemDB};

pub struct Game {
    pub mem_db: MemDB,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mem_db: MemDB::new(),
        }
    }
}

impl Game {
    pub fn create_player(&mut self, username: String) -> Player {
        let new_player = Player::new(self.mem_db.get_next_id("player"), username);
        let new_player_clone = new_player.clone();
        self.mem_db
            .set("player", &new_player.id.to_string(), new_player.to_string())
            .unwrap();
        new_player_clone
    }

    pub fn game_tick(&mut self) {
        let player_keys = self.mem_db.get_all_keys("player");

        for player_key in player_keys {
            let player_string = self.mem_db.get_raw(&player_key).unwrap();
            let mut player = Player::from_string(player_string);
            player.move_player();
            self.mem_db
                .set("player", &player.id.to_string(), player.to_string())
                .unwrap();
        }
    }

    pub fn get_player_data(&mut self) -> String {
        let player_keys = self.mem_db.get_all_keys("player");
        let mut player_data: Vec<String> = vec![];

        for player_key in player_keys {
            let player_string: String = self.mem_db.get_raw(&player_key).unwrap();
            player_data.push(player_string);
        }

        player_data.join("+")
    }

    pub fn set_player_direction(&mut self, id: String, dir: Vector2) {
        match self.mem_db.get("player", &id) {
            Ok(player_string) => {
                let mut player = Player::from_string(player_string);
                player.set_direction(dir);
                self.mem_db
                    .set("player", &player.id.to_string(), player.to_string())
                    .unwrap();
            }
            Err(_) => {}
        }
    }

    pub fn set_player_rotation(&mut self, id: String, rotation: f32) {
        match self.mem_db.get("player", &id) {
            Ok(player_string) => {
                let mut player = Player::from_string(player_string);
                player.set_rotation(rotation);
                self.mem_db
                    .set("player", &player.id.to_string(), player.to_string())
                    .unwrap();
            }
            Err(_) => {}
        }
    }
}
