
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

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

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        try {
            return f(state.a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(state.a, state.b);
                state.a = 0;

            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_18(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hee66633b5afceae2(arg0, arg1, addHeapObject(arg2));
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_21(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6c1480f21af15b9d(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_24(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8541ea54ad7ea27a(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_27(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h467f168fe7c0222c(arg0, arg1);
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('hooks_shared_state_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_cb_drop = function(arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_is_function = function(arg0) {
        var ret = typeof(getObject(arg0)) === 'function';
        return ret;
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        var ret = new Error();
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(arg0, arg1) {
        var ret = getObject(arg1).stack;
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
};
imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_Window_f826a1dec163bacb = function(arg0) {
    var ret = getObject(arg0).Window;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbg_WorkerGlobalScope_967d186155183d38 = function(arg0) {
    var ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};
imports.wbg.__wbg_document_1c64944725c0d81d = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_requestAnimationFrame_71638ca922068239 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_requestIdleCallback_183ce6abc9f992af = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).requestIdleCallback(getObject(arg1));
    return ret;
}, arguments) };
imports.wbg.__wbg_clearTimeout_2c1ba0016d8bca41 = function(arg0, arg1) {
    getObject(arg0).clearTimeout(arg1);
};
imports.wbg.__wbg_setTimeout_df66d951b1726b78 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_createComment_710531dc62f02c0c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createComment(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createElement_86c152812a141a62 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createElementNS_ae12b8681c3957a3 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var ret = getObject(arg0).createElementNS(v0, v1);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_365db3bc3d0523ab = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_getElementById_f3e94458ce77f0d0 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).getElementById(v0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_querySelectorAll_7f26183d7dfc576e = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).querySelectorAll(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_HtmlInputElement_8cafe5f30dfdb6bc = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLInputElement;
    return ret;
};
imports.wbg.__wbg_checked_39d5ce76226024a7 = function(arg0) {
    var ret = getObject(arg0).checked;
    return ret;
};
imports.wbg.__wbg_setchecked_206243371da58f6a = function(arg0, arg1) {
    getObject(arg0).checked = arg1 !== 0;
};
imports.wbg.__wbg_type_ed14b550c9bab1c2 = function(arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_value_0627d4b1c27534e6 = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setvalue_2459f62386b6967f = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).value = v0;
};
imports.wbg.__wbg_instanceof_HtmlOptionElement_5d5f65dbc2cf3509 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLOptionElement;
    return ret;
};
imports.wbg.__wbg_setselected_ed15d65374c26375 = function(arg0, arg1) {
    getObject(arg0).selected = arg1 !== 0;
};
imports.wbg.__wbg_instanceof_KeyboardEvent_96bb8fcff2603455 = function(arg0) {
    var ret = getObject(arg0) instanceof KeyboardEvent;
    return ret;
};
imports.wbg.__wbg_charCode_94907480ee48e8d5 = function(arg0) {
    var ret = getObject(arg0).charCode;
    return ret;
};
imports.wbg.__wbg_keyCode_490ed69472addfdc = function(arg0) {
    var ret = getObject(arg0).keyCode;
    return ret;
};
imports.wbg.__wbg_altKey_3dcb50d5afbc5036 = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_fb62ba10b63b34a4 = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_bd2875540e5db840 = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_metaKey_94ca09e07f21f240 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_location_5bab830bd31662a2 = function(arg0) {
    var ret = getObject(arg0).location;
    return ret;
};
imports.wbg.__wbg_repeat_8004e40e8ff1a1f1 = function(arg0) {
    var ret = getObject(arg0).repeat;
    return ret;
};
imports.wbg.__wbg_key_10dcaa4bb6d5449f = function(arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_Event_480a3ec3d45b75a6 = function(arg0) {
    var ret = getObject(arg0) instanceof Event;
    return ret;
};
imports.wbg.__wbg_type_7a49279491e15d0a = function(arg0, arg1) {
    var ret = getObject(arg1).type;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_target_cc69dde6c2d9ec90 = function(arg0) {
    var ret = getObject(arg0).target;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlSelectElement_27fb687660e6b5ba = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLSelectElement;
    return ret;
};
imports.wbg.__wbg_value_bc4bb925ad58795b = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_length_62e6735d81b8b0f1 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_get_3bae69d3ceb47cc3 = function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_Node_ea9524a9184ff6d0 = function(arg0) {
    var ret = getObject(arg0) instanceof Node;
    return ret;
};
imports.wbg.__wbg_parentNode_cd9f47bcc933ed0e = function(arg0) {
    var ret = getObject(arg0).parentNode;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_textContent_eef491bffc43d8d6 = function(arg0, arg1) {
    var ret = getObject(arg1).textContent;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_settextContent_799ebbf96e16265d = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).textContent = v0;
};
imports.wbg.__wbg_appendChild_d318db34c4559916 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_insertBefore_5b314357408fbec1 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_removeChild_d3ca7b53e537867e = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_instanceof_Text_8d7df795b9c7f8f9 = function(arg0) {
    var ret = getObject(arg0) instanceof Text;
    return ret;
};
imports.wbg.__wbg_pageX_9b9d392e5134f3db = function(arg0) {
    var ret = getObject(arg0).pageX;
    return ret;
};
imports.wbg.__wbg_pageY_ee824e6b949cdd34 = function(arg0) {
    var ret = getObject(arg0).pageY;
    return ret;
};
imports.wbg.__wbg_which_222325717b841326 = function(arg0) {
    var ret = getObject(arg0).which;
    return ret;
};
imports.wbg.__wbg_instanceof_MouseEvent_7cb2c66c3440bfb0 = function(arg0) {
    var ret = getObject(arg0) instanceof MouseEvent;
    return ret;
};
imports.wbg.__wbg_screenX_6d3e727e1e71c8ba = function(arg0) {
    var ret = getObject(arg0).screenX;
    return ret;
};
imports.wbg.__wbg_screenY_f7ccb175523cee84 = function(arg0) {
    var ret = getObject(arg0).screenY;
    return ret;
};
imports.wbg.__wbg_clientX_97ff0f5c7b19e687 = function(arg0) {
    var ret = getObject(arg0).clientX;
    return ret;
};
imports.wbg.__wbg_clientY_cacd4a7e44b9719b = function(arg0) {
    var ret = getObject(arg0).clientY;
    return ret;
};
imports.wbg.__wbg_ctrlKey_9761d22fa42f09c0 = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_78ee0fc1aa572c2e = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_altKey_8936038d973c56db = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_e6b9e0aa35aa2974 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_button_a02c0467d38e8338 = function(arg0) {
    var ret = getObject(arg0).button;
    return ret;
};
imports.wbg.__wbg_buttons_9d7b6c334f0b37de = function(arg0) {
    var ret = getObject(arg0).buttons;
    return ret;
};
imports.wbg.__wbg_instanceof_CompositionEvent_e1cd64fd98d4425d = function(arg0) {
    var ret = getObject(arg0) instanceof CompositionEvent;
    return ret;
};
imports.wbg.__wbg_data_dbff09eb89176161 = function(arg0, arg1) {
    var ret = getObject(arg1).data;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setProperty_1460c660bc329763 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setProperty(v0, v1);
}, arguments) };
imports.wbg.__wbg_addEventListener_52721772cc0a7f30 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).addEventListener(v0, getObject(arg3));
}, arguments) };
imports.wbg.__wbg_instanceof_IdleDeadline_e91ef932d81715c3 = function(arg0) {
    var ret = getObject(arg0) instanceof IdleDeadline;
    return ret;
};
imports.wbg.__wbg_timeRemaining_6fa9d12d6d15e73e = function(arg0) {
    var ret = getObject(arg0).timeRemaining();
    return ret;
};
imports.wbg.__wbg_instanceof_TouchEvent_0ab696cda73333ac = function(arg0) {
    var ret = getObject(arg0) instanceof TouchEvent;
    return ret;
};
imports.wbg.__wbg_altKey_7cfb90a08ce4db73 = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};
imports.wbg.__wbg_metaKey_49d641fb7c6fd755 = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};
imports.wbg.__wbg_ctrlKey_2008616a1339c848 = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};
imports.wbg.__wbg_shiftKey_aa89958b58ad5242 = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};
imports.wbg.__wbg_instanceof_WheelEvent_41916fe296da5b3c = function(arg0) {
    var ret = getObject(arg0) instanceof WheelEvent;
    return ret;
};
imports.wbg.__wbg_deltaX_8cfc6cd15e97d97c = function(arg0) {
    var ret = getObject(arg0).deltaX;
    return ret;
};
imports.wbg.__wbg_deltaY_080604c20160c0e8 = function(arg0) {
    var ret = getObject(arg0).deltaY;
    return ret;
};
imports.wbg.__wbg_deltaZ_2a27c02ace57a19e = function(arg0) {
    var ret = getObject(arg0).deltaZ;
    return ret;
};
imports.wbg.__wbg_deltaMode_c5ec1ee518ea0a08 = function(arg0) {
    var ret = getObject(arg0).deltaMode;
    return ret;
};
imports.wbg.__wbg_instanceof_Element_97d85e53f1805b82 = function(arg0) {
    var ret = getObject(arg0) instanceof Element;
    return ret;
};
imports.wbg.__wbg_getAttribute_bb1d602e925e860a = function(arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var ret = getObject(arg1).getAttribute(v0);
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
};
imports.wbg.__wbg_removeAttribute_eea03ed128669b8f = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).removeAttribute(v0);
}, arguments) };
imports.wbg.__wbg_setAttribute_1b533bf07966de55 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    getObject(arg0).setAttribute(v0, v1);
}, arguments) };
imports.wbg.__wbg_after_41c4547474e5c7b2 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).after(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_before_9614b2163991f8df = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_remove_c63cabc94a77cacb = function(arg0) {
    getObject(arg0).remove();
};
imports.wbg.__wbg_replaceWith_1272417ed0a0204e = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).replaceWith(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_debug_f6147a62af5fb117 = function(arg0, arg1, arg2, arg3) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_error_cc38ce2b4b661e1d = function(arg0) {
    console.error(getObject(arg0));
};
imports.wbg.__wbg_error_8b4a1487636c965d = function(arg0, arg1, arg2, arg3) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_info_74a03c22e1fa6688 = function(arg0, arg1, arg2, arg3) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_log_ad41dbc3d891c2dc = function(arg0, arg1, arg2, arg3) {
    console.log(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_warn_c1cc594c33944c11 = function(arg0, arg1, arg2, arg3) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3));
};
imports.wbg.__wbg_instanceof_HtmlElement_df66c8b4a687aa43 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLElement;
    return ret;
};
imports.wbg.__wbg_style_c88e323890d3a091 = function(arg0) {
    var ret = getObject(arg0).style;
    return addHeapObject(ret);
};
imports.wbg.__wbg_clearTimeout_c1f246d6874c0679 = function(arg0, arg1) {
    getObject(arg0).clearTimeout(arg1);
};
imports.wbg.__wbg_setTimeout_5314850184d61a44 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_instanceof_CharacterData_923547385daf396d = function(arg0) {
    var ret = getObject(arg0) instanceof CharacterData;
    return ret;
};
imports.wbg.__wbg_after_c472195dda6b326c = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).after(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_before_fb2c3a2663318a78 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_replaceWith_2c48dcaa450a6671 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).replaceWith(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_instanceof_DocumentType_0e86823a57dde305 = function(arg0) {
    var ret = getObject(arg0) instanceof DocumentType;
    return ret;
};
imports.wbg.__wbg_after_c0743062e8170d00 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).after(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_before_52cbfbd938a19b42 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).before(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_replaceWith_8c24775c6a372049 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).replaceWith(...getObject(arg1));
}, arguments) };
imports.wbg.__wbg_instanceof_AnimationEvent_741ec19d568ec9b8 = function(arg0) {
    var ret = getObject(arg0) instanceof AnimationEvent;
    return ret;
};
imports.wbg.__wbg_animationName_e6b926c751875085 = function(arg0, arg1) {
    var ret = getObject(arg1).animationName;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_4b480f2e6b2791cd = function(arg0) {
    var ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_7c2bfde68d10a4b6 = function(arg0, arg1) {
    var ret = getObject(arg1).pseudoElement;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_instanceof_HtmlTextAreaElement_c2f3b4bd6871d5ad = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLTextAreaElement;
    return ret;
};
imports.wbg.__wbg_value_686b2a68422cb88d = function(arg0, arg1) {
    var ret = getObject(arg1).value;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_setvalue_0a07023245efa3cc = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).value = v0;
};
imports.wbg.__wbg_instanceof_PointerEvent_a131c52e2d8689a4 = function(arg0) {
    var ret = getObject(arg0) instanceof PointerEvent;
    return ret;
};
imports.wbg.__wbg_pointerId_9302f0e125f0b48e = function(arg0) {
    var ret = getObject(arg0).pointerId;
    return ret;
};
imports.wbg.__wbg_width_e3c349a44d741557 = function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
};
imports.wbg.__wbg_height_b04227a17ca32a40 = function(arg0) {
    var ret = getObject(arg0).height;
    return ret;
};
imports.wbg.__wbg_pressure_ccb6528a41a7988e = function(arg0) {
    var ret = getObject(arg0).pressure;
    return ret;
};
imports.wbg.__wbg_tangentialPressure_7e9160a69763fa5a = function(arg0) {
    var ret = getObject(arg0).tangentialPressure;
    return ret;
};
imports.wbg.__wbg_tiltX_e72f0f8d2db90a01 = function(arg0) {
    var ret = getObject(arg0).tiltX;
    return ret;
};
imports.wbg.__wbg_tiltY_c632d7fa14c6264a = function(arg0) {
    var ret = getObject(arg0).tiltY;
    return ret;
};
imports.wbg.__wbg_twist_10483210c2b4bf9f = function(arg0) {
    var ret = getObject(arg0).twist;
    return ret;
};
imports.wbg.__wbg_pointerType_2b40a4fc77c24d55 = function(arg0, arg1) {
    var ret = getObject(arg1).pointerType;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_isPrimary_1b61e5694cebdb4a = function(arg0) {
    var ret = getObject(arg0).isPrimary;
    return ret;
};
imports.wbg.__wbg_instanceof_TransitionEvent_a0154619d908db2f = function(arg0) {
    var ret = getObject(arg0) instanceof TransitionEvent;
    return ret;
};
imports.wbg.__wbg_propertyName_05bec2f4569cb7fe = function(arg0, arg1) {
    var ret = getObject(arg1).propertyName;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_elapsedTime_6f18fd8e77d055cd = function(arg0) {
    var ret = getObject(arg0).elapsedTime;
    return ret;
};
imports.wbg.__wbg_pseudoElement_08a057e1e2493048 = function(arg0, arg1) {
    var ret = getObject(arg1).pseudoElement;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbg_new_949bbc1147195c4e = function() {
    var ret = new Array();
    return addHeapObject(ret);
};
imports.wbg.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_push_284486ca27c6aa8b = function(arg0, arg1) {
    var ret = getObject(arg0).push(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_instanceof_Object_66786225e0dbc8ba = function(arg0) {
    var ret = getObject(arg0) instanceof Object;
    return ret;
};
imports.wbg.__wbg_hasOwnProperty_9060fa1885e12d11 = function(arg0, arg1) {
    var ret = getObject(arg0).hasOwnProperty(getObject(arg1));
    return ret;
};
imports.wbg.__wbg_resolve_d23068002f584f22 = function(arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_2fcac196782070cc = function(arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_closure_wrapper444 = function(arg0, arg1, arg2) {
    var ret = makeClosure(arg0, arg1, 160, __wbg_adapter_18);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper446 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 160, __wbg_adapter_21);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper528 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 210, __wbg_adapter_24);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper566 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 228, __wbg_adapter_27);
    return addHeapObject(ret);
};

if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
    input = fetch(input);
}



const { instance, module } = await load(await input, imports);

wasm = instance.exports;
init.__wbindgen_wasm_module = module;
wasm.__wbindgen_start();
return wasm;
}

export default init;

