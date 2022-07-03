# text-to-sounds

[![build-svg]][build-url]
[![test-svg]][test-url]
[![codecov-svg]][codecov-url]
[![crates-svg]][crates-url]
[![docs-svg]][docs-url]
[![deps-svg]][deps-url]

[build-svg]: https://github.com/maksugr/text-to-sounds/workflows/build/badge.svg
[build-url]: https://github.com/maksugr/text-to-sounds/actions/workflows/build.yml
[test-svg]: https://github.com/maksugr/text-to-sounds/workflows/test/badge.svg
[test-url]: https://github.com/maksugr/text-to-sounds/actions/workflows/test.yml
[codecov-svg]: https://img.shields.io/codecov/c/github/maksugr/text-to-sounds
[codecov-url]: https://codecov.io/gh/maksugr/text-to-sounds
[crates-svg]: https://img.shields.io/crates/v/text-to-sounds.svg
[crates-url]: https://crates.io/crates/text-to-sounds
[docs-svg]: https://docs.rs/text-to-sounds/badge.svg
[docs-url]: https://docs.rs/text-to-sounds
[deps-svg]: https://deps.rs/repo/github/maksugr/text-to-sounds/status.svg
[deps-url]: https://deps.rs/repo/github/maksugr/text-to-sounds

Zero dependencies text-to-sounds parsing tool.

## Overview

The library has methods (`parse`, `serialize`) to parse text (`AsRef<str>`) to `Vec<Sound>` and serialize `Vec<Sound>` to `String`. `Sound` struct has information about English sound.

```rust
// English sound kinds
enum SoundKind {
    Ptk,
    Th,
    W,
    V,
    Ng,
    Ch,
    Dj,
    Undefined,
}

// Struct of the sound
pub struct Sound {
    kind: SoundKind,
    text: String,
}
```

## Installation

In order to use this crate, you have to add it under `[dependencies]` to your `Cargo.toml`:

```toml
[dependencies]
text-to-sounds = "0.1.2"
```

## Examples

```rust
use text_to_sounds::{parse, serialize, SoundKind, Sound};

let sounds = vec![
    Sound::new(SoundKind::Th, String::from("Th")),
    Sound::new(SoundKind::Undefined, String::from("e")),
    Sound::new(SoundKind::Undefined, String::from(" ")),
    Sound::new(SoundKind::Ptk, String::from("t")),
    Sound::new(SoundKind::Undefined, String::from("e")),
    Sound::new(SoundKind::Undefined, String::from("x")),
    Sound::new(SoundKind::Ptk, String::from("t")),
    Sound::new(SoundKind::Undefined, String::from(" ")),
    Sound::new(SoundKind::Dj, String::from("j")),
    Sound::new(SoundKind::Undefined, String::from("u")),
    Sound::new(SoundKind::Undefined, String::from("s")),
    Sound::new(SoundKind::Ptk, String::from("t")),
    Sound::new(SoundKind::Undefined, String::from(" ")),
    Sound::new(SoundKind::Undefined, String::from("i")),
    Sound::new(SoundKind::Undefined, String::from("n")),
    Sound::new(SoundKind::Undefined, String::from(" ")),
    Sound::new(SoundKind::Ptk, String::from("c")),
    Sound::new(SoundKind::Undefined, String::from("a")),
    Sound::new(SoundKind::Undefined, String::from("s")),
    Sound::new(SoundKind::Undefined, String::from("e")),
];

// parse
assert_eq!(parse("The text just in case"), sounds);

// serialize
assert_eq!(serialize(sounds), "The text just in case");
```

Also, you can consider tests inside the files.
