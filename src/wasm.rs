use crate::parse;
use crate::serialize;
use crate::Sound;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const SOUND_KIND_ENUM: &'static str = r#"
export declare enum SoundKindEnum {
    PTK = 'Ptk',
    TH = 'Th',
    W = 'W',
    V = 'V',
    NG = 'Ng',
    CH = 'Ch',
    DJ = 'Dj',
    UNDEFINED = 'Undefined'
}
"#;

#[wasm_bindgen(typescript_custom_section)]
const SOUND_ARRAY: &'static str = r#"
export declare interface ISound {
    readonly id: string;
    readonly kind: SoundKindEnum;
    readonly text: string;
}
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "Array<ISound>")]
    pub type SoundArray;
}

/// Parse text to sounds (wasm)
///
/// Currently it's not possible to define exact returned type
/// And it defined as `any`
/// https://github.com/rustwasm/wasm-bindgen/issues/1591
/// But the exact type is Array<Sound> (see signature of the `serialize_wasm` function)
/// Also consider example below to get more information
///
/// ## Example
///
/// ```js
/// import {parse_wasm} from "text-to-sounds";
///
/// const sounds_from_text = parse_wasm("The text just in case");
/// const target_sounds = [
///    {
///        "id": "96286ace-3209-4db5-a4b9-5f59e3aa48db",
///        "kind": "Th",
///        "text": "Th"
///    },
///    {
///        "id": "581a0df7-b186-4ee0-bc77-87e097a211a4",
///        "kind": "Undefined",
///        "text": "e"
///    },
///    {
///        "id": "fbe03258-b1a2-4cb1-95ea-d86fc0af15dc",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "fe613f4e-d94c-442d-97c7-043861ed1eaa",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "d0955b71-27d9-4db9-9fa6-2d0a9ed382e9",
///        "kind": "Undefined",
///        "text": "e"
///    },
///    {
///        "id": "a4694dae-992f-4d4b-b8a9-c69cb204b515",
///        "kind": "Undefined",
///        "text": "x"
///    },
///    {
///        "id": "a856290f-2f98-4235-879b-b73971822873",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "b6c92eae-5d9c-4f82-b7b6-a3ec784d70c6",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "c02e3bb9-39f8-478f-8812-bb2aeebffadd",
///        "kind": "Dj",
///        "text": "j"
///    },
///    {
///        "id": "f28a0cf2-7502-41c2-8605-9572c057ad5c",
///        "kind": "Undefined",
///        "text": "u"
///    },
///    {
///        "id": "8141c8ee-88fe-47a7-9996-3091932232de",
///        "kind": "Undefined",
///        "text": "s"
///    },
///    {
///        "id": "f97943fe-4c28-4650-9136-4f88a127e58d",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "c7e1a64f-e69e-4b58-9541-57db30556c48",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "76d84df7-871b-483c-b881-3d83aac54712",
///        "kind": "Undefined",
///        "text": "i"
///    },
///    {
///        "id": "7a04b7fd-93f6-4280-b35c-6ba74860d179",
///        "kind": "Undefined",
///        "text": "n"
///    },
///    {
///        "id": "6977fd36-9853-4196-ba13-6fc339178a14",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "98fe1a9f-1b29-4544-8a39-c411441dbdd5",
///        "kind": "Ptk",
///        "text": "c"
///    },
///    {
///        "id": "a72ef6d2-eb3c-477c-8d2e-6e73098f1446",
///        "kind": "Undefined",
///        "text": "a"
///    },
///    {
///        "id": "35ea88ed-a12e-4bc6-b553-1ee45798a157",
///        "kind": "Undefined",
///        "text": "s"
///    },
///    {
///        "id": "c86762e5-5b39-4f3b-bc25-b009a6025434",
///        "kind": "Undefined",
///        "text": "e"
///    }
/// ];
///
/// console.log(sounds_from_text.every((sound, index) =>
///     sound.kind === target_sounds[index].kind &&
///     sound.text === target_sounds[index].text
/// )); // true
/// ```
#[wasm_bindgen]
pub fn parse_wasm(text: &str) -> JsValue {
    JsValue::from_serde(&parse(text)).unwrap()
}

/// Serialize sounds to text (wasm)
///
/// ## Example
///
/// ```js
/// import {serialize_wasm} from "text-to-sounds";
///
/// const sounds = [
///    {
///        "id": "96286ace-3209-4db5-a4b9-5f59e3aa48db",
///        "kind": "Th",
///        "text": "Th"
///    },
///    {
///        "id": "581a0df7-b186-4ee0-bc77-87e097a211a4",
///        "kind": "Undefined",
///        "text": "e"
///    },
///    {
///        "id": "fbe03258-b1a2-4cb1-95ea-d86fc0af15dc",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "fe613f4e-d94c-442d-97c7-043861ed1eaa",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "d0955b71-27d9-4db9-9fa6-2d0a9ed382e9",
///        "kind": "Undefined",
///        "text": "e"
///    },
///    {
///        "id": "a4694dae-992f-4d4b-b8a9-c69cb204b515",
///        "kind": "Undefined",
///        "text": "x"
///    },
///    {
///        "id": "a856290f-2f98-4235-879b-b73971822873",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "b6c92eae-5d9c-4f82-b7b6-a3ec784d70c6",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "c02e3bb9-39f8-478f-8812-bb2aeebffadd",
///        "kind": "Dj",
///        "text": "j"
///    },
///    {
///        "id": "f28a0cf2-7502-41c2-8605-9572c057ad5c",
///        "kind": "Undefined",
///        "text": "u"
///    },
///    {
///        "id": "8141c8ee-88fe-47a7-9996-3091932232de",
///        "kind": "Undefined",
///        "text": "s"
///    },
///    {
///        "id": "f97943fe-4c28-4650-9136-4f88a127e58d",
///        "kind": "Ptk",
///        "text": "t"
///    },
///    {
///        "id": "c7e1a64f-e69e-4b58-9541-57db30556c48",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "76d84df7-871b-483c-b881-3d83aac54712",
///        "kind": "Undefined",
///        "text": "i"
///    },
///    {
///        "id": "7a04b7fd-93f6-4280-b35c-6ba74860d179",
///        "kind": "Undefined",
///        "text": "n"
///    },
///    {
///        "id": "6977fd36-9853-4196-ba13-6fc339178a14",
///        "kind": "Undefined",
///        "text": " "
///    },
///    {
///        "id": "98fe1a9f-1b29-4544-8a39-c411441dbdd5",
///        "kind": "Ptk",
///        "text": "c"
///    },
///    {
///        "id": "a72ef6d2-eb3c-477c-8d2e-6e73098f1446",
///        "kind": "Undefined",
///        "text": "a"
///    },
///    {
///        "id": "35ea88ed-a12e-4bc6-b553-1ee45798a157",
///        "kind": "Undefined",
///        "text": "s"
///    },
///    {
///        "id": "c86762e5-5b39-4f3b-bc25-b009a6025434",
///        "kind": "Undefined",
///        "text": "e"
///    }
/// ];
/// const text_from_sounds = serialize_wasm(sounds);
///
/// console.log(text_from_sounds === "The text just in case"); // true
/// ```
#[wasm_bindgen]
pub fn serialize_wasm(sounds: &SoundArray) -> String {
    serialize(sounds.into_serde::<Vec<Sound>>().unwrap())
}
