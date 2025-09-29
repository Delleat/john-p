use godot::classes::{AnimatedSprite2D, CharacterBody2D, CollisionShape2D, ICharacterBody2D, SpriteFrames};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct Player {
    #[export_group(name="Movement")]
    #[export]
    move_speed: f32,

    #[export_group(name="Sprite")]
    #[export]
    sprite_frames: Option<Gd<SpriteFrames>>,
    #[export]
    default_animation: GString,

    #[export_group(name="")]
    #[export]
    collider: Option<Gd<CollisionShape2D>>,

    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Self {
            move_speed: 300.0,

            sprite_frames: None,
            default_animation: GString::new(),
            
            collider: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        let coll = self.collider.clone().unwrap_or(CollisionShape2D::new_alloc());
        self.base_mut().add_child(&coll);

        let sprite_frames = self.sprite_frames.clone().unwrap();
        let mut sprite = AnimatedSprite2D::new_alloc();
        sprite.set_sprite_frames(&sprite_frames);
        sprite.set_animation(&self.default_animation.to_string());
        self.base_mut().add_child(&sprite);
    }
}
