use serde::{Deserialize, Serialize};

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

    pub fn from_string(vec3_string: String) -> Vector3 {
        let pos_vec: Vec<&str> = vec3_string.split(",").collect();
        Vector3::new(
            pos_vec[0].parse().unwrap(),
            pos_vec[1].parse().unwrap(),
            pos_vec[2].parse().unwrap(),
        )
    }
}

impl Vector3 {
    pub fn add_to(&mut self, delta: Vector3) {
        self.x += delta.x;
        self.y += delta.y;
        self.z += delta.z;
    }

    pub fn to_string(self) -> String {
        format!("{:.2},{:.2},{:.2}", self.x, self.y, self.z)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
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

    pub fn from_string(vec2_string: String) -> Vector2 {
        let pos_vec: Vec<&str> = vec2_string.split(",").collect();
        Vector2::new(pos_vec[0].parse().unwrap(), pos_vec[1].parse().unwrap())
    }
}

impl Vector2 {
    pub fn add_to(&mut self, delta: Vector2) {
        self.x += delta.x;
        self.y += delta.y;
    }

    pub fn to_string(self) -> String {
        format!("{:.2},{:.2}", self.x, self.y)
    }
}
