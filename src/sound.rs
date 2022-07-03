/// English sound kinds
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct Sound {
    kind: SoundKind,
    text: String,
}

impl Sound {
    /// Creates new Sound
    pub fn new(kind: SoundKind, text: String) -> Self {
        Self { kind, text }
    }

    /// Returns text of the sound
    pub fn text(&self) -> &String {
        &self.text
    }
}
