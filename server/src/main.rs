mod game;
mod network;
mod storage;

use crate::game::game::Game;
use std::io;
use std::net::UdpSocket;
use std::{thread, time::Duration};

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    socket.set_nonblocking(true).unwrap();

    let _game_thread = thread::spawn(move || {
        let mut game = Game::new();

        loop {
            game.game_tick();
            thread::sleep(Duration::from_millis(10));
        }
    });

    loop {
        let mut game = Game::new();

        loop {
            let mut buf = [0; 10000];
            match socket.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    let message_string = String::from_utf8_lossy(&buf[..len]);
                    network::process_packet(
                        &socket,
                        &mut game,
                        addr,
                        len,
                        message_string.trim().into(),
                    );
                }
                Err(e) => {
                    if e.kind() != io::ErrorKind::WouldBlock {
                        println!("Error recieving packet: {:?}", e)
                    }
                }
            }
        }
    }
}
