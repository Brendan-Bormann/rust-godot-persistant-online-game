#[derive(Debug, Clone)]
pub struct Packet {
    pub packet_type: i16,
    pub packet_subtype: i16,
    pub payload: String,
}

impl Packet {
    pub fn new(t: i16, s: i16, p: String) -> Packet {
        Packet {
            packet_type: t,
            packet_subtype: s,
            payload: p,
        }
    }

    pub fn encode_new(t: i16, s: i16, p: String) -> String {
        Packet::new(t, s, p).encode()
    }

    pub fn decode(packet: String) -> Packet {
        let mut parts = packet.splitn(3, '|'); // split into max 3 parts

        let packet_type = parts
            .next()
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(-1);

        let subtype = parts
            .next()
            .and_then(|s| s.parse::<i16>().ok())
            .unwrap_or(-1);

        let payload = parts.next().unwrap_or("").to_string();

        Packet::new(packet_type, subtype, payload)
    }
}

impl Packet {
    pub fn encode(&mut self) -> String {
        format!(
            "{}|{}|{}",
            self.packet_type, self.packet_subtype, self.payload
        )
    }

    pub fn parse_payload(&self) -> Vec<String> {
        self.payload.clone().split(';').map(|s| s.into()).collect()
    }
}
