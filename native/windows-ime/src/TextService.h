// Native Bangla — TSF text service (ITfTextInputProcessorEx + sinks).
// Copyright (c) Native Bangla contributors. MIT License.
//
// Design note: Native Bangla avoids SendInput / simulated keystrokes.
// All typed output is delivered through TSF composition APIs so applications
// receive proper Unicode insertion, undo, and caret behavior.

#pragma once

#include "NativeBanglaIME.h"

#include "CandidateList.h"
#include "Composition.h"

class CTextService final : public ITfTextInputProcessorEx,
                           public ITfKeyEventSink,
                           public ITfThreadMgrEventSink {
public:
    CTextService();
    ~CTextService();

    // IUnknown
    STDMETHODIMP QueryInterface(REFIID riid, void** ppvObj) override;
    STDMETHODIMP_(ULONG) AddRef() override;
    STDMETHODIMP_(ULONG) Release() override;

    // ITfTextInputProcessor
    STDMETHODIMP Activate(ITfThreadMgr* ptim, TfClientId tid) override;
    STDMETHODIMP Deactivate() override;

    // ITfTextInputProcessorEx
    STDMETHODIMP ActivateEx(ITfThreadMgr* ptim, TfClientId tid, DWORD dwFlags) override;

    // ITfKeyEventSink
    STDMETHODIMP OnSetFocus(BOOL fForeground) override;
    STDMETHODIMP OnTestKeyDown(ITfContext* pic, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) override;
    STDMETHODIMP OnTestKeyUp(ITfContext* pic, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) override;
    STDMETHODIMP OnKeyDown(ITfContext* pic, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) override;
    STDMETHODIMP OnKeyUp(ITfContext* pic, WPARAM wParam, LPARAM lParam, BOOL* pfEaten) override;
    STDMETHODIMP OnPreservedKey(ITfContext* pic, REFGUID rguid, BOOL* pfEaten) override;

    // ITfThreadMgrEventSink
    STDMETHODIMP OnInitDocumentMgr(ITfDocumentMgr* pdim) override;
    STDMETHODIMP OnUninitDocumentMgr(ITfDocumentMgr* pdim) override;
    STDMETHODIMP OnSetFocus(ITfDocumentMgr* pdimFocus, ITfDocumentMgr* pdimPrevFocus) override;
    STDMETHODIMP OnPushContext(ITfContext* pic) override;
    STDMETHODIMP OnPopContext(ITfContext* pic) override;

private:
    bool InitKeyEventSink();
    void UninitKeyEventSink();
    bool InitThreadMgrEventSink();
    void UninitThreadMgrEventSink();

    /// Forward a character into Native Engine via ime-bridge and update composition.
    HRESULT HandleComposeChar(ITfContext* pic, char chAscii);
    HRESULT HandleBackspace(ITfContext* pic);
    HRESULT HandleCommit(ITfContext* pic);
    HRESULT HandleCancel(ITfContext* pic);

    LONG m_cRef;
    ITfThreadMgr* m_pThreadMgr;
    TfClientId m_tfClientId;
    DWORD m_dwThreadMgrEventSinkCookie;
    DWORD m_dwActivateFlags;

    CComposition m_composition;
    CCandidateList m_candidateList;
};
