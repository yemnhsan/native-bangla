// Native Bangla — candidate UI integration stubs for Native Predict.
// Copyright (c) Native Bangla contributors. MIT License.

#pragma once

#include <msctf.h>
#include <windows.h>

#include <string>
#include <vector>

struct CandidateItem {
    std::wstring text;
    double score = 0.0;
    std::string source;  // e.g. "phonetic", "ngram", "personal"
};

class CCandidateList {
public:
    CCandidateList();
    ~CCandidateList();

    void Clear();
    void SetCandidates(std::vector<CandidateItem> items);
    const std::vector<CandidateItem>& Items() const;
    int SelectedIndex() const;
    void SetSelectedIndex(int index);

    /// Show / hide the candidate window near the composition caret.
    /// TODO: Integrate ITfCandidateListUIElement / custom HWND UI.
    HRESULT Show(ITfContext* pContext);
    HRESULT Hide();
    bool IsVisible() const;

    /// Apply the currently selected candidate as commit text.
    HRESULT SelectCurrent(std::wstring& outCommitText);

private:
    std::vector<CandidateItem> m_items;
    int m_selectedIndex;
    bool m_visible;
};
