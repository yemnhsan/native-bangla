// Native Bangla — TSF composition helper (ITfContext / ITfComposition).
// Copyright (c) Native Bangla contributors. MIT License.
//
// Simulated keystrokes are intentionally avoided. Text is inserted and updated
// only through TSF composition edit sessions.

#pragma once

#include <msctf.h>
#include <windows.h>

#include <string>

class CComposition {
public:
    CComposition();
    ~CComposition();

    void SetClientId(TfClientId clientId);
    bool IsComposing() const;

    /// Begin a composition on the focused context (via edit session — TODO).
    HRESULT StartComposition(ITfContext* pContext);

    /// Update the in-progress composing string shown to the application.
    HRESULT UpdateComposition(ITfContext* pContext, const std::wstring& text);

    /// Finalize and insert the committed Unicode string, then end composition.
    HRESULT CommitComposition(ITfContext* pContext, const std::wstring& commitText);

    /// Cancel and discard the current composition without inserting text.
    HRESULT CancelComposition(ITfContext* pContext);

    void Clear();

private:
    // TODO: Implement ITfEditSession subclasses that call:
    //   ITfContext::RequestEditSession → ITfInsertAtSelection / ITfRange
    //   ITfContextComposition::StartComposition
    //   ITfComposition::ShiftStart / ShiftEnd / EndComposition
    // until those land, these methods retain structure and return S_OK / E_NOTIMPL.

    TfClientId m_clientId;
    ITfComposition* m_pComposition;
    std::wstring m_composingText;
};
