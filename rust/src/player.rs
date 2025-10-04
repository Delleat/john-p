use godot::classes::{CharacterBody2D, ICharacterBody2D, Input};
use godot::global::Key;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export_group(name = "Movement")]
    #[export]
    move_speed: f32,
    #[export]
    jump_velocity: f32,
    #[export]
    jump_fix_time: f32,

    jump_fix_timer: f32,
    nodes_to_interact: Vec<Gd<Node>>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl Player {
    #[func]
    fn entered_interact_area(&mut self, node: Gd<Node>) {
        self.nodes_to_interact.push(node);
    }

    #[func]
    fn exited_interact_area(&mut self, node: Gd<Node>) {
        self.nodes_to_interact.retain(|f| *f != node);
    }
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            move_speed: 300.0,
            jump_velocity: 400.0,
            jump_fix_time: 1.0,

            jump_fix_timer: 0.0,
            nodes_to_interact: vec![],

            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();
        let move_speed = self.get_move_speed();

        // ---Movement---
        let vel = self.base().get_velocity();
        let direction = input.get_axis("move_left", "move_right");

        if direction != 0.0 {
            self.base_mut().set_velocity(Vector2 {
                x: move_speed * direction,
                y: vel.y,
            });
        } else {
            self.base_mut().set_velocity(Vector2 { x: 0.0, y: vel.y });
        }

        // ---Jump---
        let vel = self.base().get_velocity();
        if input.is_action_just_pressed("jump")
            && (self.base().is_on_floor() || self.jump_fix_timer >= 0.0)
        {
            let jump_speed = self.get_jump_velocity();
            let up = self.base().get_up_direction() * jump_speed;
            self.base_mut().set_velocity(vel + up);
        }

        // ---Jump fix---
        let grounded = self.base().is_on_floor();
        if grounded {
            self.jump_fix_timer = self.get_jump_fix_time();
        } else if !grounded && self.jump_fix_timer > 0.0 {
            self.jump_fix_timer -= 10.0 * delta as f32;
        }

        // ---Rotation---
        if input.is_key_pressed(Key::F) {
            self.base_mut().rotate(19.0);
            self.base_mut().set_up_direction(Vector2 { x: 1.0, y: 0.0 });
        }

        // ---Interaction---
        if input.is_action_just_pressed("interact") {
            for node in &mut self.nodes_to_interact {
                if let Some(mut node) = node.get_parent() {
                    if let Err(err) = node.try_call("interact", &[]) {
                        godot_print!("Error: {:?}", err);
                    }
                }
            }
        }

        // ---Gravity---
        let vel = self.base().get_velocity();
        if !self.base().is_on_floor() {
            let up = self.base().get_up_direction();
            let up = up.x + up.y;
            let grav = self.base().get_gravity() * up * delta as f32;
            self.base_mut().set_velocity(vel - grav);
        }

        self.base_mut().move_and_slide();

        // ---Debug---
        if input.is_key_pressed(Key::ESCAPE) {
            let mut t = self.base().get_tree().unwrap();
            t.quit();
        }

        if input.is_key_pressed(Key::R) {
            let mut t = self.base().get_tree().unwrap();
            t.reload_current_scene();
        }
    }
}
