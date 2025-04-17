use std::net::{SocketAddr, UdpSocket};

use crate::game::vector::Vector2;
use crate::game::{self};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub packet_type: i16,
    pub packet_subtype: i16,
    pub payload: String,
}

impl Packet {
    pub fn new(t: i16, s: i16, p: String) -> Packet {
        Packet {
            packet_type: t,
            packet_subtype: s,
            payload: p,
        }
    }

    pub fn encode(packet: Packet) -> String {
        format!(
            "{}|{}|{}",
            packet.packet_type, packet.packet_subtype, packet.payload
        )
    }

    pub fn encode_new(t: i16, s: i16, p: String) -> String {
        let packet = Packet::new(t, s, p);
        Packet::encode(packet)
    }

    pub fn decode(packet: String) -> Packet {
        let mut parts = packet.splitn(3, '|'); // split into max 3 parts

        let packet_type = parts
            .next()
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(-1);

        let subtype = parts
            .next()
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(-1);

        let payload = parts.next().unwrap_or("").to_string();

        Packet::new(packet_type, subtype, payload)
    }
}

pub fn parse_payload(payload: &str) -> Vec<&str> {
    payload.split(';').collect()
}

pub fn process_packet(
    socket: &UdpSocket,
    game: &mut game::game::Game,
    addr: SocketAddr,
    len: usize,
    packet: String,
) {
    let packet = Packet::decode(packet);

    println!(
        "- Processing packet from {}: [{}|{}] (size: {})",
        addr, packet.packet_type, packet.packet_subtype, len
    );

    match packet.packet_type {
        0 => {
            // ping
            let encoded = Packet::encode_new(packet.packet_type, packet.packet_subtype, "".into());
            socket.send_to(encoded.as_bytes(), addr).unwrap();
        }
        1 => {
            // auth
            match packet.packet_subtype {
                0 => {
                    // create player - packet.payload = username
                    let payload = parse_payload(&packet.payload);
                    let username = payload[0];

                    let player = game.create_player(username.into());
                    let encoded = Packet::encode_new(
                        packet.packet_type,
                        packet.packet_subtype,
                        player.to_string(),
                    );
                    socket.send_to(encoded.as_bytes(), addr).unwrap();
                }
                1 => {
                    // login
                    println!("logged player in");
                    unimplemented!()
                }
                2 => {
                    //logout
                    println!("logged player out");
                    unimplemented!()
                }
                _ => {}
            }
        }
        2 => {
            // update
            match packet.packet_subtype {
                0 => {
                    // players
                    let player_data_string = game.get_player_data();
                    let encoded = Packet::encode_new(
                        packet.packet_type,
                        packet.packet_subtype,
                        player_data_string,
                    );
                    socket.send_to(encoded.as_bytes(), addr).unwrap();
                    println!("- - Replied with {} of data", encoded.as_bytes().len());
                }
                _ => {}
            }
        }
        3 => {
            // input
            match packet.packet_subtype {
                0 => {
                    // direction - packet.payload = id;x,y
                    let payload = parse_payload(&packet.payload);
                    let id = payload[0];
                    let dirs: Vec<&str> = payload[1].split(",").collect();
                    let x: f64 = dirs[0].parse().unwrap_or(0.0);
                    let y: f64 = dirs[1].parse().unwrap_or(0.0);
                    let input_direction: Vector2 = Vector2::new(x, y);
                    game.set_player_direction(id.into(), input_direction);
                    let encoded = Packet::encode_new(3, 0, "".into());
                    socket.send_to(encoded.as_bytes(), addr).unwrap();
                }
                1 => {
                    // rotation - packet.payload = id;r
                    let payload = parse_payload(&packet.payload);
                    let id = payload[0];
                    let rotation: f32 = payload[1].parse().unwrap();

                    game.set_player_rotation(id.into(), rotation);
                    let encoded = Packet::encode_new(3, 0, "".into());
                    socket.send_to(encoded.as_bytes(), addr).unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}
