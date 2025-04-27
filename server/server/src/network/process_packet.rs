use std::net::SocketAddr;

use crate::storage::mem_db::MemDB;
use shared::game::player::Player;
use shared::game::vector::Vector2;
use shared::network::command::Command;
use shared::network::packet::Packet;

pub fn process_packet(mem_db: &mut MemDB, sender: &SocketAddr, packet: &Packet) -> Option<Packet> {
    // println!(
    //     "- Processing packet from {}: [{}|{}] (size: {})",
    //     addr, packet.packet_type, packet.packet_subtype, len
    // );

    match packet.packet_type {
        0 => {
            // ping
            Some(Packet::new(
                packet.packet_type,
                packet.packet_subtype,
                "y".into(),
            ))

            // Some(Command::new("ping".into(), "".into(), vec![], &sender))
        }
        1 => {
            // auth
            match packet.packet_subtype {
                0 => {
                    // init - packet.payload = username
                    let payload = packet.parse_payload();
                    let username = payload[0].clone();

                    let next_key = mem_db.get_next_id("player");
                    let player = mem_db
                        .upsert_player(Player::new(next_key, username))
                        .unwrap();
                    Some(Packet::new(
                        packet.packet_type,
                        packet.packet_subtype,
                        player.to_string(),
                    ))
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
                _ => None,
            }
        }
        2 => {
            // update
            match packet.packet_subtype {
                0 => {
                    // players
                    let players: Vec<Player> = mem_db.get_all_players();
                    let player_data: String = Player::player_vec_to_string(players);
                    Some(Packet::new(2, 0, player_data))
                }
                _ => None,
            }
        }
        3 => {
            // input
            match packet.packet_subtype {
                0 => {
                    // movement_input - packet.payload = id;dir_x,dir_y;rotation
                    let payload = packet.parse_payload();
                    let id = payload[0].clone();

                    let dirs: Vec<&str> = payload[1].split(",").collect();
                    let x: f32 = dirs[0].parse().unwrap_or(0.0);
                    let y: f32 = dirs[1].parse().unwrap_or(0.0);

                    let rotation: f32 = payload[2].parse().unwrap_or(0.0);

                    let input_direction: Vector2 = Vector2::new(x, y);
                    mem_db.set_player_movement_input(id, input_direction, rotation);
                    Some(Packet::new(3, 0, "y".into()))
                }
                _ => None,
            }
        }
        _ => None,
    }
}
