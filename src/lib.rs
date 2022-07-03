//! # text-to-sounds
//!
//! Zero dependencies text-to-sounds parsing tool.
//!
//! ## Overview
//!
//! The library has methods (`parse`, `serialize`) to parse text (`AsRef<str>`) to `Vec<Sound>` and serialize `Vec<Sound>` to `String`. `Sound` struct has information about English sound.
//!
//! ```rust
//! // English sound kinds
//! enum SoundKind {
//!     Ptk,
//!     Th,
//!     W,
//!     V,
//!     Ng,
//!     Ch,
//!     Dj,
//!     Undefined,
//! }
//!
//! // Struct of the sound
//! pub struct Sound {
//!     kind: SoundKind,
//!     text: String,
//! }
//! ```
//!
//! ## Installation
//!
//! In order to use this crate, you have to add it under `[dependencies]` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! text-to-sounds = "0.1.4"
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use text_to_sounds::{parse, serialize, SoundKind, Sound};
//!
//! let sounds = vec![
//!     Sound::new(SoundKind::Th, String::from("Th")),
//!     Sound::new(SoundKind::Undefined, String::from("e")),
//!     Sound::new(SoundKind::Undefined, String::from(" ")),
//!     Sound::new(SoundKind::Ptk, String::from("t")),
//!     Sound::new(SoundKind::Undefined, String::from("e")),
//!     Sound::new(SoundKind::Undefined, String::from("x")),
//!     Sound::new(SoundKind::Ptk, String::from("t")),
//!     Sound::new(SoundKind::Undefined, String::from(" ")),
//!     Sound::new(SoundKind::Dj, String::from("j")),
//!     Sound::new(SoundKind::Undefined, String::from("u")),
//!     Sound::new(SoundKind::Undefined, String::from("s")),
//!     Sound::new(SoundKind::Ptk, String::from("t")),
//!     Sound::new(SoundKind::Undefined, String::from(" ")),
//!     Sound::new(SoundKind::Undefined, String::from("i")),
//!     Sound::new(SoundKind::Undefined, String::from("n")),
//!     Sound::new(SoundKind::Undefined, String::from(" ")),
//!     Sound::new(SoundKind::Ptk, String::from("c")),
//!     Sound::new(SoundKind::Undefined, String::from("a")),
//!     Sound::new(SoundKind::Undefined, String::from("s")),
//!     Sound::new(SoundKind::Undefined, String::from("e")),
//! ];
//!
//! // parse
//! assert_eq!(parse("The text just in case"), sounds);
//!
//! // serialize
//! assert_eq!(serialize(sounds), "The text just in case");
//! ```
//!
//! Also, you can consider tests inside the files.

mod parser;
mod scanner;
mod serializer;
mod sound;

pub use crate::parser::parse;
pub use crate::serializer::serialize;
pub use crate::sound::{Sound, SoundKind};
