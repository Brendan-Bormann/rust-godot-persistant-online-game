use godot::global::godot_error;
use shared::network::packet::Packet;
use std::{
    io::ErrorKind,
    os::unix::net::{SocketAddr, UnixDatagram},
};

pub struct NetworkManager {
    socket: UnixDatagram,
}

impl NetworkManager {
    pub fn new() -> Self {
        let socket_name = String::from("/tmp/server.socket");
        std::fs::remove_file(&socket_name).unwrap_or_default();
        let unix_socket = UnixDatagram::bind(&socket_name).unwrap();
        unix_socket.set_nonblocking(true).unwrap();

        NetworkManager {
            socket: unix_socket,
        }
    }
}

impl NetworkManager {
    pub fn recv_packet(&mut self) -> Option<(Packet, SocketAddr)> {
        let mut buf = [0; 65000];

        match self.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if size > buf.len() {
                    godot_error!("Packet size exceeded buffer size.");
                    return None;
                }

                let packet: Packet = bitcode::decode(&buf[..size]).unwrap();
                Some((packet, addr))
            }

            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => None,
                _ => {
                    godot_error!("Packet receiving error: {}", e);
                    None
                }
            },
        }
    }

    pub fn recv_all_packets(&mut self) -> Vec<(Packet, SocketAddr)> {
        let mut packets: Vec<(Packet, SocketAddr)> = vec![];

        loop {
            let packet = self.recv_packet();

            if packet.is_some() {
                let unwrapped_packet = packet.unwrap();
                packets.push(unwrapped_packet);
            } else {
                return packets;
            }
        }
    }

    pub fn send_packet(&mut self, packet: &Packet, addr: &SocketAddr) {
        let dest: String = addr
            .as_pathname()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
            .into();

        let _ = self.socket.send_to(&bitcode::encode(packet).to_vec(), dest);
    }
}
