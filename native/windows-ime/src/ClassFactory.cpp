// Native Bangla — COM class factory implementation.
// Copyright (c) Native Bangla contributors. MIT License.

#include "ClassFactory.h"
#include "TextService.h"
#include "NativeBanglaIME.h"

CClassFactory::CClassFactory() : m_cRef(1) {
    DllAddRef();
}

CClassFactory::~CClassFactory() {
    DllRelease();
}

STDMETHODIMP CClassFactory::QueryInterface(REFIID riid, void** ppvObj) {
    if (ppvObj == nullptr) {
        return E_INVALIDARG;
    }
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_IClassFactory)) {
        *ppvObj = static_cast<IClassFactory*>(this);
    } else {
        return E_NOINTERFACE;
    }

    AddRef();
    return S_OK;
}

STDMETHODIMP_(ULONG) CClassFactory::AddRef() {
    return static_cast<ULONG>(InterlockedIncrement(&m_cRef));
}

STDMETHODIMP_(ULONG) CClassFactory::Release() {
    const LONG cRef = InterlockedDecrement(&m_cRef);
    if (cRef == 0) {
        delete this;
    }
    return static_cast<ULONG>(cRef);
}

STDMETHODIMP CClassFactory::CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObj) {
    if (ppvObj == nullptr) {
        return E_INVALIDARG;
    }
    *ppvObj = nullptr;

    if (pUnkOuter != nullptr) {
        return CLASS_E_NOAGGREGATION;
    }

    CTextService* pService = new (std::nothrow) CTextService();
    if (pService == nullptr) {
        return E_OUTOFMEMORY;
    }

    const HRESULT hr = pService->QueryInterface(riid, ppvObj);
    pService->Release();
    return hr;
}

STDMETHODIMP CClassFactory::LockServer(BOOL fLock) {
    if (fLock) {
        DllAddRef();
    } else {
        DllRelease();
    }
    return S_OK;
}
