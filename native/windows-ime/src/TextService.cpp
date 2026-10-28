// Native Bangla — TSF text service implementation.
// Copyright (c) Native Bangla contributors. MIT License.
//
// Simulated keystrokes (SendInput / keybd_event) are avoided in favor of TSF
// composition so host applications receive correct Unicode text insertion.

#include "TextService.h"

#include "Bridge.h"
#include "NativeBanglaIME.h"

#include <new>

namespace {

bool IsAsciiLetterOrDigit(WPARAM wParam) {
    return (wParam >= 'A' && wParam <= 'Z') || (wParam >= '0' && wParam <= '9') ||
           (wParam >= 'a' && wParam <= 'z');
}

char WparamToAscii(WPARAM wParam) {
    if (wParam >= 'A' && wParam <= 'Z') {
        // VK codes for letters match uppercase ASCII; shift state handled later.
        return static_cast<char>(wParam);
    }
    if (wParam >= '0' && wParam <= '9') {
        return static_cast<char>(wParam);
    }
    return static_cast<char>(wParam & 0xFF);
}

void FreeBridgeString(char* ptr) {
    if (ptr != nullptr) {
        native_bangla_string_free(ptr);
    }
}

}  // namespace

CTextService::CTextService()
    : m_cRef(1),
      m_pThreadMgr(nullptr),
      m_tfClientId(TF_CLIENTID_NULL),
      m_dwThreadMgrEventSinkCookie(TF_INVALID_COOKIE),
      m_dwActivateFlags(0) {
    DllAddRef();
}

CTextService::~CTextService() {
    Deactivate();
    DllRelease();
}

STDMETHODIMP CTextService::QueryInterface(REFIID riid, void** ppvObj) {
    if (ppvObj == nullptr) {
        return E_INVALIDARG;
    }
    *ppvObj = nullptr;

    if (IsEqualIID(riid, IID_IUnknown) || IsEqualIID(riid, IID_ITfTextInputProcessor)) {
        *ppvObj = static_cast<ITfTextInputProcessor*>(this);
    } else if (IsEqualIID(riid, IID_ITfTextInputProcessorEx)) {
        *ppvObj = static_cast<ITfTextInputProcessorEx*>(this);
    } else if (IsEqualIID(riid, IID_ITfKeyEventSink)) {
        *ppvObj = static_cast<ITfKeyEventSink*>(this);
    } else if (IsEqualIID(riid, IID_ITfThreadMgrEventSink)) {
        *ppvObj = static_cast<ITfThreadMgrEventSink*>(this);
    } else {
        return E_NOINTERFACE;
    }

    AddRef();
    return S_OK;
}

STDMETHODIMP_(ULONG) CTextService::AddRef() {
    return static_cast<ULONG>(InterlockedIncrement(&m_cRef));
}

STDMETHODIMP_(ULONG) CTextService::Release() {
    const LONG cRef = InterlockedDecrement(&m_cRef);
    if (cRef == 0) {
        delete this;
    }
    return static_cast<ULONG>(cRef);
}

STDMETHODIMP CTextService::Activate(ITfThreadMgr* ptim, TfClientId tid) {
    return ActivateEx(ptim, tid, 0);
}

STDMETHODIMP CTextService::ActivateEx(ITfThreadMgr* ptim, TfClientId tid, DWORD dwFlags) {
    if (ptim == nullptr) {
        return E_INVALIDARG;
    }

    m_pThreadMgr = ptim;
    m_pThreadMgr->AddRef();
    m_tfClientId = tid;
    m_dwActivateFlags = dwFlags;
    m_composition.SetClientId(tid);

    // Optional: verify ime-bridge protocol when the Rust DLL is linked.
    // const UINT32 ver = native_bangla_protocol_version();
    // if (ver != kExpectedBridgeProtocolVersion) { ... }

    if (!InitThreadMgrEventSink()) {
        Deactivate();
        return E_FAIL;
    }
    if (!InitKeyEventSink()) {
        Deactivate();
        return E_FAIL;
    }

    return S_OK;
}

