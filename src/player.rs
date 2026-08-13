use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player;

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Bases<CharacterBody2D>) -> Self {
        println!("This is a paddle");
    }
}
