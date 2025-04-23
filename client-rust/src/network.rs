use godot::classes::{INode, Node};
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::str::from_utf8;

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
        format!("{}|{}|{}", t, s, p)
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
        let socket = UdpSocket::bind("0.0.0.0:8081").expect("failed to bind socket");
        socket
            .set_nonblocking(true)
            .expect("failed to set socket to nonblocking");

        let addr = server_addr
            .parse::<SocketAddr>()
            .expect(&format!("got bad server addr {}", server_addr));
        socket.connect(addr).expect(&format!(
            "failed to connect socket to server addr {}",
            server_addr
        ));

        self.socket = Some(socket);

        godot_print!("RUST: server started - connected to {}", addr);
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
    fn send_packet(&mut self, packet_type: i16, packet_subtype: i16, payload: String) {
        match self.socket.as_mut() {
            Some(socket) => {
                let packet_string = Packet::encode_new(packet_type, packet_subtype, payload);

                match socket.send(packet_string.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => {
                        godot_print!("RUST: Failed to send UDP packet: {:?}", e)
                    }
                }
            }
            _ => return,
        }
    }

    #[func]
    fn read_packet(&mut self) -> [GString; 3] {
        let mut buf = [0; 10000];

        match self.socket.as_mut() {
            Some(socket) => match socket.recv(&mut buf) {
                Ok(len) => {
                    if len > 0 {
                        let message_string = from_utf8(&buf[..len])
                            .expect("failed to parse packet")
                            .trim();

                        let packet = Packet::decode(message_string.into());

                        return [
                            packet.packet_type.to_string().into(),
                            packet.packet_subtype.to_string().into(),
                            packet.payload.into(),
                        ];
                    }
                }
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        godot_print!("Error recieving packet: {:?}", e)
                    }
                }
            },
            _ => godot_print!("RUST: tried to recv packet with no socket"),
        }

        return [
            (-1).to_string().into(),
            (-1).to_string().into(),
            "empty".into(),
        ];
    }
}
