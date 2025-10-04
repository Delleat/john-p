use godot::classes::{AnimatableBody2D, Control, IAnimatableBody2D, Label};
use godot::prelude::*;

use crate::dialogues::Dialogues;

#[derive(GodotClass)]
#[class(base=AnimatableBody2D)]
struct Npc {
    #[export]
    dialogues: Option<Gd<Dialogues>>,

    interacted: bool,
    dialogue_counter: isize,
    active_dialogue: Vec<GString>,

    dialogue_box: Option<Gd<Control>>,
    dialogue_label: Option<Gd<Label>>,

    base: Base<AnimatableBody2D>,
}

#[godot_api]
impl Npc {
    #[func]
    fn interact(&mut self) {
        let dialogue_box = self.dialogue_box.as_mut().unwrap();
        let dialogue_label = self.dialogue_label.as_mut().unwrap();

        if let Some(dialogues) = &mut self.dialogues {
            let ad = &mut self.active_dialogue;

            let current = if ad.is_empty() {
                *ad = if !self.interacted {
                    dialogues.call("get_dialogues_for", &["interact".to_string().to_variant()])
                } else {
                    dialogues.call(
                        "get_dialogues_for",
                        &["interact_again".to_string().to_variant()],
                    )
                }
                .to::<Vec<GString>>();

                dialogue_box.set_visible(false);
                self.dialogue_counter += 1;
                return;
            } else {
                dialogue_box.set_visible(true);
                &ad[self.dialogue_counter as usize]
            };

            godot_print!("dialogues: '{:?}'", current);

            dialogue_label.set_text(current);

            self.dialogue_counter += 1;

            if !ad.is_empty() && self.dialogue_counter >= ad.len() as isize {
                self.interacted = true;
                self.dialogue_counter = -1;
                *ad = vec![];
            }
        }
    }
}

#[godot_api]
impl IAnimatableBody2D for Npc {
    fn init(base: Base<AnimatableBody2D>) -> Self {
        Self {
            dialogues: None,

            interacted: false,
            dialogue_counter: -1,
            active_dialogue: vec![],

            dialogue_box: None,
            dialogue_label: None,

            base,
        }
    }

    fn enter_tree(&mut self) {
        self.dialogue_box = Some(
            self.base()
                .find_child("DialogueBox")
                .unwrap()
                .cast::<Control>(),
        );
        self.dialogue_label = Some(self.base().find_child("Label").unwrap().cast::<Label>());

        self.dialogue_box.as_mut().unwrap().set_visible(false);
    }
}
