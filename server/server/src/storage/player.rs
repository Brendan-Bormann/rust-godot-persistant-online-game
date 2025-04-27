use super::mem_db::MemDB;
use shared::game::player::Player;
use shared::game::vector::{Vector2, Vector3};

impl MemDB {
    pub fn get_player(&mut self, id: &str) -> Result<Option<Player>, ()> {
        match self.get("player", &id.to_string()) {
            Ok(player_string) => match player_string {
                Some(player_string) => Ok(Some(Player::from_string(player_string))),
                None => Ok(None),
            },
            Err(_) => panic!("Failed to get player!"),
        }
    }

    pub fn get_all_players(&mut self) -> Vec<Player> {
        let player_keys = self.get_all_keys("player");
        let mut players: Vec<Player> = vec![];

        for player_key in player_keys {
            let player = self
                .get_raw(&player_key)
                .expect("Failed to get all players");

            match player {
                Some(player) => players.push(Player::from_string(player)),
                None => {
                    println!("searched for a player but did not find it???")
                }
            }
        }

        players
    }

    pub fn upsert_player(&mut self, player: Player) -> Result<Player, ()> {
        match self.set("player", &player.id, &player.to_string()) {
            Ok(_) => Ok(player),
            Err(_) => Err(()),
        }
    }

    pub fn delete_player(&mut self, id: String) -> Result<(), ()> {
        match self.del("player", &id.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn set_player_position(&mut self, id: String, new_position: Vector3) -> Player {
        let mut player = self.get_player(&id).unwrap().unwrap();
        player.position = new_position;
        self.upsert_player(player).unwrap()
    }

    pub fn set_player_movement_input(&mut self, id: String, dir: Vector2, rot: f32) -> Player {
        let mut player = self.get_player(&id).unwrap().unwrap();
        player.input_direction = dir;
        player.rotation = rot;
        self.upsert_player(player).unwrap()
    }

    pub fn get_physics_player(&mut self, id: &str) -> Result<Option<Player>, ()> {
        match self.get("physics_player", &id.to_string()) {
            Ok(player_string) => match player_string {
                Some(player_string) => Ok(Some(Player::from_string(player_string))),
                None => Ok(None),
            },
            Err(_) => panic!("Failed to get player!"),
        }
    }
}
