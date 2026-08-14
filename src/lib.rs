mod ball;
mod player;
mod game;

use godot::prelude::*;

struct PongExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PongExtension {}
