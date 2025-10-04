use godot::classes::{Resource, Time};
use godot::prelude::*;

mod dialogues;
mod npc;
mod player;
mod spinnn;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Resource, internal)]
struct Loaded {}

#[godot_api]
impl IResource for Loaded {
    fn init(_base: Base<Resource>) -> Self {
        let now = Time::get_time_string_from_system(&Time::singleton());

        godot_print!("{now} | Rust extension was loaded successfully!");

        Self {}
    }
}
