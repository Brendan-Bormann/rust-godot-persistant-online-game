use godot::global::{godot_error, godot_print};

use shared::network::packet::Packet;
use std::io::ErrorKind;
use std::net::UdpSocket;

const SERVER_PORT: &str = "8080";
const CLIENT_PORT: &str = "8081";

pub struct ClientNetwork {
    udp_socket: Option<UdpSocket>,
}

impl ClientNetwork {
    pub fn new() -> Self {
        ClientNetwork { udp_socket: None }
    }
}

impl ClientNetwork {
    pub fn start(&mut self, server_ip: String) -> Result<(), ()> {
        match UdpSocket::bind(format!("0.0.0.0:{CLIENT_PORT}")) {
            Ok(socket) => {
                socket.set_nonblocking(true).unwrap();
                socket
                    .connect(format!("{server_ip}:{SERVER_PORT}"))
                    .unwrap();
                self.udp_socket = Some(socket);
                godot_print!("RUST_NETWORK: Connected to SERVER");
                Ok(())
            }
            Err(e) => {
                godot_error!("RUST_NETWORK: Failed to connect to SERVER: {e}");
                Err(())
            }
        }
    }

    pub fn stop_server(&mut self) {
        self.udp_socket = None
    }

    pub fn is_active(&mut self) -> bool {
        // needs better checking
        self.udp_socket.is_some()
    }

    pub fn send_packet(&mut self, packet: Packet) -> Result<(), ()> {
        match &mut self.udp_socket {
            Some(socket) => {
                let data = format!("{}", packet.clone().encode());

                match socket.send(data.as_bytes()) {
                    Ok(_) => Ok(()),
                    Err(e) => {
                        godot_error!("RUST_NETWORK: Failed to send packet to SERVER: {}", e);
                        Err(())
                    }
                }
            }
            None => {
                godot_error!("RUST_NETWORK: Failed to send packet to SERVER: No Socket!");
                Err(())
            }
        }
    }

    pub fn recv_packet(&mut self) -> Option<Packet> {
        if let Some(socket) = &mut self.udp_socket {
            let mut buf = [0; 10000];
            match socket.recv(&mut buf) {
                Ok(size) => {
                    if size > buf.len() {
                        godot_error!("RUST_NETWORK: Recieved a packet that was too large!");
                        return None;
                    }

                    let message_string = String::from_utf8_lossy(&buf[..size]);
                    let packet: Packet = Packet::decode(message_string.trim().into());
                    return Some(packet);
                }
                Err(e) => {
                    if e.kind() != ErrorKind::WouldBlock {
                        godot_error!("RUST_NETWORK: Error when reading packet: {}", e);
                    }
                }
            }
        }

        None
    }
}
