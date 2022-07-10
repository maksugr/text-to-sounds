//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate text_to_sounds;
use text_to_sounds::highlight_wasm;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn it_should_highlight_ptk() {
    assert_eq!(highlight_wasm("Put a cat"), "<span class='Ptk'>P</span>u<span class='Ptk'>t</span> a <span class='Ptk'>c</span>a<span class='Ptk'>t</span>".to_string());
}
