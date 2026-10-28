import { usePreferences } from "../lib/PreferencesContext";
import { SettingRow } from "../components/SettingRow";
import { Toggle } from "../components/Toggle";

export function SuggestionsPage() {
  const { prefs, updatePrefs } = usePreferences();
  const { suggestions } = prefs;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Suggestions</h1>
        <p className="mt-1 text-[13px] text-muted">Control how Native Predict offers word suggestions.</p>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <SettingRow title="Word Suggestions" description="Show Bengali word candidates while you type phonetically.">
          <Toggle checked={suggestions.enabled} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, enabled: v } }))} label="Enable suggestions" />
        </SettingRow>

        <SettingRow title="Maximum Suggestions" description="Number of candidates shown in the suggestion bar (3–5 recommended).">
          <input
            id="max-suggestions-input"
            type="number"
            min={1}
            max={9}
            value={suggestions.max_suggestions}
            onChange={(e) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, max_suggestions: Math.max(1, Math.min(9, parseInt(e.target.value) || 5)) } }))}
            className="nb-input w-20 text-center"
          />
        </SettingRow>

        <SettingRow title="Next-word Prediction" description="Suggest likely next words based on sentence context using n-gram models.">
          <Toggle checked={suggestions.next_word} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, next_word: v } }))} label="Next-word prediction" />
        </SettingRow>

        <SettingRow title="Autocorrect" description="Automatically correct common typing mistakes before committing a word.">
          <Toggle checked={suggestions.autocorrect} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, autocorrect: v } }))} label="Autocorrect" />
        </SettingRow>

        <SettingRow title="Aggressive Autocorrect" description="More aggressively replace words even when the confidence is moderate. Disable if autocorrect changes words unexpectedly.">
          <Toggle checked={suggestions.autocorrect_aggressive} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, autocorrect_aggressive: v } }))} label="Aggressive autocorrect" disabled={!suggestions.autocorrect} />
        </SettingRow>

        <SettingRow title="Number Key Selection" description="Press 1–5 to select the corresponding suggestion in the candidate bar.">
          <Toggle checked={suggestions.number_key_select} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, number_key_select: v } }))} label="Number key selection" />
        </SettingRow>

        <SettingRow title="Animations" description="Animate the candidate bar appearance and highlight transitions.">
          <Toggle checked={suggestions.animations} onChange={(v) => updatePrefs((p) => ({ ...p, suggestions: { ...p.suggestions, animations: v } }))} label="Animations" />
        </SettingRow>
      </div>

      <div className="nb-card p-5">
        <div className="mb-2 text-sm font-medium text-ink">Candidate Selection Shortcuts</div>
        <div className="mt-3 space-y-2 text-[13px] text-muted">
          {[
            ["Tab", "Accept highlighted suggestion"],
            ["Arrow keys", "Navigate candidates (when bar is active)"],
            ["1–5", "Select numbered candidate (when enabled)"],
            ["Enter", "Commit composition"],
            ["Escape", "Dismiss candidate bar / cancel composition"],
            ["Space", "Commit current word (configurable)"],
          ].map(([key, desc]) => (
            <div key={key} className="flex items-center gap-3">
              <kbd className="min-w-[60px] rounded bg-border px-2 py-0.5 text-center text-[11px] font-mono text-ink">{key}</kbd>
              <span>{desc}</span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
