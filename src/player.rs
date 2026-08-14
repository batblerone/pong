use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, GodotConvert, Var, Export)]
#[godot(via = GString)]
enum Direction {
    Left,
    Right,
}

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export]
    paddle_side: Direction,
    #[export]
    paddle_speed: f64,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            paddle_side: Direction::Left,
            paddle_speed: 400.0,
            base,
        }
    }

    fn physics_process(&mut self, _delta: f64) {

        let input = Input::singleton();
        let mut movement: f32 = 0.0;

        let (up_action, down_action) = match self.paddle_side {
            Direction::Left => ("move_p1_up", "move_p1_down"),
            Direction::Right => ("move_p2_up", "move_p2_down"),
        };

        if input.is_action_pressed(up_action) {
            movement += -1.0;
        }
        if input.is_action_pressed(down_action) {
            movement += 1.0;
        }

        let target = Vector2::new(0.0, movement * self.paddle_speed as f32);

        let mut body = self.base_mut();

        body.set_velocity(target);
        body.move_and_slide();
    }
}
