// Native Bangla — Windows TSF text service identifiers and product constants.
// Copyright (c) Native Bangla contributors. MIT License.

#pragma once

#include <windows.h>
#include <msctf.h>

// Stable GUIDs for Native Bangla TSF registration. Do not regenerate casually;
// changing them breaks existing user profiles and registry state.

// {9F2E4A1B-8C3D-4E5F-A6B7-C8D9E0F1A2B3}
static const CLSID CLSID_NativeBanglaTextService = {
    0x9F2E4A1B,
    0x8C3D,
    0x4E5F,
    {0xA6, 0xB7, 0xC8, 0xD9, 0xE0, 0xF1, 0xA2, 0xB3}};

// {B1C2D3E4-F5A6-4789-ABCD-EF0123456789}
static const GUID GUID_NativeBanglaProfile = {
    0xB1C2D3E4,
    0xF5A6,
    0x4789,
    {0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89}};

// Bengali (Bangladesh) primary language ID for the language profile.
constexpr LANGID NATIVE_BANGLA_LANGID = MAKELANGID(LANG_BENGALI, SUBLANG_BENGALI_BANGLADESH);

constexpr wchar_t kProductName[] = L"Native Bangla";
constexpr wchar_t kEngineName[] = L"Native Engine";
constexpr wchar_t kTagline[] = L"Type Naturally. Write Beautifully.";
constexpr wchar_t kDescription[] = L"Native Bangla Input Method Editor";
constexpr wchar_t kDllName[] = L"NativeBanglaIME.dll";
constexpr wchar_t kProfileDescription[] = L"Native Bangla";

// ime-bridge protocol version expected by this build of the TSF host.
constexpr UINT32 kExpectedBridgeProtocolVersion = 1;

// Module lifetime (DllMain / class factory).
extern HINSTANCE g_hInst;
extern LONG g_cRefDll;

void DllAddRef();
void DllRelease();
