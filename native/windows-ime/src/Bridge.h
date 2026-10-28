// Native Bangla — C ABI declarations matching crates/ime-bridge FFI.
// Copyright (c) Native Bangla contributors. MIT License.

#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Initialize Native Predict with a dictionary JSON path (UTF-8).
/// Returns 0 on success, negative on error.
int32_t native_bangla_init_predictor(const char* dictionary_path);

/// Transliterate UTF-8 phonetic input.
/// Caller must free the result with native_bangla_string_free.
char* native_bangla_transliterate(const char* input);

/// Push a UTF-8 character into composition.
/// Returns JSON CompositionUpdate; free with native_bangla_string_free.
char* native_bangla_compose_char(const char* ch_utf8);

/// Composition editing helpers; return JSON CompositionUpdate.
char* native_bangla_compose_backspace(void);
char* native_bangla_compose_commit(void);
char* native_bangla_compose_cancel(void);

/// mode: 0=phonetic, 1=traditional, 2=english.
/// Returns JSON CompositionUpdate; free with native_bangla_string_free.
char* native_bangla_set_mode(int32_t mode);

/// Predict suggestions. context_json is a JSON array of preceding words.
/// Returns JSON; free with native_bangla_string_free.
char* native_bangla_predict(const char* phonetic,
                            const char* context_json,
                            uint32_t limit,
                            int32_t next_word);

uint32_t native_bangla_protocol_version(void);

void native_bangla_string_free(char* ptr);

#ifdef __cplusplus
}
#endif
