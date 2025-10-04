use std::collections::HashMap;

use serde::Deserialize;
use toml::de;

#[derive(Debug, Deserialize, Clone)]
struct Section {
    value: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
pub struct DialogueFile {
    #[serde(flatten)]
    sections: HashMap<String, Section>,
}

impl DialogueFile {
    pub fn get(&self, what: &str) -> Option<&Vec<String>> {
        self.sections
            .get(what)
            .map(|a| &a.value)
            .and_then(|f| f.as_ref())
    }
}

pub fn parse_toml(text: &str) -> Result<DialogueFile, de::Error> {
    toml::from_str::<DialogueFile>(text)
}
