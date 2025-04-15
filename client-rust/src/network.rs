use godot::classes::{INode, Node};
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;

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

    pub fn serialize_packet(packet: Packet) -> String {
        serde_json::to_string(&packet).expect("Failed to serialize packet")
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Network {
    socket: Option<UdpSocket>,
}

#[godot_api]
impl INode for Network {
    fn init(base: Base<Node>) -> Self {
        godot_print!("RUST: extension compiled");
        Self { socket: None }
    }
}

#[godot_api]
impl Network {
    #[func]
    fn start_server(&mut self, server_addr: String) {
        godot_print!("RUST: addr - {:?}", server_addr);

        let socket_addr = match server_addr.parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(e) => {
                godot_print!("RUST: addr err - {:?}", e);
                panic!("addr err - {:?}", e)
            }
        };

        let socket = match UdpSocket::bind("0.0.0.0:8081") {
            Ok(socket) => socket,
            Err(e) => {
                godot_print!("RUST: socket err - {:?}", e);
                panic!("socket err - {:?}", e)
            }
        };

        match socket.set_nonblocking(true) {
            Ok(_) => {}
            Err(e) => {
                godot_print!("RUST: socket nonblocking err - {:?}", e);
                panic!("socket nonblocking err - {:?}", e)
            }
        }

        match socket.connect(socket_addr) {
            Ok(_) => {}
            Err(e) => {
                godot_print!("RUST: socket connect err - {:?}", e);
                panic!("socket connect err - {:?}", e)
            }
        }

        self.socket = Some(socket);

        godot_print!("RUST: server started - connected to {}", socket_addr);
    }

    #[func]
    fn stop_server(&mut self) {
        self.socket = None;
    }

    #[func]
    fn is_active(&mut self) -> bool {
        self.socket.is_some()
    }

    #[func]
    fn send_packet(&mut self, packet_type: String, packet_data: String) {
        match self.socket.as_mut() {
            Some(socket) => {
                let packet_to_send = Packet::new(packet_type, packet_data);
                let packet_string = Packet::serialize_packet(packet_to_send);

                match socket.send(packet_string.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        godot_print!("RUST: Failed to send UDP packet: {:?}", e)
                    }
                }
            }
            _ => godot_print!("RUST: tried to send packet with no socket"),
        }
    }

    #[func]
    fn read_packet(&mut self) -> [GString; 2] {
        let mut buf = [0; 10000];

        match self.socket.as_mut() {
            Some(socket) => match socket.recv(&mut buf) {
                Ok(len) => {
                    let message_string = from_utf8(&buf[..len])
                        .expect("failed to parse packet")
                        .trim();
                    let packet: Packet =
                        serde_json::from_str(message_string).expect("failed to deserialize packet");

                    return [packet.packet_type.into(), packet.packet_data.into()];
                }
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        godot_print!("Error recieving packet: {:?}", e)
                    }
                }
            },
            _ => godot_print!("RUST: tried to recv packet with no socket"),
        }

        return ["empty".into(), "empty".into()];
    }
}
