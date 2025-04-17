use crate::game::vector::{Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub username: String,
    pub position: Vector3,
    pub direction: Vector2,
    pub rotation: f32,
    pub speed: f64,
}

impl Player {
    pub fn new(id: u32, username: String) -> Player {
        Player {
            id,
            username,
            position: Vector3::zero(),
            direction: Vector2::zero(),
            rotation: 0.0,
            speed: 0.1,
        }
    }

    pub fn from_string(player_string: String) -> Player {
        let player_parts: Vec<&str> = player_string.split(';').collect();

        let id: u32 = player_parts[0].parse().unwrap();
        let username: String = player_parts[1].into();

        let position: Vector3 = Vector3::from_string(player_parts[2].into());
        let direction: Vector2 = Vector2::from_string(player_parts[3].into());

        let rotation: f32 = player_parts[4].parse().unwrap();

        Player {
            id,
            username,
            position,
            direction,
            rotation,
            speed: 0.1,
        }
    }
}

impl Player {
    pub fn to_string(self) -> String {
        format!(
            "{};{};{};{};{:.2}",
            self.id,
            self.username,
            self.position.to_string(),
            self.direction.to_string(),
            self.rotation
        )
    }

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

    // pub fn move_player(&mut self) {
    //     if self.direction == Vector2::zero() {
    //         return;
    //     }

    //     // Convert rotation (in degrees) to radians
    //     let radians = self.rotation.to_radians() as f64;

    //     // Calculate movement direction based on rotation
    //     let dir_x = radians.sin();
    //     let dir_z = radians.cos();

    //     // Apply direction magnitude from input (e.g., WASD-style) to rotate the input vector
    //     let move_x = self.direction.y * dir_x + self.direction.x * dir_z;
    //     let move_z = self.direction.y * dir_z - self.direction.x * dir_x;

    //     let delta = Vector3::new(move_x * self.speed, 0.0, move_z * self.speed);

    //     self.position.add_to(delta);
    // }

    pub fn set_direction(&mut self, new_dir: Vector2) {
        self.direction = new_dir;
    }

    pub fn set_rotation(&mut self, new_rot: f32) {
        self.rotation = new_rot.clamp(-180.0, 180.0);
    }
}
