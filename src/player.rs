use godot::classes::{CharacterBody2D, ICharacterBody2D};
use godot::prelude::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, GodotConvert, Var)]
#[godot(via = GString)]
enum Direction {
    Left,
    Rright,
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
    fn init(base: Bases<CharacterBody2D>) -> Self {
        Self {
            paddle_side: Left,
            paddle_speed: 400.0,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // First get input direciton and handle movement/deceleration

        let input = Input::get_singleton();
        let mut movement = 0.0;

        let (up_action, down_action) = match self.paddle_side {
            Direction::Left => ("move_p1_up", "move_p1_down"),
            Direction::Right => ("move_p2_up", "move_p2_down"),
        };

        if input.is_action_pressed(up_action.into()) {
            movement += 1.0;
        }
        if input.is_action_pressed(down_action.into()) {
            movement += -1.0;
        }

        let body = self.base_mut();

        let target = Vector2::new(0.0, movement * self.speed);

        body.set_velocity(target);
        body.move_and_slide();
    }
}

/*
func _physics_process(delta):
    # Get the input direction and handle the movement/deceleration.
    # As good practice, you should replace UI actions with custom gameplay actions.
    var direction
    if paddle_side == "L":
        direction = Input.get_axis("left_paddle_up", "left_paddle_down")
    elif paddle_side == "R":
        direction = Input.get_axis("right_paddle_up", "right_paddle_down")
    if direction:
        velocity.y = direction * SPEED
    else:
        velocity.y = move_toward(velocity.y, 0, SPEED)

    move_and_slide()
 */
