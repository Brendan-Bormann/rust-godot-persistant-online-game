use godot::prelude::*;

mod network;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
