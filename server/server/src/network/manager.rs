use std::{collections::HashMap, net::SocketAddr, time::Instant};
use tokio::net::UdpSocket;

use crate::storage::mem_db::MemDB;
use shared::{
    game::{player::Player, vector::Vector2},
    network::packet::Packet,
};

pub struct NetworkManager {
    udp_socket: UdpSocket,
    mem_db: MemDB,
    active_sessions: HashMap<SocketAddr, Session>,
}

impl NetworkManager {
    pub async fn new(udp_port: &str, mem_db: MemDB) -> Self {
        let udp_socket = UdpSocket::bind(format!("0.0.0.0:{udp_port}"))
            .await
            .unwrap();

        NetworkManager {
            udp_socket,
            mem_db,
            active_sessions: HashMap::new(),
        }
    }
}

impl NetworkManager {
    pub async fn start(&mut self) -> anyhow::Result<()> {
        loop {
            match self.recv_packet().await {
                Ok((packet, addr)) => {
                    self.handle_packet(addr, packet).await;
                }
                Err(_) => {}
            }
        }
    }

    async fn recv_packet(&mut self) -> Result<(Packet, SocketAddr), ()> {
        let mut buf = [0; 10000];

        match self.udp_socket.recv_from(&mut buf).await {
            Ok((size, addr)) => {
                if size > buf.len() {
                    eprintln!("Packet size exceeded buffer size.");
                    return Err(());
                }

                let message_string = String::from_utf8_lossy(&buf[..size]);
                let packet: Packet = Packet::decode(message_string.trim().into());

                Ok((packet, addr))
            }

            Err(e) => match e.kind() {
                _ => {
                    eprintln!("Error receiving packet: {}", e);
                    return Err(());
                }
            },
        }
    }

    async fn handle_packet(&mut self, sender: SocketAddr, packet: Packet) {
        let current_session = self.manage_session(sender);
        let response = self.process_packet(&current_session, &packet);

        if response.is_some() {
            let response = response.unwrap().encode();
            let result = self.udp_socket.send_to(response.as_bytes(), sender).await;

            match result {
                Ok(_) => {}
                Err(e) => match e.kind() {
                    _ => {
                        eprintln!("Error sending packet: {}", e);
                    }
                },
            }
        }
    }

    fn manage_session(&mut self, sender: SocketAddr) -> Session {
        match self.active_sessions.get_mut(&sender) {
            Some(session) => {
                session.last_active = Instant::now();
                session.clone()
            }
            None => {
                let new_session = Session::new(sender);
                self.active_sessions
                    .insert(sender.clone(), new_session.clone());
                new_session
            }
        }
    }

    fn set_session_player_id(&mut self, sender: SocketAddr, player_id: Option<String>) -> Session {
        match self.active_sessions.get_mut(&sender) {
            Some(session) => {
                session.last_active = Instant::now();
                session.player_id = player_id;
                session.clone()
            }
            None => {
                let mut new_session = Session::new(sender);
                new_session.player_id = player_id;
                self.active_sessions
                    .insert(sender.clone(), new_session.clone())
                    .unwrap();
                new_session
            }
        }
    }

    pub fn process_packet(&mut self, session: &Session, packet: &Packet) -> Option<Packet> {
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
            }
            1 => {
                // auth
                match packet.packet_subtype {
                    0 => {
                        // init - packet.payload = username
                        let payload = packet.parse_payload();
                        let username = payload[0].clone();

                        if let Some(player_id) = &session.player_id {
                            let player = self.mem_db.get_player(&player_id).unwrap();

                            if player.is_some() {
                                return Some(Packet::new(
                                    packet.packet_type,
                                    packet.packet_subtype,
                                    player.unwrap().to_string(),
                                ));
                            } else {
                                self.set_session_player_id(session.peer, None);
                            }
                        }

                        // TODO: find existing player

                        let next_key = self.mem_db.get_next_id("player");
                        let player = self
                            .mem_db
                            .upsert_player(Player::new(next_key, username))
                            .unwrap();

                        self.set_session_player_id(session.peer, Some(player.id.clone()));

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
                        if session.player_id.is_none() {
                            return None;
                        }

                        // players
                        let players: Vec<Player> = self.mem_db.get_all_players();
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
                        if session.player_id.is_none() {
                            return None;
                        }

                        // movement_input - packet.payload = dir_x,dir_y;rotation
                        let payload = packet.parse_payload();
                        let id = session.player_id.clone().unwrap();

                        let dirs: Vec<&str> = payload[0].split(",").collect();
                        let x: f32 = dirs[0].parse().unwrap_or(0.0);
                        let y: f32 = dirs[1].parse().unwrap_or(0.0);

                        let rotation: f32 = payload[1].parse().unwrap_or(0.0);

                        let input_direction: Vector2 = Vector2::new(x, y);
                        self.mem_db
                            .set_player_movement_input(id, input_direction, rotation);
                        Some(Packet::new(3, 0, "y".into()))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct Session {
    peer: SocketAddr,
    player_id: Option<String>,
    last_active: Instant,
}

impl Session {
    pub fn new(peer: SocketAddr) -> Session {
        Session {
            peer,
            player_id: None,
            last_active: Instant::now(),
        }
    }
}
