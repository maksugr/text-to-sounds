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

Text-to-sounds parsing tool. Used in [Spoken Sounds Highlighter](https://spokensoundshighlighter.com/).

![Website screenshot](/www/assets/website.png)

## Overview

The library has functions (`parse`, `serialize`) to parse text (`AsRef<str>`) to `Vec<Sound>` and serialize `Vec<Sound>` to `String`. `Sound` struct has information about English sound. `highlight` function adds `html` tags to text that can be used to highlight sounds in the browser via `css`.

> If you are interested in a version for JavaScript (WASM), move below to the `Javascript / WASM` section.

```rust
use uuid::Uuid;

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
    id: Uuid,
    kind: SoundKind,
    text: String,
}
```

## Installation

In order to use this crate, you have to add it under `[dependencies]` to your `Cargo.toml`:

```toml
[dependencies]
text-to-sounds = "1.1.1"
```

## Javascript / WASM

In `www` directory you can find the source code of the website [Spoken Sounds Highlighter](https://spokensoundshighlighter.com/). It uses [wasm version](https://www.npmjs.com/package/text-to-sounds) of the `highlight` function. You can get it too from `npm`:

```sh
npm i --save text-to-sounds
```

And use:

```js
import {highlight_wasm} from "text-to-sounds";

// example #1
// "<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase"
const highlightedText = highlight_wasm("The text just in case");


// example #2
const contenteditableEl = document.getElementById('contenteditable');
contenteditableEl.innerHTML = highlight_wasm(contenteditableEl.textContent);
```

Consider adding some css styles for these classes and we are done:

```css
.Ptk, .Th, .W, .V, .Ng, .Ch, .Dj {
    font-weight: 700;
}

.Ptk {
    color: #7F7EFF;
}

.Th {
    color: #A390E4;
}

.W {
    color: #C69DD2;
}

.V {
    color: #CC8B8C;
}

.Ng {
    color: #C68866;
}

.Ch {
    color: #417B5A;
}

.Dj {
    color: #4B3F72;
}
```

You can find a workable example in the `www` directory in the source code of the [Spoken Sounds Highlighter](https://spokensoundshighlighter.com/) website.

## Examples

```rust
use text_to_sounds::{parse, serialize, highlight, SoundKind, Sound};

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

// highlight
assert_eq!(highlight("The text just in case"), "<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase".to_string());
```

Also, you can consider tests inside the files.
