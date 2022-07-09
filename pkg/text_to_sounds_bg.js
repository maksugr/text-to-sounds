import * as wasm from './text_to_sounds_bg.wasm';

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachedUint8Memory0;
function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedInt32Memory0;
function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}
/**
* Parse text to sounds (wasm)
*
* Currently it's not possible to define exact returned type
* And it defined as `any`
* https://github.com/rustwasm/wasm-bindgen/issues/1591
* But the exact type is Array<Sound> (see signature of the `serialize_wasm` function)
* Also consider example below to get more information
*
* ## Example
*
* ```js
* import {parse_wasm} from "text-to-sounds";
*
* const sounds_from_text = parse_wasm("The text just in case");
* const target_sounds = [
*    {
*        "id": "96286ace-3209-4db5-a4b9-5f59e3aa48db",
*        "kind": "Th",
*        "text": "Th"
*    },
*    {
*        "id": "581a0df7-b186-4ee0-bc77-87e097a211a4",
*        "kind": "Undefined",
*        "text": "e"
*    },
*    {
*        "id": "fbe03258-b1a2-4cb1-95ea-d86fc0af15dc",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "fe613f4e-d94c-442d-97c7-043861ed1eaa",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "d0955b71-27d9-4db9-9fa6-2d0a9ed382e9",
*        "kind": "Undefined",
*        "text": "e"
*    },
*    {
*        "id": "a4694dae-992f-4d4b-b8a9-c69cb204b515",
*        "kind": "Undefined",
*        "text": "x"
*    },
*    {
*        "id": "a856290f-2f98-4235-879b-b73971822873",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "b6c92eae-5d9c-4f82-b7b6-a3ec784d70c6",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "c02e3bb9-39f8-478f-8812-bb2aeebffadd",
*        "kind": "Dj",
*        "text": "j"
*    },
*    {
*        "id": "f28a0cf2-7502-41c2-8605-9572c057ad5c",
*        "kind": "Undefined",
*        "text": "u"
*    },
*    {
*        "id": "8141c8ee-88fe-47a7-9996-3091932232de",
*        "kind": "Undefined",
*        "text": "s"
*    },
*    {
*        "id": "f97943fe-4c28-4650-9136-4f88a127e58d",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "c7e1a64f-e69e-4b58-9541-57db30556c48",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "76d84df7-871b-483c-b881-3d83aac54712",
*        "kind": "Undefined",
*        "text": "i"
*    },
*    {
*        "id": "7a04b7fd-93f6-4280-b35c-6ba74860d179",
*        "kind": "Undefined",
*        "text": "n"
*    },
*    {
*        "id": "6977fd36-9853-4196-ba13-6fc339178a14",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "98fe1a9f-1b29-4544-8a39-c411441dbdd5",
*        "kind": "Ptk",
*        "text": "c"
*    },
*    {
*        "id": "a72ef6d2-eb3c-477c-8d2e-6e73098f1446",
*        "kind": "Undefined",
*        "text": "a"
*    },
*    {
*        "id": "35ea88ed-a12e-4bc6-b553-1ee45798a157",
*        "kind": "Undefined",
*        "text": "s"
*    },
*    {
*        "id": "c86762e5-5b39-4f3b-bc25-b009a6025434",
*        "kind": "Undefined",
*        "text": "e"
*    }
* ];
*
* console.log(sounds_from_text.every((sound, index) =>
*     sound.kind === target_sounds[index].kind &&
*     sound.text === target_sounds[index].text
* )); // true
* ```
* @param {string} text
* @returns {any}
*/
export function parse_wasm(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.parse_wasm(ptr0, len0);
    return takeObject(ret);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
/**
* Serialize sounds to text (wasm)
*
* ## Example
*
* ```js
* import {serialize_wasm} from "text-to-sounds";
*
* const sounds = [
*    {
*        "id": "96286ace-3209-4db5-a4b9-5f59e3aa48db",
*        "kind": "Th",
*        "text": "Th"
*    },
*    {
*        "id": "581a0df7-b186-4ee0-bc77-87e097a211a4",
*        "kind": "Undefined",
*        "text": "e"
*    },
*    {
*        "id": "fbe03258-b1a2-4cb1-95ea-d86fc0af15dc",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "fe613f4e-d94c-442d-97c7-043861ed1eaa",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "d0955b71-27d9-4db9-9fa6-2d0a9ed382e9",
*        "kind": "Undefined",
*        "text": "e"
*    },
*    {
*        "id": "a4694dae-992f-4d4b-b8a9-c69cb204b515",
*        "kind": "Undefined",
*        "text": "x"
*    },
*    {
*        "id": "a856290f-2f98-4235-879b-b73971822873",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "b6c92eae-5d9c-4f82-b7b6-a3ec784d70c6",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "c02e3bb9-39f8-478f-8812-bb2aeebffadd",
*        "kind": "Dj",
*        "text": "j"
*    },
*    {
*        "id": "f28a0cf2-7502-41c2-8605-9572c057ad5c",
*        "kind": "Undefined",
*        "text": "u"
*    },
*    {
*        "id": "8141c8ee-88fe-47a7-9996-3091932232de",
*        "kind": "Undefined",
*        "text": "s"
*    },
*    {
*        "id": "f97943fe-4c28-4650-9136-4f88a127e58d",
*        "kind": "Ptk",
*        "text": "t"
*    },
*    {
*        "id": "c7e1a64f-e69e-4b58-9541-57db30556c48",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "76d84df7-871b-483c-b881-3d83aac54712",
*        "kind": "Undefined",
*        "text": "i"
*    },
*    {
*        "id": "7a04b7fd-93f6-4280-b35c-6ba74860d179",
*        "kind": "Undefined",
*        "text": "n"
*    },
*    {
*        "id": "6977fd36-9853-4196-ba13-6fc339178a14",
*        "kind": "Undefined",
*        "text": " "
*    },
*    {
*        "id": "98fe1a9f-1b29-4544-8a39-c411441dbdd5",
*        "kind": "Ptk",
*        "text": "c"
*    },
*    {
*        "id": "a72ef6d2-eb3c-477c-8d2e-6e73098f1446",
*        "kind": "Undefined",
*        "text": "a"
*    },
*    {
*        "id": "35ea88ed-a12e-4bc6-b553-1ee45798a157",
*        "kind": "Undefined",
*        "text": "s"
*    },
*    {
*        "id": "c86762e5-5b39-4f3b-bc25-b009a6025434",
*        "kind": "Undefined",
*        "text": "e"
*    }
* ];
* const text_from_sounds = serialize_wasm(sounds);
*
* console.log(text_from_sounds === "The text just in case"); // true
* ```
* @param {Array<ISound>} sounds
* @returns {string}
*/
export function serialize_wasm(sounds) {
    try {
        const retptr = wasm.__wbindgen_add_to_stack_pointer(-16);
        wasm.serialize_wasm(retptr, addBorrowedObject(sounds));
        var r0 = getInt32Memory0()[retptr / 4 + 0];
        var r1 = getInt32Memory0()[retptr / 4 + 1];
        return getStringFromWasm0(r0, r1);
    } finally {
        wasm.__wbindgen_add_to_stack_pointer(16);
        heap[stack_pointer++] = undefined;
        wasm.__wbindgen_free(r0, r1);
    }
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

export function __wbindgen_json_parse(arg0, arg1) {
    const ret = JSON.parse(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbindgen_json_serialize(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = JSON.stringify(obj === undefined ? null : obj);
    const ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbg_process_e56fd54cf6319b6c(arg0) {
    const ret = getObject(arg0).process;
    return addHeapObject(ret);
};

export function __wbindgen_is_object(arg0) {
    const val = getObject(arg0);
    const ret = typeof(val) === 'object' && val !== null;
    return ret;
};

export function __wbg_versions_77e21455908dad33(arg0) {
    const ret = getObject(arg0).versions;
    return addHeapObject(ret);
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_node_0dd25d832e4785d5(arg0) {
    const ret = getObject(arg0).node;
    return addHeapObject(ret);
};

export function __wbindgen_is_string(arg0) {
    const ret = typeof(getObject(arg0)) === 'string';
    return ret;
};

export function __wbg_require_0db1598d9ccecb30() { return handleError(function (arg0, arg1, arg2) {
    const ret = getObject(arg0).require(getStringFromWasm0(arg1, arg2));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_crypto_b95d7173266618a9(arg0) {
    const ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};

export function __wbg_msCrypto_5a86d77a66230f81(arg0) {
    const ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};

export function __wbg_getRandomValues_b14734aa289bc356() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };

export function __wbg_static_accessor_NODE_MODULE_26b231378c1be7dd() {
    const ret = module;
    return addHeapObject(ret);
};

export function __wbg_randomFillSync_91e2b39becca6147() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };

export function __wbg_newnoargs_fc5356289219b93b(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export function __wbg_call_4573f605ca4b5f10() { return handleError(function (arg0, arg1) {
    const ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };

export function __wbg_self_ba1ddafe9ea7a3a2() { return handleError(function () {
    const ret = self.self;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_window_be3cc430364fd32c() { return handleError(function () {
    const ret = window.window;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_globalThis_56d9c9f814daeeee() { return handleError(function () {
    const ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };

export function __wbg_global_8c35aeee4ac77f2b() { return handleError(function () {
    const ret = global.global;
    return addHeapObject(ret);
}, arguments) };

export function __wbindgen_is_undefined(arg0) {
    const ret = getObject(arg0) === undefined;
    return ret;
};

export function __wbg_buffer_de1150f91b23aa89(arg0) {
    const ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export function __wbg_new_97cf52648830a70d(arg0) {
    const ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export function __wbg_set_a0172b213e2469e9(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};

export function __wbg_length_e09c0b925ab8de5d(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

export function __wbg_newwithlength_e833b89f9db02732(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};

export function __wbg_subarray_9482ae5cd5cd99d3(arg0, arg1, arg2) {
    const ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export function __wbindgen_object_clone_ref(arg0) {
    const ret = getObject(arg0);
    return addHeapObject(ret);
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_memory() {
    const ret = wasm.memory;
    return addHeapObject(ret);
};

cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);