STDMETHODIMP CTextService::Deactivate() {
    UninitKeyEventSink();
    UninitThreadMgrEventSink();

    m_composition.Clear();
    m_candidateList.Hide();
    m_candidateList.Clear();

    if (m_pThreadMgr != nullptr) {
        m_pThreadMgr->Release();
        m_pThreadMgr = nullptr;
    }
    m_tfClientId = TF_CLIENTID_NULL;
    m_dwActivateFlags = 0;
    return S_OK;
}

bool CTextService::InitKeyEventSink() {
    ITfKeystrokeMgr* pKeystrokeMgr = nullptr;
    if (FAILED(m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr,
                                            reinterpret_cast<void**>(&pKeystrokeMgr)))) {
        return false;
    }
    const HRESULT hr = pKeystrokeMgr->AdviseKeyEventSink(m_tfClientId,
                                                         static_cast<ITfKeyEventSink*>(this),
                                                         TRUE);
    pKeystrokeMgr->Release();
    return SUCCEEDED(hr);
}

void CTextService::UninitKeyEventSink() {
    if (m_pThreadMgr == nullptr) {
        return;
    }
    ITfKeystrokeMgr* pKeystrokeMgr = nullptr;
    if (SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfKeystrokeMgr,
                                               reinterpret_cast<void**>(&pKeystrokeMgr)))) {
        pKeystrokeMgr->UnadviseKeyEventSink(m_tfClientId);
        pKeystrokeMgr->Release();
    }
}

bool CTextService::InitThreadMgrEventSink() {
    ITfSource* pSource = nullptr;
    if (FAILED(m_pThreadMgr->QueryInterface(IID_ITfSource, reinterpret_cast<void**>(&pSource)))) {
        return false;
    }
    const HRESULT hr = pSource->AdviseSink(IID_ITfThreadMgrEventSink,
                                           static_cast<ITfThreadMgrEventSink*>(this),
                                           &m_dwThreadMgrEventSinkCookie);
    pSource->Release();
    if (FAILED(hr)) {
        m_dwThreadMgrEventSinkCookie = TF_INVALID_COOKIE;
        return false;
    }
    return true;
}

void CTextService::UninitThreadMgrEventSink() {
    if (m_pThreadMgr == nullptr || m_dwThreadMgrEventSinkCookie == TF_INVALID_COOKIE) {
        return;
    }
    ITfSource* pSource = nullptr;
    if (SUCCEEDED(m_pThreadMgr->QueryInterface(IID_ITfSource, reinterpret_cast<void**>(&pSource)))) {
        pSource->UnadviseSink(m_dwThreadMgrEventSinkCookie);
        pSource->Release();
    }
    m_dwThreadMgrEventSinkCookie = TF_INVALID_COOKIE;
}

