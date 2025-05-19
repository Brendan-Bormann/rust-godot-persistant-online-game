use bitcode::{Decode, Encode};

#[derive(Decode, Encode, Debug, Clone, PartialEq)]
pub struct Packet {
    pub id: String,
    pub packet_type: i16,
    pub packet_subtype: i16,
    pub payload: Option<Vec<u8>>,
}

impl Packet {
    pub fn new(id: String, t: i16, s: i16, p: Option<Vec<u8>>) -> Self {
        Packet {
            id,
            packet_type: t,
            packet_subtype: s,
            payload: p,
        }
    }

    pub fn respond_to(packet: &Packet, success: bool, payload: Option<Vec<u8>>) -> Self {
        Packet::new(packet.id.clone(), 2, if success { 0 } else { -1 }, payload)
    }
}

// id - provided by client to track for a repsonse. If no id, there was no request

// type 0 - network
// - 0 ping
// - 1 heartbeat

// type 1 - auth
// - 0 init session
// - 1 kill session
// - 2 login
// - 3 logout

// type 2 - action
// - -1 cmd failure (only send to client)
// - 0 cmd success (only send to client)
// - 1 cmd create character
// - 2 input - setbearing

// type 3 - state
// - -1 set network state
// - 0 self full
// - 1 self diff
// - 2 players full
// - 3 players diff
// - 4 entities full
// - 5 entities diff
