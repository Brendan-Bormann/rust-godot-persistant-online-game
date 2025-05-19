use godot::prelude::*;

mod network_node;
mod player_object;

struct GDNetwork;

#[gdextension]
unsafe impl ExtensionLibrary for GDNetwork {}
