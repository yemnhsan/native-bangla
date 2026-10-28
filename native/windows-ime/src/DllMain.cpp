// Native Bangla — Windows TSF text service DLL entry points & registration.
// Copyright (c) Native Bangla contributors. MIT License.

#include "NativeBanglaIME.h"
#include "ClassFactory.h"

#include <msctf.h>
#include <new>

HINSTANCE g_hInst = nullptr;
LONG g_cRefDll = 0;

void DllAddRef() {
    InterlockedIncrement(&g_cRefDll);
}

void DllRelease() {
    InterlockedDecrement(&g_cRefDll);
}

BOOL WINAPI DllMain(HINSTANCE hInstance, DWORD dwReason, LPVOID lpReserved) {
    switch (dwReason) {
        case DLL_PROCESS_ATTACH:
            g_hInst = hInstance;
            DisableThreadLibraryCalls(hInstance);
            break;
        case DLL_PROCESS_DETACH:
            break;
        default:
            break;
    }
    return TRUE;
}

STDAPI DllGetClassObject(REFCLSID rclsid, REFIID riid, void** ppv) {
    if (ppv == nullptr) {
        return E_INVALIDARG;
    }
    *ppv = nullptr;

    if (!IsEqualCLSID(rclsid, CLSID_NativeBanglaTextService)) {
        return CLASS_E_CLASSNOTAVAILABLE;
    }

    CClassFactory* pFactory = new (std::nothrow) CClassFactory();
    if (pFactory == nullptr) {
        return E_OUTOFMEMORY;
    }

    const HRESULT hr = pFactory->QueryInterface(riid, ppv);
    pFactory->Release();
    return hr;
}

STDAPI DllCanUnloadNow(void) {
    return (g_cRefDll == 0) ? S_OK : S_FALSE;
}

namespace {

constexpr wchar_t kClsidPrefix[] = L"CLSID\\{9F2E4A1B-8C3D-4E5F-A6B7-C8D9E0F1A2B3}";
constexpr wchar_t kInprocServerKey[] = L"CLSID\\{9F2E4A1B-8C3D-4E5F-A6B7-C8D9E0F1A2B3}\\InProcServer32";
constexpr wchar_t kThreadingModelVal[] = L"Apartment";

bool RegisterCOMServer(HMODULE hMod) {
    wchar_t szDllPath[MAX_PATH];
    if (GetModuleFileNameW(hMod, szDllPath, ARRAYSIZE(szDllPath)) == 0) {
        return false;
    }

    HKEY hKey = nullptr;
    if (RegCreateKeyExW(HKEY_CLASSES_ROOT, kClsidPrefix, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hKey, nullptr) != ERROR_SUCCESS) {
        return false;
    }
    RegSetValueExW(hKey, nullptr, 0, REG_SZ, reinterpret_cast<const BYTE*>(kDescription),
                   static_cast<DWORD>((wcslen(kDescription) + 1) * sizeof(wchar_t)));
    RegCloseKey(hKey);

    if (RegCreateKeyExW(HKEY_CLASSES_ROOT, kInprocServerKey, 0, nullptr,
                        REG_OPTION_NON_VOLATILE, KEY_WRITE, nullptr, &hKey, nullptr) != ERROR_SUCCESS) {
        return false;
    }
    RegSetValueExW(hKey, nullptr, 0, REG_SZ, reinterpret_cast<const BYTE*>(szDllPath),
                   static_cast<DWORD>((wcslen(szDllPath) + 1) * sizeof(wchar_t)));
    RegSetValueExW(hKey, L"ThreadingModel", 0, REG_SZ, reinterpret_cast<const BYTE*>(kThreadingModelVal),
                   static_cast<DWORD>((wcslen(kThreadingModelVal) + 1) * sizeof(wchar_t)));
    RegCloseKey(hKey);

    return true;
}

void UnregisterCOMServer() {
    RegDeleteKeyW(HKEY_CLASSES_ROOT, kInprocServerKey);
    RegDeleteKeyW(HKEY_CLASSES_ROOT, kClsidPrefix);
}

}  // namespace

STDAPI DllRegisterServer(void) {
    if (!RegisterCOMServer(g_hInst)) {
        return E_FAIL;
    }

    // Register with Windows Text Services Framework profiles
    ITfInputProcessorProfiles* pProfiles = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr, CLSCTX_INPROC_SERVER,
                                  IID_ITfInputProcessorProfiles, reinterpret_cast<void**>(&pProfiles));
    if (SUCCEEDED(hr) && pProfiles != nullptr) {
        hr = pProfiles->Register(CLSID_NativeBanglaTextService);
        if (SUCCEEDED(hr)) {
            wchar_t szDllPath[MAX_PATH];
            GetModuleFileNameW(g_hInst, szDllPath, ARRAYSIZE(szDllPath));

            hr = pProfiles->AddLanguageProfile(
                CLSID_NativeBanglaTextService,
                NATIVE_BANGLA_LANGID,
                GUID_NativeBanglaProfile,
                kProfileDescription,
                static_cast<ULONG>(wcslen(kProfileDescription)),
                szDllPath,
                static_cast<ULONG>(wcslen(szDllPath)),
                0);
        }
        pProfiles->Release();
    }

    return hr;
}

STDAPI DllUnregisterServer(void) {
    ITfInputProcessorProfiles* pProfiles = nullptr;
    HRESULT hr = CoCreateInstance(CLSID_TF_InputProcessorProfiles, nullptr, CLSCTX_INPROC_SERVER,
                                  IID_ITfInputProcessorProfiles, reinterpret_cast<void**>(&pProfiles));
    if (SUCCEEDED(hr) && pProfiles != nullptr) {
        pProfiles->Unregister(CLSID_NativeBanglaTextService);
        pProfiles->Release();
    }

    UnregisterCOMServer();
    return S_OK;
}
