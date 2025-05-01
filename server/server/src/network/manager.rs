use shared::game::player::Player;
use std::sync::mpsc::{Receiver, channel};
use std::time::Duration;
use std::{collections::HashMap, net::SocketAddr, sync::mpsc::Sender, time::Instant};
use tokio::net::UdpSocket;
use tracing::warn;

use crate::game::command::{Command, CommandType, SubcommandType};
use crate::storage::mem_db::MemDB;
use shared::network::packet::Packet;

use super::session::{self, Session};

pub struct NetworkManager {
    udp_socket: UdpSocket,
    mem_db: MemDB,
    active_sessions: HashMap<SocketAddr, Session>,
    command_tx: Sender<Command>,
}

impl NetworkManager {
    pub async fn new(udp_port: &str, mem_db: MemDB, command_tx: Sender<Command>) -> Self {
        let udp_socket = UdpSocket::bind(format!("0.0.0.0:{udp_port}"))
            .await
            .unwrap();

        NetworkManager {
            udp_socket,
            mem_db,
            active_sessions: HashMap::new(),
            command_tx,
        }
    }
}

impl NetworkManager {
    pub async fn start(&mut self) -> Result<(), ()> {
        loop {
            match self.recv_packet().await {
                Ok((packet, addr)) => {
                    self.handle_packet(addr, &packet).await;
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

    async fn send_packet(&mut self, addr: SocketAddr, packet: &Packet) {
        let data = packet.clone().encode();
        match self.udp_socket.send_to(data.as_bytes(), addr).await {
            Ok(_) => {}
            Err(_) => {
                warn!("Failed to send packet!")
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

    pub async fn handle_packet(&mut self, sender: SocketAddr, packet: &Packet) -> Result<(), ()> {
        let mut current_session = self.manage_session(sender);
        let (resp_tx, resp_rx) = channel::<Result<(), ()>>();
        let command = Command::from_packet(packet.clone(), current_session.clone(), resp_tx);

        if let Some(command) = command {
            match command.command_type {
                CommandType::Network => {
                    self.handle_network_command(&mut current_session, command, &packet)
                        .await
                }
                CommandType::Auth => {
                    self.handle_auth_command(&mut current_session, command)
                        .await
                }
                CommandType::Update => {
                    self.handle_update_command(&mut current_session, command, &packet)
                        .await
                }
                CommandType::Input => {
                    self.handle_engine_command(&mut current_session, command, &packet, resp_rx)
                        .await
                }
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    pub async fn handle_network_command(
        &mut self,
        session: &mut Session,
        command: Command,
        original_packet: &Packet,
    ) -> Result<(), ()> {
        match command.command_subtype {
            SubcommandType::Ping => {
                self.send_packet(
                    session.peer,
                    &Packet::new(
                        original_packet.packet_type,
                        original_packet.packet_subtype,
                        "y".into(),
                    ),
                )
                .await;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_auth_command(
        &mut self,
        session: &mut Session,
        command: Command,
    ) -> Result<(), ()> {
        match command.command_subtype {
            SubcommandType::Login => {
                let args = command.parse_args();
                let username = args[0].clone();

                if let Some(player_id) = session.player_id.clone() {
                    let player = self
                        .mem_db
                        .get_player(&player_id)
                        .expect("Failed to get user from session.");

                    if let Some(player) = player {
                        self.send_packet(session.peer, &Packet::new(1, 0, player.to_string()))
                            .await;
                        return Ok(());
                    } else {
                        self.set_session_player_id(session.peer, None);
                    }
                }

                let next_key = self.mem_db.get_next_id("player");
                let player = self
                    .mem_db
                    .upsert_player(Player::new(next_key, username))
                    .unwrap();

                self.set_session_player_id(session.peer, Some(player.id.clone()));
                self.send_packet(session.peer, &Packet::new(1, 0, player.to_string()))
                    .await;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_update_command(
        &mut self,
        session: &mut Session,
        command: Command,
        original_packet: &Packet,
    ) -> Result<(), ()> {
        match command.command_subtype {
            SubcommandType::Entity => {
                let players = self.mem_db.get_all_players();
                self.send_packet(
                    session.peer,
                    &Packet::new(
                        original_packet.packet_type,
                        original_packet.packet_subtype,
                        Player::player_vec_to_string(players),
                    ),
                )
                .await;
            }
            _ => {}
        }
        Ok(())
    }

    pub async fn handle_engine_command(
        &mut self,
        session: &Session,
        command: Command,
        original_packet: &Packet,
        resp_rx: Receiver<Result<(), ()>>,
    ) -> Result<(), ()> {
        let result = self.command_tx.send(command);

        if result.is_err() {
            warn!("Error in command channel: {}", result.unwrap_err());
            return Err(());
        }

        match resp_rx.recv_timeout(Duration::from_millis(100)) {
            Ok(response) => {
                let resp_packet: Packet;

                if response.is_ok() {
                    resp_packet = Packet::new(
                        original_packet.packet_type,
                        original_packet.packet_subtype,
                        "y".into(),
                    );
                } else {
                    resp_packet = Packet::new(
                        original_packet.packet_type,
                        original_packet.packet_subtype,
                        "n".into(),
                    );
                }

                let _ = self.send_packet(session.peer, &resp_packet).await;
                return Ok(());
            }
            Err(e) => {
                eprintln!("Error in command channel: {}", e);
                return Err(());
            }
        }
    }
}
