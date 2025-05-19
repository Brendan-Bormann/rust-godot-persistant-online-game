use std::{
    io,
    net::{SocketAddr, UdpSocket},
    sync::Arc,
};

use super::packet::Packet;

pub struct PacketUDP {
    pub socket: Arc<UdpSocket>,
}

impl PacketUDP {
    pub fn new(socket: Arc<UdpSocket>) -> Self {
        PacketUDP { socket }
    }

    pub fn recv_packet(&mut self) -> Result<(Packet, SocketAddr), io::Error> {
        let mut buf = [0; 65000];

        match self.socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if size > buf.len() {
                    eprintln!("Packet size exceeded buffer size.");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Packet size too large",
                    ));
                }

                let packet: Packet = bitcode::decode(&buf[..size]).map_err(|e| {
                    io::Error::new(io::ErrorKind::InvalidData, format!("Decode error: {}", e))
                })?;

                Ok((packet, addr))
            }

            Err(e) => match e.kind() {
                _ => {
                    // eprintln!("Error receiving packet: {}", e);
                    Err(e)
                }
            },
        }
    }

    pub fn send_packet(&mut self, addr: &String, packet: &Packet) -> io::Result<usize> {
        self.socket.send_to(&bitcode::encode(packet).to_vec(), addr)
    }
}
