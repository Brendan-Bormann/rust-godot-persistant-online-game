mod game;
mod network;
mod storage;
mod test_main;

use game::physics::PhysicsManager;
use network::Network;
use storage::mem_db::MemDB;

use crate::game::game::Game;
use std::io;
use std::net::UdpSocket;
use std::{
    thread,
    time::{Duration, Instant},
};

// ticks per second
const TPS: u64 = 20;
const DELTA: f32 = 1.0 / TPS as f32;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    socket.set_nonblocking(true).unwrap();

    let _game = thread::spawn(move || {
        let pm = PhysicsManager::new();
        let mem_db = MemDB::new();
        let mut game = Game::new(mem_db, pm);

        let tick_duration = Duration::from_secs_f32(1.0 / TPS as f32);

        loop {
            let start = Instant::now();

            game.game_tick(DELTA);

            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
    });

    loop {
        let mem_db = MemDB::new();
        let mut network = Network::new(mem_db);

        loop {
            let mut buf = [0; 65507];
            match socket.recv_from(&mut buf) {
                Ok((len, addr)) => {
                    let message_string = String::from_utf8_lossy(&buf[..len]);
                    network.process_packet(&socket, addr, len, message_string.trim().into());
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
