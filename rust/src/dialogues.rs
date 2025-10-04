use godot::classes::file_access::ModeFlags;
use godot::classes::{FileAccess, Resource};
use godot::prelude::*;

mod dialogue_file;
use dialogue_file::parse_toml;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Dialogues {
    #[var(
        hint = FILE,
        hint_string = "*.toml",
    )]
    #[export]
    toml_path: GString,

    base: Base<Resource>,
}

#[godot_api]
impl Dialogues {
    #[func]
    pub fn get_dialogues_for(&mut self, what: String) -> Vec<GString> {
        let toml = &self.toml_path;
        if FileAccess::file_exists(toml) {
            if let Some(file) = FileAccess::open(toml, ModeFlags::READ) {
                let contents = file.get_as_text().to_string();
                let dialogue_file = parse_toml(&contents).unwrap_or_default();

                let dialogues = dialogue_file.get(&what);
                if let Some(lines) = dialogues {
                    return lines.iter().map(GString::from).collect();
                }

                return vec![];
            }
        }

        vec![]
    }
}

#[godot_api]
impl IResource for Dialogues {
    fn init(base: Base<Resource>) -> Self {
        Self {
            toml_path: GString::new(),
            base,
        }
    }
}
