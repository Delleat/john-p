use godot::classes::{AnimatableBody2D, IAnimatableBody2D};
use godot::prelude::*;

use crate::dialogues::Dialogues;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
struct Npc {
    #[export]
    dialogues: Option<Gd<Dialogues>>,

    interaction_counter: u32,

    base: Base<AnimatableBody2D>,
}

#[godot_api]
impl Npc {
    #[func]
    fn interact(&mut self) {
        if let Some(dialogues) = &mut self.dialogues {
            let f = if self.interaction_counter == 0 {
                dialogues.call("get_dialogues_for", &["interact".to_string().to_variant()])
            } else {
                dialogues.call("get_dialogues_for", &["interact_again".to_string().to_variant()])
            };

            godot_print!("dialogues: '{:?}'", f);
        }
        self.interaction_counter += 1;
    }
}

#[godot_api]
impl IAnimatableBody2D for Npc {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Self {
            dialogues: None,
            interaction_counter: 0,
            base,
        }
    }
}
