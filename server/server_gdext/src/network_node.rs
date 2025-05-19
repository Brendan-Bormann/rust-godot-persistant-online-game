use std::collections::HashMap;
use std::os::unix::net::SocketAddr;

use crate::network_manager::NetworkManager;

use godot::classes::{INode, Node};
use godot::prelude::*;
use shared::game::vector::Vector3;
use shared::network::packet::Packet;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct NetworkNode {
    network_manager: NetworkManager,
    players: HashMap<String, Vector3>,
}

#[godot_api]
impl INode for NetworkNode {
    fn init(_base: Base<Node>) -> Self {
        godot_print!("SERVER_GD: Network Node Initialized.");

        Self {
            network_manager: NetworkManager::new(),
            players: HashMap::new(),
        }
    }

    fn physics_process(&mut self, _delta: f64) {}

    fn process(&mut self, _delta: f64) {
        let packets = self.network_manager.recv_all_packets();

        if packets.len() > 0 {
            godot_print!("found {} packets", packets.len());
        }

        for (packet, addr) in packets {
            self.handle_packet(&packet, addr);
        }
    }
}

#[godot_api]
impl NetworkNode {
    #[func]
    fn start(&mut self) {}

    #[func]
    fn sync_entity_position(
        &mut self,
        id: GString,
        position_x: f32,
        position_y: f32,
        position_z: f32,
    ) -> bool {
        let player = self.players.get_mut(&id.to_string());

        if player.is_none() {
            return false;
        }

        let player = player.unwrap();

        player.x = position_x;
        player.y = position_y;
        player.z = position_z;

        return true;
    }

    // fn add_player(&mut) {}

    // fn remove_player(&mut) {}

    fn handle_packet(&mut self, packet: &Packet, addr: SocketAddr) {
        godot_print!("GODOT: Got a packet!");
        let response = Packet::respond_to(packet, true, None);
        self.network_manager.send_packet(&response, &addr);
    }
}
