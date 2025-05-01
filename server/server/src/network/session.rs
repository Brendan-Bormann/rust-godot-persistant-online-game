use std::{net::SocketAddr, time::Instant};

#[derive(Clone)]
pub struct Session {
    pub peer: SocketAddr,
    pub player_id: Option<String>,
    pub last_active: Instant,
}

impl Session {
    pub fn new(peer: SocketAddr) -> Session {
        Session {
            peer,
            player_id: None,
            last_active: Instant::now(),
        }
    }
}
