use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// English sound kinds
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum SoundKind {
    Ptk,
    Th,
    W,
    V,
    Ng,
    Ch,
    Dj,
    Undefined,
}

/// Struct of the sound
#[derive(Debug, Serialize, Deserialize)]
pub struct Sound {
    #[allow(dead_code)]
    id: Uuid,
    kind: SoundKind,
    text: String,
}

impl PartialEq for Sound {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.text == other.text
    }
}

impl Sound {
    /// Creates new Sound
    pub fn new(kind: SoundKind, text: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            text,
        }
    }

    /// Returns text of the sound
    pub fn text(&self) -> &String {
        &self.text
    }
}
