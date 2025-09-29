use std::f64::consts::PI;

use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Spinner {
    #[export]
    speed: f64,
    angular_speed: f64,

    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Spinner {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            speed: 400.0,
            angular_speed: PI,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);

        let rotation = self.base().get_rotation();
        let velocity = Vector2::UP.rotated(rotation) * self.speed as f32;
        self.base_mut().translate(velocity * delta as f32);
    }
}

#[godot_api]
impl Spinner {
    #[func]
    fn increase_speed(&mut self, amount: f64) {
        self.speed += amount;
        self.signals().speed_increased().emit();
    }

    #[signal]
    fn speed_increased();
}
