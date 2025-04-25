use godot::prelude::*;

mod network_node;
mod packet;
mod rust_network;

struct Network;

#[gdextension]
unsafe impl ExtensionLibrary for Network {}
