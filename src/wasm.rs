use crate::highlight;
use wasm_bindgen::prelude::*;

/// Highlight sounds in the text with html tags (wasm)
///
/// ## Example
///
/// ```js
/// import {highlight_wasm} from "text-to-sounds";
///
/// console.log(highlight_wasm("The text just in case") === "<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase"); // true
/// ```
#[wasm_bindgen]
pub fn highlight_wasm(text: &str) -> String {
    highlight(text)
}
