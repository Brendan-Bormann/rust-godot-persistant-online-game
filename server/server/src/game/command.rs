use crate::network::session::Session;
use shared::network::packet::Packet;
use std::sync::mpsc::Sender;

#[derive(Clone)]
pub struct Command {
    pub command_type: CommandType,
    pub command_subtype: SubcommandType,
    pub command_args: String,
    pub issuing_player_id: Option<String>,
    pub respond_to: Sender<Result<(), ()>>,
}

#[derive(Clone)]
pub enum CommandType {
    _None,
    Network,
    Auth,
    Update,
    Input,
}

#[derive(Clone)]
pub enum SubcommandType {
    _None,
    Ping,
    Login,
    Entity,
    DirectionAndRotation,
}

impl Command {
    pub fn new(
        command_type: CommandType,
        command_subtype: SubcommandType,
        command_args: String,
        issuing_player_id: Option<String>,
        respond_to: Sender<Result<(), ()>>,
    ) -> Self {
        Command {
            command_type,
            command_subtype,
            command_args,
            issuing_player_id,
            respond_to,
        }
    }

    pub fn from_packet(
        packet: Packet,
        session: Session,
        respond_to: Sender<Result<(), ()>>,
    ) -> Option<Command> {
        match packet.packet_type {
            // network request
            0 => {
                match packet.packet_subtype {
                    // ping
                    0 => Some(Command::new(
                        CommandType::Network,
                        SubcommandType::Ping,
                        packet.payload,
                        session.player_id,
                        respond_to,
                    )),
                    // invalid
                    _ => None,
                }
            }
            // auth request
            1 => {
                match packet.packet_subtype {
                    // login - packet.payload = username
                    0 => Some(Command::new(
                        CommandType::Auth,
                        SubcommandType::Login,
                        packet.payload,
                        session.player_id,
                        respond_to,
                    )),
                    // logout
                    1 => {
                        unimplemented!()
                    }
                    _ => None,
                }
            }
            // update request
            2 => match packet.packet_subtype {
                // entities
                0 => {
                    if session.player_id.is_none() {
                        return None;
                    }

                    Some(Command::new(
                        CommandType::Update,
                        SubcommandType::Entity,
                        packet.payload,
                        session.player_id,
                        respond_to,
                    ))
                }
                _ => None,
            },
            // player input request
            3 => {
                match packet.packet_subtype {
                    // rotation & direction
                    0 => {
                        if session.player_id.is_none() {
                            return None;
                        }

                        Some(Command::new(
                            CommandType::Input,
                            SubcommandType::DirectionAndRotation,
                            packet.payload,
                            session.player_id,
                            respond_to,
                        ))
                    }
                    _ => None,
                }
            }
            _ => None,
        }
    }

    // pub fn from_packet(
    //     packet: Packet,
    //     session: Session,
    //     respond_to: Option<Sender<Result<(), ()>>>,
    // ) -> Option<Command> {
    //     match packet.packet_type {
    //         0 => {
    //             // network related
    //             match packet.packet_subtype {
    //                 0 => {
    //                     // ping
    //                     Some(Command::new(
    //                         CommandType::Ping,
    //                         SubcommandType::None,
    //                         packet.payload,
    //                         session.player_id,
    //                         respond_to,
    //                     ))
    //                 }
    //                 _ => None,
    //             }
    //         }
    //         1 => {
    //             // auth
    //             match packet.packet_subtype {
    //                 0 => {
    //                     // init - packet.payload = username
    //                     let payload = packet.parse_payload();
    //                     let username = payload[0].clone();

    //                     if let Some(player_id) = &session.player_id {
    //                         let player = self.mem_db.get_player(&player_id).unwrap();

    //                         if player.is_some() {
    //                             return Some(Packet::new(
    //                                 packet.packet_type,
    //                                 packet.packet_subtype,
    //                                 player.unwrap().to_string(),
    //                             ));
    //                         } else {
    //                             self.set_session_player_id(session.peer, None);
    //                         }
    //                     }

    //                     // TODO: find existing player

    //                     let next_key = self.mem_db.get_next_id("player");
    //                     let player = self
    //                         .mem_db
    //                         .upsert_player(Player::new(next_key, username))
    //                         .unwrap();

    //                     self.set_session_player_id(session.peer, Some(player.id.clone()));

    //                     Some(Packet::new(
    //                         packet.packet_type,
    //                         packet.packet_subtype,
    //                         player.to_string(),
    //                     ))
    //                 }
    //                 1 => {
    //                     // login
    //                     println!("logged player in");
    //                     unimplemented!()
    //                 }
    //                 2 => {
    //                     //logout
    //                     println!("logged player out");
    //                     unimplemented!()
    //                 }
    //                 _ => None,
    //             }
    //         }
    //         2 => {
    //             // update
    //             match packet.packet_subtype {
    //                 0 => {
    //                     if session.player_id.is_none() {
    //                         return None;
    //                     }

    //                     // players
    //                     let players: Vec<Player> = self.mem_db.get_all_players();
    //                     let player_data: String = Player::player_vec_to_string(players);
    //                     Some(Packet::new(2, 0, player_data))
    //                 }
    //                 _ => None,
    //             }
    //         }
    //         3 => {
    //             // input
    //             match packet.packet_subtype {
    //                 0 => {
    //                     if session.player_id.is_none() {
    //                         return None;
    //                     }

    //                     // movement_input - packet.payload = dir_x,dir_y;rotation
    //                     let payload = packet.parse_payload();
    //                     let id = session.player_id.clone().unwrap();

    //                     let dirs: Vec<&str> = payload[0].split(",").collect();
    //                     let x: f32 = dirs[0].parse().unwrap_or(0.0);
    //                     let y: f32 = dirs[1].parse().unwrap_or(0.0);

    //                     let rotation: f32 = payload[1].parse().unwrap_or(0.0);

    //                     let input_direction: Vector2 = Vector2::new(x, y);
    //                     self.mem_db
    //                         .set_player_movement_input(id, input_direction, rotation);
    //                     Some(Packet::new(3, 0, "y".into()))
    //                 }
    //                 _ => None,
    //             }
    //         }
    //         _ => None,
    //     }
    // }
}

impl Command {
    pub fn parse_args(&self) -> Vec<String> {
        self.command_args
            .clone()
            .split(';')
            .map(|s| s.into())
            .collect()
    }
}
