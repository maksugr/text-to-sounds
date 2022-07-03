use crate::sound::Sound;

/// Serialize sounds to text
///
/// ## Example
///
/// ```rust
/// use text_to_sounds::{serialize, SoundKind, Sound};
///
/// let sounds = vec![
///     Sound::new(SoundKind::Th, String::from("Th")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
///     Sound::new(SoundKind::Undefined, String::from("x")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Dj, String::from("j")),
///     Sound::new(SoundKind::Undefined, String::from("u")),
///     Sound::new(SoundKind::Undefined, String::from("s")),
///     Sound::new(SoundKind::Ptk, String::from("t")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Undefined, String::from("i")),
///     Sound::new(SoundKind::Undefined, String::from("n")),
///     Sound::new(SoundKind::Undefined, String::from(" ")),
///     Sound::new(SoundKind::Ptk, String::from("c")),
///     Sound::new(SoundKind::Undefined, String::from("a")),
///     Sound::new(SoundKind::Undefined, String::from("s")),
///     Sound::new(SoundKind::Undefined, String::from("e")),
/// ];
///
/// assert_eq!(serialize(sounds), "The text just in case");
/// ```
pub fn serialize(sounds: Vec<Sound>) -> String {
    if sounds.is_empty() {
        return String::from("");
    }

    sounds
        .iter()
        .fold(String::new(), |string, sound| string + sound.text())
}

#[cfg(test)]
mod serialize {
    use super::{serialize, Sound};
    use crate::sound::SoundKind;

    #[test]
    fn it_should_serialize_empty() {
        assert_eq!(serialize(Vec::new()), String::from(""));
    }

    #[test]
    fn it_should_serialize_sounds() {
        let sounds = vec![
            Sound::new(SoundKind::Th, String::from("Th")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("n")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("P")),
            Sound::new(SoundKind::Undefined, String::from("u")),
            Sound::new(SoundKind::Ptk, String::from("T")),
            Sound::new(SoundKind::Undefined, String::from(" ")),
            Sound::new(SoundKind::Ptk, String::from("t")),
            Sound::new(SoundKind::Undefined, String::from("O")),
            Sound::new(SoundKind::Undefined, String::from("g")),
            Sound::new(SoundKind::Undefined, String::from("E")),
            Sound::new(SoundKind::Th, String::from("TH")),
            Sound::new(SoundKind::Undefined, String::from("e")),
            Sound::new(SoundKind::Undefined, String::from("r")),
        ];

        assert_eq!(serialize(sounds), "Then PuT tOgETHer");
    }
}
