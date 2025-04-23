use redis::{Client, Commands, Connection, RedisResult};

use crate::game::{
    player::Player,
    vector::{Vector2, Vector3},
};

pub struct MemDB {
    client: Client,
}

impl MemDB {
    pub fn new() -> MemDB {
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        MemDB { client }
    }
}

impl MemDB {
    fn get(&mut self, pre: &str, key: &str) -> RedisResult<Option<String>> {
        let k = format!("{}:{}", pre, key);
        let mut con: Connection = self.client.get_connection().unwrap();
        con.get(&k)
    }

    fn get_raw(&mut self, key: &str) -> RedisResult<Option<String>> {
        let mut con: Connection = self.client.get_connection().unwrap();
        con.get(&key)
    }

    fn set(&mut self, pre: &str, key: &str, value: &str) -> RedisResult<()> {
        let k = format!("{}:{}", pre, key);
        let mut con: Connection = self.client.get_connection().unwrap();
        con.set(k, value)
    }

    fn del(&mut self, pre: &str, key: &str) -> RedisResult<()> {
        let k = format!("{}:{}", pre, key);
        let mut con: Connection = self.client.get_connection().unwrap();
        con.del(k)
    }

    fn get_all_keys(&mut self, pre: &str) -> Vec<String> {
        let k = format!("{}:*", pre);
        let mut con: Connection = self.client.get_connection().unwrap();
        let collection = con.keys(&k).unwrap();

        collection
    }

    pub fn get_next_id(&mut self, key_name: &str) -> String {
        let pre = "next_key";

        match self.get(pre, &key_name) {
            Ok(result) => match result {
                Some(next_id) => {
                    let next_id: u32 = next_id.parse().expect("failed to parse id!");
                    self.set(pre, key_name, &(next_id + 1).to_string()).unwrap();
                    next_id.to_string()
                }
                None => {
                    let next_id: u32 = 1;
                    self.set(pre, key_name, &(next_id + 1).to_string()).unwrap();
                    next_id.to_string()
                }
            },
            Err(e) => {
                panic!("Failed to get next id! {}", e)
            }
        }
    }

    pub fn wipe(&mut self) {
        let mut con: Connection = self.client.get_connection().unwrap();
        let _result: String = redis::cmd("FLUSHALL").query(&mut con).unwrap();
    }

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
