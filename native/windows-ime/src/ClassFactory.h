// Native Bangla — COM class factory for the TSF text service.
// Copyright (c) Native Bangla contributors. MIT License.

#pragma once

#include <windows.h>

class CClassFactory final : public IClassFactory {
public:
    CClassFactory();
    ~CClassFactory();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void** ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // IClassFactory
    STDMETHODIMP CreateInstance(IUnknown* pUnkOuter, REFIID riid, void** ppvObj) override;
    STDMETHODIMP LockServer(BOOL fLock) override;

private:
    LONG m_cRef;
};
