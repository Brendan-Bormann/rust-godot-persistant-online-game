use crate::packet::Packet;
use crate::rust_network::RustNetwork;

use godot::classes::{INode, Node};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkNode {
    rust_network: RustNetwork,
}

#[godot_api]
impl INode for NetworkNode {
    fn init(_base: Base<Node>) -> Self {
        godot_print!("RUST_NETWORK: NetworkNode Initialized.");

        Self {
            rust_network: RustNetwork::new(),
        }
    }
}

#[godot_api]
impl NetworkNode {
    #[func]
    fn start_server(&mut self, server_ip: String) -> bool {
        if let Ok(_) = self.rust_network.start(server_ip) {
            return true;
        }

        false
    }

    #[func]
    fn stop_server(&mut self) {
        self.rust_network.stop_server()
    }

    #[func]
    fn is_active(&mut self) -> bool {
        self.rust_network.is_active()
    }

    #[func]
    fn send_packet(&mut self, packet_type: i16, packet_subtype: i16, payload: String) -> bool {
        let new_packet: Packet = Packet::new(packet_type, packet_subtype, payload);
        match self.rust_network.send_packet(new_packet) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    #[func]
    fn read_packet(&mut self) -> [GString; 3] {
        match self.rust_network.recv_packet() {
            Some(packet) => [
                packet.packet_type.to_string().into(),
                packet.packet_subtype.to_string().into(),
                packet.payload.into(),
            ],
            None => [
                (-1).to_string().into(),
                (-1).to_string().into(),
                "".to_string().into(),
            ],
        }
    }
}
