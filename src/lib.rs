//! # text-to-sounds
//!
//! Text-to-sounds parsing tool.
//!
//! ## Overview
//!
//! The library has functions (`parse`, `serialize`) to parse text (`AsRef<str>`) to `Vec<Sound>` and serialize `Vec<Sound>` to `String`. `Sound` struct has information about English sound. `highlight` function adds `html` tags to text that can be used to highlight sounds in the browser via `css`.
//!
//! ```rust
//! use uuid::Uuid;
//!
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
//!     id: Uuid,
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
//! text-to-sounds = "1.1.1"
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use text_to_sounds::{parse, serialize, highlight, SoundKind, Sound};
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
//!
//! // highlight
//! assert_eq!(highlight("The text just in case"), "<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase".to_string());
//! ```
//!
//! Also, you can consider tests inside the files.

mod highlighter;
mod parser;
mod scanner;
mod serializer;
mod sound;
mod wasm;

pub use crate::highlighter::highlight;
pub use crate::parser::parse;
pub use crate::serializer::serialize;
pub use crate::sound::{Sound, SoundKind};
pub use crate::wasm::highlight_wasm;
