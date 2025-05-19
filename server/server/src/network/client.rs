use std::{
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket},
    sync::{Arc, mpsc},
    thread,
};

use shared::game::game_state::GameState;
use tokio::sync::watch;
use tracing::{info, warn};

use crate::game::command::Command;

use super::session::Session;

pub struct ClientManager {
    tcp_listener: TcpListener,
    udp_socket: Arc<UdpSocket>,
    session_id: u32,
    state_watch_rx: watch::Receiver<GameState>,
    cmd_tx: mpsc::Sender<Command>,
}

impl ClientManager {
    pub fn new(
        tcp_listener: TcpListener,
        udp_socket: Arc<UdpSocket>,
        state_watch_rx: watch::Receiver<GameState>,
        cmd_tx: mpsc::Sender<Command>,
    ) -> Self {
        ClientManager {
            tcp_listener,
            udp_socket,
            session_id: 0,
            state_watch_rx,
            cmd_tx,
        }
    }
}

impl ClientManager {
    pub fn start(&mut self) {
        info!("ClientManager started");

        loop {
            match self.tcp_listener.accept() {
                Ok((stream, origin)) => {
                    self.accept_connection(stream, origin);
                }
                Err(e) => {
                    warn!("Error while accepting tcp stream: {e}")
                }
            }
        }
    }

    fn next_session_id(&mut self) -> u32 {
        self.session_id += 1;
        self.session_id
    }

    pub fn accept_connection(&mut self, tcp_stream: TcpStream, origin: SocketAddr) {
        let mut session = Session::new(
            self.next_session_id(),
            tcp_stream,
            self.udp_socket.clone(),
            origin,
            self.state_watch_rx.clone(),
            self.cmd_tx.clone(),
        );
        thread::spawn(move || {
            let _ = session.start();
        });
    }
}
