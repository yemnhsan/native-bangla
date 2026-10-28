// Native Bangla — candidate UI stubs.
// Copyright (c) Native Bangla contributors. MIT License.

#include "CandidateList.h"

CCandidateList::CCandidateList() : m_selectedIndex(0), m_visible(false) {}

CCandidateList::~CCandidateList() {
    Hide();
}

void CCandidateList::Clear() {
    m_items.clear();
    m_selectedIndex = 0;
}

void CCandidateList::SetCandidates(std::vector<CandidateItem> items) {
    m_items = std::move(items);
    m_selectedIndex = m_items.empty() ? 0 : 0;
}

const std::vector<CandidateItem>& CCandidateList::Items() const {
    return m_items;
}

int CCandidateList::SelectedIndex() const {
    return m_selectedIndex;
}

void CCandidateList::SetSelectedIndex(int index) {
    if (m_items.empty()) {
        m_selectedIndex = 0;
        return;
    }
    if (index < 0) {
        m_selectedIndex = 0;
    } else if (index >= static_cast<int>(m_items.size())) {
        m_selectedIndex = static_cast<int>(m_items.size()) - 1;
    } else {
        m_selectedIndex = index;
    }
}

HRESULT CCandidateList::Show(ITfContext* pContext) {
    if (pContext == nullptr) {
        return E_INVALIDARG;
    }

    // TODO: Create or update candidate UI (ITfUIElementMgr / custom window)
    // positioned near the composition extent from ITfContextView.
    m_visible = !m_items.empty();
    return S_OK;
}

HRESULT CCandidateList::Hide() {
    // TODO: Tear down or hide candidate UI element.
    m_visible = false;
    return S_OK;
}

bool CCandidateList::IsVisible() const {
    return m_visible;
}

HRESULT CCandidateList::SelectCurrent(std::wstring& outCommitText) {
    outCommitText.clear();
    if (m_items.empty() || m_selectedIndex < 0 ||
        m_selectedIndex >= static_cast<int>(m_items.size())) {
        return E_FAIL;
    }
    outCommitText = m_items[static_cast<size_t>(m_selectedIndex)].text;
    return S_OK;
}