STDMETHODIMP CTextService::OnSetFocus(BOOL /*fForeground*/) {
    return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyDown(ITfContext* /*pic*/,
                                         WPARAM wParam,
                                         LPARAM /*lParam*/,
                                         BOOL* pfEaten) {
    if (pfEaten == nullptr) {
        return E_INVALIDARG;
    }
    *pfEaten = IsAsciiLetterOrDigit(wParam) || wParam == VK_SPACE || wParam == VK_RETURN ||
               wParam == VK_BACK || wParam == VK_ESCAPE;
    return S_OK;
}

STDMETHODIMP CTextService::OnTestKeyUp(ITfContext* /*pic*/,
                                       WPARAM /*wParam*/,
                                       LPARAM /*lParam*/,
                                       BOOL* pfEaten) {
    if (pfEaten == nullptr) {
        return E_INVALIDARG;
    }
    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CTextService::OnKeyDown(ITfContext* pic,
                                     WPARAM wParam,
                                     LPARAM /*lParam*/,
                                     BOOL* pfEaten) {
    if (pfEaten == nullptr) {
        return E_INVALIDARG;
    }
    *pfEaten = FALSE;

    if (wParam == VK_ESCAPE) {
        HandleCancel(pic);
        *pfEaten = TRUE;
        return S_OK;
    }
    if (wParam == VK_BACK) {
        HandleBackspace(pic);
        *pfEaten = TRUE;
        return S_OK;
    }
    if (wParam == VK_RETURN || wParam == VK_SPACE) {
        HandleCommit(pic);
        *pfEaten = (wParam == VK_RETURN) ? TRUE : FALSE;
        // Space: commit then let the app receive space — refine when composition is live.
        if (wParam == VK_RETURN) {
            return S_OK;
        }
    }
    if (IsAsciiLetterOrDigit(wParam)) {
        HandleComposeChar(pic, WparamToAscii(wParam));
        *pfEaten = TRUE;
        return S_OK;
    }

    return S_OK;
}

STDMETHODIMP CTextService::OnKeyUp(ITfContext* /*pic*/,
                                   WPARAM /*wParam*/,
                                   LPARAM /*lParam*/,
                                   BOOL* pfEaten) {
    if (pfEaten == nullptr) {
        return E_INVALIDARG;
    }
    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CTextService::OnPreservedKey(ITfContext* /*pic*/, REFGUID /*rguid*/, BOOL* pfEaten) {
    if (pfEaten == nullptr) {
        return E_INVALIDARG;
    }
    *pfEaten = FALSE;
    return S_OK;
}

STDMETHODIMP CTextService::OnInitDocumentMgr(ITfDocumentMgr* /*pdim*/) {
    return S_OK;
}

STDMETHODIMP CTextService::OnUninitDocumentMgr(ITfDocumentMgr* /*pdim*/) {
    return S_OK;
}

STDMETHODIMP CTextService::OnSetFocus(ITfDocumentMgr* /*pdimFocus*/,
                                      ITfDocumentMgr* /*pdimPrevFocus*/) {
    return S_OK;
}

STDMETHODIMP CTextService::OnPushContext(ITfContext* /*pic*/) {
    return S_OK;
}

STDMETHODIMP CTextService::OnPopContext(ITfContext* /*pic*/) {
    return S_OK;
}

HRESULT CTextService::HandleComposeChar(ITfContext* pic, char chAscii) {
    if (pic == nullptr) {
        return E_INVALIDARG;
    }

    char buf[2] = {chAscii, '\0'};
    char* jsonUpdate = native_bangla_compose_char(buf);
    // TODO: Parse JSON CompositionUpdate (composed / commit_text / candidates)
    // and drive m_composition.UpdateComposition / m_candidateList.
    if (!m_composition.IsComposing()) {
        m_composition.StartComposition(pic);
    }
    // Placeholder: show ASCII until JSON parsing is wired.
    std::wstring preview(1, static_cast<wchar_t>(static_cast<unsigned char>(chAscii)));
    m_composition.UpdateComposition(pic, preview);
    FreeBridgeString(jsonUpdate);
    return S_OK;
}

HRESULT CTextService::HandleBackspace(ITfContext* pic) {
    char* jsonUpdate = native_bangla_compose_backspace();
    // TODO: Apply composed string from JSON; cancel if buffer empty.
    FreeBridgeString(jsonUpdate);
    if (pic != nullptr && m_composition.IsComposing()) {
        m_composition.UpdateComposition(pic, L"");
    }
    return S_OK;
}

HRESULT CTextService::HandleCommit(ITfContext* pic) {
    char* jsonUpdate = native_bangla_compose_commit();
    // TODO: Parse commit_text from JSON and call CommitComposition.
    FreeBridgeString(jsonUpdate);
    if (pic != nullptr) {
        m_composition.CommitComposition(pic, L"");
    }
    m_candidateList.Hide();
    return S_OK;
}

HRESULT CTextService::HandleCancel(ITfContext* pic) {
    char* jsonUpdate = native_bangla_compose_cancel();
    FreeBridgeString(jsonUpdate);
    if (pic != nullptr) {
        m_composition.CancelComposition(pic);
    }
    m_candidateList.Hide();
    m_candidateList.Clear();
    return S_OK;
}
