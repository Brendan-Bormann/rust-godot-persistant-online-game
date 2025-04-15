use std::net::{SocketAddr, UdpSocket};

use crate::game::{self, Vector2, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub packet_type: String,
    pub packet_data: String,
}

impl Packet {
    pub fn new(packet_type: String, packet_data: String) -> Packet {
        Packet {
            packet_type,
            packet_data,
        }
    }

    pub fn new_from_str(packet_type: &str, packet_data: &str) -> Packet {
        Packet {
            packet_type: packet_type.into(),
            packet_data: packet_data.into(),
        }
    }
}

pub fn process_packet(socket: &UdpSocket, game: &mut game::Game, addr: SocketAddr, packet: Packet) {
    let reponse_packet: Packet;

    match packet.packet_type.as_str() {
        "ping" => {
            reponse_packet = Packet::new_from_str("pong", "pong");
        }
        "init" => {
            let name = packet.packet_data;
            let new_player = game.add_player(addr, name);
            let new_player_string = serde_json::to_string(&new_player).unwrap();
            reponse_packet = Packet::new("init".into(), new_player_string);
        }
        "map" => {
            let map_data = game.get_map_data();
            let map_data_string = serde_json::to_string(&map_data).unwrap();
            reponse_packet = Packet::new("map".into(), map_data_string);
        }
        "dir" => {
            // println!("dir data {:?}", &packet.packet_data);
            let direction: Vector2 = serde_json::from_str(&packet.packet_data).unwrap();
            game.set_player_direction(addr, direction);
            reponse_packet = Packet::new_from_str("dir", "dir")
        }
        _ => return,
    }

    let serialized = serde_json::to_string(&reponse_packet).unwrap();
    socket.send_to(serialized.as_bytes(), addr).unwrap();
    // println!("Sent {} packet to {}", packet.packet_type, addr);
}
