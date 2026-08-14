use godot::classes::{CharacterBody2D, ICharacterBody2D, RandomNumberGenerator, Timer};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Ball {
    #[export]
    ball_speed: f32,
    direction: Vector2,
    wait_to_start: bool,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Ball {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            ball_speed: 600.0,
            direction: Vector2::ZERO,
            wait_to_start: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.reset_ball();
    }

    fn physics_process(&mut self, delta: f64) {
        if self.wait_to_start {
            return;
        }

        let movement = self.direction * self.ball_speed * delta as f32;
        let mut collided = false;
        let mut normal = Vector2::ZERO;

        {
            // internal block to un fuck borrow checker
            let mut base = self.base_mut();
            if let Some(collision) = base.move_and_collide(movement) {
                collided = true;
                normal = collision.get_normal();
            }
        }

        if collided {
            self.direction = self.direction.bounce(normal);
        }
    }
}

#[godot_api]
impl Ball{
    #[func]
    pub fn reset_ball(&mut self) {
        self.wait_to_start = true;
        let mut view_size = Vector2::new(1080.0, 720.0);

        {
            let base = self.base();
            if let Some(view) = base.get_viewport() {
                view_size = view.get_visible_rect().size;
            }
        }

        {
            let mut base = self.base_mut();
            base.set_position(Vector2::new(view_size.x / 2.0, view_size.y / 2.0));
        }

        let mut rng = RandomNumberGenerator::new_gd();
        rng.randomize();
        let x_dir = if rng.randf() > 0.5 { 1.0 } else { -1.0 };
        let y_dir = rng.randf_range(-0.5, 0.5);
        self.direction = Vector2::new(x_dir, y_dir as f32).normalized();

        let base = self.base();
        if let Some(mut timer) = base.try_get_node_as::<Timer>("Timer") {
            timer.start();
        } else {
            godot_error!("Error: `Timer` child node not found relative to Parent - {:?}", *base)
        }
    }

    #[func]
    pub fn _on_timer_timeout(&mut self) {
        self.wait_to_start = false;
    }
}