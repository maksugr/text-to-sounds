(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/text_to_sounds.js":
/*!********************************!*\
  !*** ../pkg/text_to_sounds.js ***!
  \********************************/
/*! exports provided: highlight_wasm, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./text_to_sounds_bg.wasm */ \"../pkg/text_to_sounds_bg.wasm\");\n/* harmony import */ var _text_to_sounds_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./text_to_sounds_bg.js */ \"../pkg/text_to_sounds_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"highlight_wasm\", function() { return _text_to_sounds_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"highlight_wasm\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return _text_to_sounds_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_throw\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/text_to_sounds.js?");

/***/ }),

/***/ "../pkg/text_to_sounds_bg.js":
/*!***********************************!*\
  !*** ../pkg/text_to_sounds_bg.js ***!
  \***********************************/
/*! exports provided: highlight_wasm, __wbindgen_throw */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"highlight_wasm\", function() { return highlight_wasm; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_throw\", function() { return __wbindgen_throw; });\n/* harmony import */ var _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./text_to_sounds_bg.wasm */ \"../pkg/text_to_sounds_bg.wasm\");\n\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nlet cachedUint8Memory0;\nfunction getUint8Memory0() {\n    if (cachedUint8Memory0.byteLength === 0) {\n        cachedUint8Memory0 = new Uint8Array(_text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedUint8Memory0;\n}\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachedInt32Memory0;\nfunction getInt32Memory0() {\n    if (cachedInt32Memory0.byteLength === 0) {\n        cachedInt32Memory0 = new Int32Array(_text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachedInt32Memory0;\n}\n/**\n* Highlight sounds in the text with html tags (wasm)\n*\n* ## Example\n*\n* ```js\n* import {highlight_wasm} from \"text-to-sounds\";\n*\n* console.log(highlight_wasm(\"The text just in case\") === \"<span class='Th'>Th</span>e <span class='Ptk'>t</span>ex<span class='Ptk'>t</span> <span class='Dj'>j</span>us<span class='Ptk'>t</span> in <span class='Ptk'>c</span>ase\"); // true\n* ```\n* @param {string} text\n* @returns {string}\n*/\nfunction highlight_wasm(text) {\n    try {\n        const retptr = _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        const ptr0 = passStringToWasm0(text, _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        const len0 = WASM_VECTOR_LEN;\n        _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"highlight_wasm\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\nfunction __wbindgen_throw(arg0, arg1) {\n    throw new Error(getStringFromWasm0(arg0, arg1));\n};\n\ncachedInt32Memory0 = new Int32Array(_text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\ncachedUint8Memory0 = new Uint8Array(_text_to_sounds_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/text_to_sounds_bg.js?");

/***/ }),

/***/ "../pkg/text_to_sounds_bg.wasm":
/*!*************************************!*\
  !*** ../pkg/text_to_sounds_bg.wasm ***!
  \*************************************/
/*! exports provided: memory, highlight_wasm, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./text_to_sounds_bg.js */ \"../pkg/text_to_sounds_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/text_to_sounds_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var text_to_sounds__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! text-to-sounds */ \"../pkg/text_to_sounds.js\");\n\n\nconst MAX_TEXT_LENGTH = 10000;\n\nconst contenteditableEl = document.getElementById('contenteditable');\nconst maxLengthTextEl = document.getElementById('max-length-text');\n\nconst contenteditableFocus = () => {\n    contenteditableEl.focus();\n};\n\nconst contenteditableFontSize = () => {\n    const textContentLength = contenteditableEl.textContent.length;\n\n    if (textContentLength > 150) {\n        contenteditableEl.style.fontSize = 24 + 'px';\n        return;\n    }\n\n    if (textContentLength > 50) {\n        contenteditableEl.style.fontSize = 50 + 'px';\n        return;\n    }\n\n    contenteditableEl.style.fontSize = 100 + 'px';\n};\n\nconst contenteditableHighlight = () => {\n    const textContent = contenteditableEl.textContent;\n\n    const isMaxTextLength = textContent.length >= MAX_TEXT_LENGTH;\n\n    contenteditableEl.innerHTML = Object(text_to_sounds__WEBPACK_IMPORTED_MODULE_0__[\"highlight_wasm\"])(textContent.slice(0, MAX_TEXT_LENGTH));\n    document.execCommand('selectAll', false, null);\n    document.getSelection().collapseToEnd();\n\n    if (isMaxTextLength) {\n        maxLengthTextEl.classList.remove('hidden');\n    } else {\n        maxLengthTextEl.classList.add('hidden');\n    }\n};\n\ncontenteditableFocus();\ncontenteditableFontSize();\n\ncontenteditableEl.addEventListener('input', () => {\n    contenteditableFontSize();\n    contenteditableHighlight();\n});\n\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);