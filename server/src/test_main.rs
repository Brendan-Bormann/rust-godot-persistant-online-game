// use game::physics::PhysicsManager;
// use network::Network;
// use storage::mem_db::MemDB;

// use crate::game::game::Game;
// use std::io;
// use std::net::{TcpListener, UdpSocket};
// use std::{
//     thread,
//     time::{Duration, Instant},
// };

// // ms per tick
// const TICK_RATE: u32 = 50;

// fn main() {
//     let tcp_listener = TcpListener::bind("0.0.0.0:42111").unwrap();

//     for steam in tcp_listener.incoming() {}

//     let socket = UdpSocket::bind("0.0.0.0:42222").unwrap();
//     socket.set_nonblocking(true).unwrap();

//     let _game = thread::spawn(move || {
//         let pm = PhysicsManager::new();
//         let mem_db = MemDB::new();
//         let mut game = Game::new(mem_db, pm);

//         let tick_duration = Duration::from_secs_f32(1.0 / TPS as f32);

//         loop {
//             let start = Instant::now();

//             game.game_tick(DELTA);

//             let elapsed = start.elapsed();
//             if elapsed < tick_duration {
//                 thread::sleep(tick_duration - elapsed);
//             }
//         }
//     });

//     loop {
//         let mem_db = MemDB::new();
//         let mut network = Network::new(mem_db);

//         loop {
//             let mut buf = [0; 65507];
//             match socket.recv_from(&mut buf) {
//                 Ok((len, addr)) => {
//                     let message_string = String::from_utf8_lossy(&buf[..len]);
//                     network.process_packet(&socket, addr, len, message_string.trim().into());
//                 }
//                 Err(e) => {
//                     if e.kind() != io::ErrorKind::WouldBlock {
//                         println!("Error recieving packet: {:?}", e)
//                     }
//                 }
//             }
//         }
//     }
// }
