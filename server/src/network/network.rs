use std::{
    io::ErrorKind,
    net::{SocketAddr, UdpSocket},
};

use r2d2::Pool;
use redis::Client;

use crate::storage::mem_db::MemDB;

use super::{packet::Packet, process_packet::process_packet};

// ticks per second
const NETWORK_TICK_RATE: u16 = 60;

pub struct NetworkManager {
    udp_socket: UdpSocket,
    mem_db_pool: Pool<Client>,
}

impl NetworkManager {
    pub fn new(udp_port: &str, mem_db_pool: Pool<Client>) -> Self {
        let udp_socket = UdpSocket::bind(format!("0.0.0.0:{udp_port}")).unwrap();
        udp_socket.set_nonblocking(true).unwrap();

        NetworkManager {
            udp_socket: udp_socket,
            mem_db_pool,
        }
    }
}

impl NetworkManager {
    pub fn start(&mut self) {
        let mut mem_db = MemDB::new(self.mem_db_pool.get().unwrap());

        loop {
            let packet = self.recv_packet();

            if packet.is_some() {
                let (packet, addr) = packet.unwrap();
                let response = process_packet(&mut mem_db, &packet);

                if response.is_some() {
                    let response = response.unwrap();
                    let _ = self.send_packet(response, addr);
                }
            }
        }
    }

    pub fn recv_packet(&mut self) -> Option<(Packet, SocketAddr)> {
        let mut buf = [0; 10000];

        match self.udp_socket.recv_from(&mut buf) {
            Ok((size, addr)) => {
                if size > buf.len() {
                    eprintln!("- Recieved a packet that was too large!");
                    return None;
                }

                let message_string = String::from_utf8_lossy(&buf[..size]);
                let packet: Packet = Packet::decode(message_string.trim().into());
                Some((packet, addr))
            }
            Err(e) => match e.kind() {
                ErrorKind::WouldBlock => None,
                _ => {
                    eprintln!("- Error when reading packet: {}", e);
                    None
                }
            },
        }
    }

    pub fn send_packet(&mut self, packet: Packet, addr: SocketAddr) -> Result<(), ()> {
        let data = packet.clone().encode();
        match self.udp_socket.send_to(data.as_bytes(), addr) {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("- Error when reading packet: {}", e);
                Err(())
            }
        }
    }
}
