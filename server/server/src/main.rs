mod game;
mod network;
mod storage;

use game::game_manager::GameManager;
use network::client::ClientManager;

use std::{
    net::{TcpListener, UdpSocket},
    sync::Arc,
    thread,
};
use tracing_subscriber;

const CLIENT_PORT: &str = "8080";

fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .compact()
        .init();

    let (mut game_manager, state_watch_rx, cmd_tx) = GameManager::new();

    let _game = thread::spawn(move || {
        let _ = game_manager.start();
    });

    let client_udp = Arc::new(UdpSocket::bind(format!("0.0.0.0:{CLIENT_PORT}")).unwrap());
    let client_tcp = TcpListener::bind(format!("0.0.0.0:{CLIENT_PORT}")).unwrap();

    let _network = thread::spawn(move || {
        let mut client_manager =
            ClientManager::new(client_tcp, client_udp.clone(), state_watch_rx, cmd_tx);
        let _ = client_manager.start();
    });

    loop {}
}
