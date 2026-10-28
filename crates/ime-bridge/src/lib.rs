//! # ime-bridge
//!
//! C ABI bridge between Native Engine (C++ TSF) and Rust linguistic crates.
//!
//! Protocol versioning: bump `BRIDGE_PROTOCOL_VERSION` when the FFI layout changes.

#![allow(clippy::missing_safety_doc)]

use once_cell::sync::Lazy;
use phonetic_engine::{
    transliterate, CompositionEvent, CompositionSession, InputMode, PhoneticConfig,
};
use prediction_engine::{PredictRequest, Predictor};
use serde_json::json;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

pub const BRIDGE_PROTOCOL_VERSION: u32 = 1;
pub const PRODUCT_NAME: &str = "Native Bangla";

static SESSION: Lazy<Mutex<CompositionSession>> =
    Lazy::new(|| Mutex::new(CompositionSession::default()));

static PREDICTOR: Lazy<Mutex<Option<Predictor>>> = Lazy::new(|| Mutex::new(None));

fn with_session<F, R>(f: F) -> R
where
    F: FnOnce(&mut CompositionSession) -> R,
{
    let mut guard = SESSION.lock().expect("session lock");
    f(&mut guard)
}

/// Initialize Native Predict with a dictionary JSON path (UTF-8).
/// Returns 0 on success, negative on error.
#[no_mangle]
pub extern "C" fn native_bangla_init_predictor(dictionary_path: *const c_char) -> i32 {
    if dictionary_path.is_null() {
        return -1;
    }
    let path = unsafe { CStr::from_ptr(dictionary_path) };
    let Ok(path) = path.to_str() else {
        return -2;
    };
    match dictionary_core::Dictionary::load(path) {
        Ok(dict) => {
            *PREDICTOR.lock().expect("predictor lock") = Some(Predictor::new(dict));
            0
        }
        Err(_) => -3,
    }
}

/// Transliterate UTF-8 phonetic input. Caller must free with `native_bangla_string_free`.
#[no_mangle]
pub extern "C" fn native_bangla_transliterate(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return std::ptr::null_mut();
    }
    let Ok(s) = (unsafe { CStr::from_ptr(input) }).to_str() else {
        return std::ptr::null_mut();
    };
    let out = transliterate(s, &PhoneticConfig::default());
    CString::new(out).map(|c| c.into_raw()).unwrap_or(std::ptr::null_mut())
}

/// Push a UTF-8 character (may be multi-byte) into composition.
/// Returns JSON CompositionUpdate. Caller frees with `native_bangla_string_free`.
#[no_mangle]
pub extern "C" fn native_bangla_compose_char(ch_utf8: *const c_char) -> *mut c_char {
    if ch_utf8.is_null() {
        return std::ptr::null_mut();
    }
    let Ok(s) = (unsafe { CStr::from_ptr(ch_utf8) }).to_str() else {
        return std::ptr::null_mut();
    };
    let Some(ch) = s.chars().next() else {
        return std::ptr::null_mut();
    };
    let update = with_session(|session| session.process(CompositionEvent::Char(ch)));
    update_to_cjson(&update)
}

#[no_mangle]
pub extern "C" fn native_bangla_compose_backspace() -> *mut c_char {
    let update = with_session(|s| s.process(CompositionEvent::Backspace));
    update_to_cjson(&update)
}

#[no_mangle]
pub extern "C" fn native_bangla_compose_commit() -> *mut c_char {
    let update = with_session(|s| s.process(CompositionEvent::Commit));
    update_to_cjson(&update)
}

#[no_mangle]
pub extern "C" fn native_bangla_compose_cancel() -> *mut c_char {
    let update = with_session(|s| s.process(CompositionEvent::Cancel));
    update_to_cjson(&update)
}

/// mode: 0=phonetic, 1=traditional, 2=english
#[no_mangle]
pub extern "C" fn native_bangla_set_mode(mode: i32) -> *mut c_char {
    let m = match mode {
        1 => InputMode::Traditional,
        2 => InputMode::English,
        _ => InputMode::Phonetic,
    };
    let update = with_session(|s| s.process(CompositionEvent::SetMode(m)));
    update_to_cjson(&update)
}

/// Predict suggestions. `context_json` is a JSON array of preceding words.
#[no_mangle]
pub extern "C" fn native_bangla_predict(
    phonetic: *const c_char,
    context_json: *const c_char,
    limit: u32,
    next_word: i32,
) -> *mut c_char {
    let phonetic = if phonetic.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(phonetic) }
            .to_str()
            .unwrap_or("")
            .to_string()
    };
    let context_words: Vec<String> = if context_json.is_null() {
        vec![]
    } else {
        let s = unsafe { CStr::from_ptr(context_json) }.to_str().unwrap_or("[]");
        serde_json::from_str(s).unwrap_or_default()
    };

    let guard = PREDICTOR.lock().expect("predictor lock");
    let Some(predictor) = guard.as_ref() else {
        // Fallback: phonetic only
        let primary = transliterate(&phonetic, &PhoneticConfig::default());
        let payload = json!({
            "primary": primary,
            "suggestions": [{ "text": primary, "score": 1.0, "source": "phonetic" }]
        });
        return cjson(&payload);
    };

    let resp = predictor.predict(&PredictRequest {
        phonetic_prefix: phonetic,
        context_words,
        limit: limit as usize,
        next_word: next_word != 0,
    });
    cjson(&resp)
}

#[no_mangle]
pub extern "C" fn native_bangla_protocol_version() -> u32 {
    BRIDGE_PROTOCOL_VERSION
}

#[no_mangle]
pub extern "C" fn native_bangla_string_free(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        drop(CString::from_raw(ptr));
    }
}

fn update_to_cjson(update: &phonetic_engine::CompositionUpdate) -> *mut c_char {
    let payload = json!({
        "raw_buffer": update.raw_buffer,
        "composed": update.composed,
        "commit_text": update.commit_text,
        "composing": update.composing,
        "candidates": update.candidates,
    });
    cjson(&payload)
}

fn cjson<T: serde::Serialize>(value: &T) -> *mut c_char {
    let Ok(s) = serde_json::to_string(value) else {
        return std::ptr::null_mut();
    };
    CString::new(s)
        .map(|c| c.into_raw())
        .unwrap_or(std::ptr::null_mut())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn transliterate_ffi() {
        let input = CString::new("ami").unwrap();
        let ptr = native_bangla_transliterate(input.as_ptr());
        assert!(!ptr.is_null());
        let out = unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned();
        native_bangla_string_free(ptr);
        assert_eq!(out, "আমি");
    }

    #[test]
    fn protocol_version() {
        assert_eq!(native_bangla_protocol_version(), 1);
    }
}
