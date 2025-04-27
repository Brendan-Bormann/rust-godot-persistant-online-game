use std::net::SocketAddr;

use super::mem_db::MemDB;

pub struct Session {
    pub player_id: String,
}

impl Session {
    pub fn new(player_id: String) -> Self {
        Session { player_id }
    }

    pub fn from_string(string: String) -> Self {
        let parts: Vec<&str> = string.split("|").collect();
        Session::new(parts[0].trim().into())
    }
}

impl Session {
    pub fn to_string(&mut self) -> String {
        format!("{}|", self.player_id)
    }
}

impl MemDB {
    pub fn create_session(&mut self, addr: &SocketAddr, session: &mut Session) -> Result<(), ()> {
        match self.set("session", &addr.to_string(), &session.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn get_session(&mut self, addr: &SocketAddr) -> Result<Option<Session>, ()> {
        match self.get("player", &addr.to_string()) {
            Ok(session) => match session {
                Some(session) => Ok(Some(Session::from_string(session))),
                None => Ok(None),
            },
            Err(_) => panic!("Failed to get session!"),
        }
    }

    pub fn update_session(&mut self, addr: &SocketAddr, session: &mut Session) -> Result<(), ()> {
        match self.set("session", &addr.to_string(), &session.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }

    pub fn delete_session(&mut self, addr: &SocketAddr) -> Result<(), ()> {
        match self.del("session", &addr.to_string()) {
            Ok(_) => Ok(()),
            Err(_) => Err(()),
        }
    }
}
