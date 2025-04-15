mod game;
mod network;

use std::io;
use std::net::UdpSocket;
use std::{thread, time::Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    socket.set_nonblocking(true).unwrap();

    let mut game = game::Game::new();

    loop {
        let mut buf = [0; 10000];
        match socket.recv_from(&mut buf) {
            Ok((len, addr)) => {
                println!("Got packet from {} (amt: {})", addr, len);

                let message_string = String::from_utf8_lossy(&buf[..len]);

                println!("- message: {}", message_string);

                let request_packet: network::Packet =
                    serde_json::from_str(message_string.trim()).unwrap();

                network::process_packet(&socket, &mut game, addr, request_packet);
            }
            Err(e) => {
                if e.kind() != io::ErrorKind::WouldBlock {
                    println!("Error recieving packet: {:?}", e)
                }
            }
        }

        game.game_tick();
        thread::sleep(Duration::from_millis(10));
    }
}
