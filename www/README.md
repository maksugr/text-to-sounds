# [Spoken Sounds Highlighter](https://spokensoundshighlighter.com)

Highlights English sounds in the provided text.

This project intends to be education only. It highlights English sounds in the provided text. Just it. Under the hood a couple of the not stable enough technologies like [contenteditable](https://developer.mozilla.org/en-US/docs/Web/HTML/Global_attributes/contenteditable) or [wasm](https://developer.mozilla.org/en-US/docs/WebAssembly). Hence, if you have some troubles, sorry about that, try to use the last Google Chrome. However, there are many known issues:

1. cursor carriage always moves to the end of the line.
2. there are no new lines.
3. there is a max text length limitation — 5000 characters.

This project was born in [text-to-sounds](https://crates.io/crates/text-to-sounds) crate and at the moment consists of the crate and `www` directory inside it with the source code of the website that uses compiled to wasm version of the crate and provides a user interface to it.
