use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub players: HashMap<SocketAddr, Player>,
    next_id: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: HashMap::new(),
            next_id: 1,
        }
    }
}

impl Game {
    pub fn get_next_id(&mut self) -> u32 {
        let next_id = self.next_id;
        self.next_id += 1;
        next_id
    }

    pub fn add_player(&mut self, addr: SocketAddr, name: String) -> Player {
        if self.players.contains_key(&addr) {
            return self.players[&addr].clone();
        }

        let new_player = Player::new(self.get_next_id(), name);
        let new_player_clone = new_player.clone();
        self.players.insert(addr, new_player);
        new_player_clone
    }

    pub fn game_tick(&mut self) {
        for player in self.players.values_mut() {
            player.move_player();
        }
    }

    pub fn get_map_data(&mut self) -> Vec<Player> {
        let mut players: Vec<Player> = vec![];
        for (_, player) in &self.players {
            players.push(player.clone());
        }

        players
    }

    pub fn set_player_direction(&mut self, addr: SocketAddr, dir: Vector2) {
        let player = self.players.get_mut(&addr).unwrap();
        player.set_direction(dir);
    }
}

// ===== Player =====

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub position: Vector3,
    pub direction: Vector2,
    pub speed: f64,
}

impl Player {
    pub fn new(id: u32, name: String) -> Player {
        Player {
            id,
            name,
            position: Vector3::zero(),
            direction: Vector2::zero(),
            speed: 0.1,
        }
    }
}

impl Player {
    pub fn move_player(&mut self) {
        if self.direction == Vector2::zero() {
            return;
        }

        let delta = Vector3::new(
            self.direction.x * self.speed,
            0.0,
            self.direction.y * self.speed,
        );

        self.position.add_to(delta);
    }

    pub fn set_direction(&mut self, new_dir: Vector2) {
        self.direction = new_dir;
    }
}

// ===== Vector3 =====

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn add(vec: Vector3, delta: Vector3) -> Vector3 {
        Vector3 {
            x: vec.x + delta.x,
            y: vec.y + delta.y,
            z: vec.z + delta.z,
        }
    }
}

impl Vector3 {
    pub fn add_to(&mut self, delta: Vector3) {
        self.x += delta.x;
        self.y += delta.y;
        self.z += delta.z;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn zero() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }

    pub fn add(vec: Vector2, delta: Vector2) -> Vector2 {
        Vector2 {
            x: vec.x + delta.x,
            y: vec.y + delta.y,
        }
    }
}

impl Vector2 {
    pub fn add_to(&mut self, delta: Vector2) {
        self.x += delta.x;
        self.y += delta.y;
    }
}
