use godot::prelude::*;

struct PongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PongExtension {}
