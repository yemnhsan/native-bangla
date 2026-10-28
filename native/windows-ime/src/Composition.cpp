// Native Bangla — TSF composition helper implementation.
// Copyright (c) Native Bangla contributors. MIT License.

#include "Composition.h"

CComposition::CComposition()
    : m_clientId(TF_CLIENTID_NULL), m_pComposition(nullptr) {}

CComposition::~CComposition() {
    Clear();
}

void CComposition::SetClientId(TfClientId clientId) {
    m_clientId = clientId;
}

bool CComposition::IsComposing() const {
    return m_pComposition != nullptr || !m_composingText.empty();
}

HRESULT CComposition::StartComposition(ITfContext* pContext) {
    if (pContext == nullptr) {
        return E_INVALIDARG;
    }

    // TODO: Request a read/write edit session and call
    // ITfContextComposition::StartComposition with an insert-at-selection range.
    // Store the resulting ITfComposition* in m_pComposition.
    (void)m_clientId;
    return S_OK;
}

HRESULT CComposition::UpdateComposition(ITfContext* pContext, const std::wstring& text) {
    if (pContext == nullptr) {
        return E_INVALIDARG;
    }

    m_composingText = text;

    // TODO: In an ITfEditSession::DoEditSession implementation:
    //   1. Obtain the composition range from m_pComposition
    //   2. SetText on the range to `text`
    //   3. Apply display attributes (underline) via ITfProperty
    // Do not use SendInput / keybd_event — TSF composition only.
    return S_OK;
}

HRESULT CComposition::CommitComposition(ITfContext* pContext, const std::wstring& commitText) {
    if (pContext == nullptr) {
        return E_INVALIDARG;
    }

    // TODO: Edit session — set final text on the composition range, then
    // ITfComposition::EndComposition and Release m_pComposition.
    m_composingText = commitText;
    if (m_pComposition != nullptr) {
        m_pComposition->Release();
        m_pComposition = nullptr;
    }
    m_composingText.clear();
    return S_OK;
}

HRESULT CComposition::CancelComposition(ITfContext* pContext) {
    if (pContext == nullptr) {
        return E_INVALIDARG;
    }

    // TODO: Edit session — clear range text and EndComposition.
    if (m_pComposition != nullptr) {
        m_pComposition->Release();
        m_pComposition = nullptr;
    }
    m_composingText.clear();
    return S_OK;
}

void CComposition::Clear() {
    if (m_pComposition != nullptr) {
        m_pComposition->Release();
        m_pComposition = nullptr;
    }
    m_composingText.clear();
}
