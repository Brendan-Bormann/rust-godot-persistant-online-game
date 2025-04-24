mod game;
mod network;
mod storage;

use crate::game::game::Game;
use game::physics::PhysicsManager;
use network::Network;
use storage::mem_db::MemDB;

use r2d2;
use std::io;
use std::net::UdpSocket;
use std::{
    thread,
    time::{Duration, Instant},
};

// ticks per second
const TICK_RATE: u64 = 20;
const MS_PER_TICK: u64 = 1000 / TICK_RATE;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8080").unwrap();
    socket.set_nonblocking(true).unwrap();

    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let pool = r2d2::Pool::builder().build(client).unwrap();

    let game_mem_db = MemDB::new(pool.get().unwrap());
    let _game = thread::spawn(move || {
        let pm = PhysicsManager::new();
        let mut game = Game::new(game_mem_db, pm);

        let tick_duration = Duration::from_millis(MS_PER_TICK);
        loop {
            let start = Instant::now();

            game.game_tick(MS_PER_TICK as f32 / 1000.0);

            let elapsed = start.elapsed();
            if elapsed < tick_duration {
                thread::sleep(tick_duration - elapsed);
            }
        }
    });

    let network_mem_db = MemDB::new(pool.get().unwrap());
    loop {
        let mut network = Network::new(network_mem_db);

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
