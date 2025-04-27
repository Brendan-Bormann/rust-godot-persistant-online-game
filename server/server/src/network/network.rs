use anyhow::Error;
use r2d2::Pool;
use redis::Client;
use std::io::ErrorKind;
use tokio::net::UdpSocket;

use super::process_packet::process_packet;
use crate::storage::mem_db::MemDB;
use shared::network::packet::Packet;

pub struct NetworkManager {
    udp_socket: UdpSocket,
    mem_db_pool: Pool<Client>,
}

impl NetworkManager {
    pub async fn new(udp_port: &str, mem_db_pool: Pool<Client>) -> Self {
        let udp_socket = UdpSocket::bind(format!("0.0.0.0:{udp_port}"))
            .await
            .unwrap();

        NetworkManager {
            udp_socket,
            mem_db_pool,
        }
    }
}

impl NetworkManager {
    pub async fn start(&mut self) -> anyhow::Result<()> {
        let mut mem_db = MemDB::new(self.mem_db_pool.get().unwrap());

        loop {
            let mut buf = [0; 10000];

            match self.udp_socket.recv_from(&mut buf).await {
                Ok((size, addr)) => {
                    if size > buf.len() {
                        return Err(Error::msg("Packet size exceeded buffer size."));
                    }

                    let message_string = String::from_utf8_lossy(&buf[..size]);
                    let packet: Packet = Packet::decode(message_string.trim().into());
                    let response = process_packet(&mut mem_db, &addr, &packet);

                    if response.is_some() {
                        let response = response.unwrap().encode();
                        self.udp_socket.send_to(response.as_bytes(), addr).await?;
                    }
                }

                Err(error) => match error.kind() {
                    ErrorKind::WouldBlock => {
                        return Ok(());
                    }
                    _ => {
                        return Err(Error::msg(format!("Error receiving packet: {}", error)));
                    }
                },
            }
        }
    }
}
