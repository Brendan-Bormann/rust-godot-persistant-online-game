use godot::prelude::*;

mod network_manager;
mod network_node;

struct GDNetwork;

#[gdextension]
unsafe impl ExtensionLibrary for GDNetwork {}
