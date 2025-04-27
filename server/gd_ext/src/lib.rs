use godot::prelude::*;

mod client_network;
mod network_node;

struct GDNetwork;

#[gdextension]
unsafe impl ExtensionLibrary for GDNetwork {}
