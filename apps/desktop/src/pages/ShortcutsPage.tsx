import { useState } from "react";
import { usePreferences } from "../lib/PreferencesContext";

interface ShortcutRowProps {
  id: string;
  label: string;
  description: string;
  value: string;
  onChange: (v: string) => void;
}

function ShortcutRow({ id, label, description, value, onChange }: ShortcutRowProps) {
  const [editing, setEditing] = useState(false);
  const [draft, setDraft] = useState(value);

  return (
    <div className="flex items-start justify-between gap-6 border-b border-border py-4 last:border-b-0">
      <div className="min-w-0 flex-1">
        <div className="text-sm font-medium text-ink">{label}</div>
        <p className="mt-0.5 text-[13px] text-muted">{description}</p>
      </div>
      <div className="flex shrink-0 items-center gap-2">
        {editing ? (
          <>
            <input
              id={id}
              type="text"
              value={draft}
              onChange={(e) => setDraft(e.target.value)}
              className="nb-input w-36 font-mono text-[13px]"
              autoFocus
              placeholder="e.g. Ctrl+Space"
            />
            <button
              onClick={() => { onChange(draft); setEditing(false); }}
              className="nb-btn-primary text-[12px] px-2.5 py-1"
            >
              Save
            </button>
            <button
              onClick={() => { setDraft(value); setEditing(false); }}
              className="nb-btn-ghost text-[12px] px-2.5 py-1"
            >
              Cancel
            </button>
          </>
        ) : (
          <button
            id={id}
            onClick={() => { setDraft(value); setEditing(true); }}
            className="rounded-nb border border-border bg-elevated px-3 py-1.5 font-mono text-[13px] text-ink hover:border-accent/50 transition"
          >
            {value}
          </button>
        )}
      </div>
    </div>
  );
}

export function ShortcutsPage() {
  const { prefs, updatePrefs } = usePreferences();

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Shortcuts</h1>
        <p className="mt-1 text-[13px] text-muted">Assign keyboard shortcuts for Native Bangla actions.</p>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <ShortcutRow
          id="shortcut-toggle-language"
          label="Toggle Language"
          description="Switch between Bengali and English input modes instantly."
          value={prefs.shortcuts.toggle_language}
          onChange={(v) => updatePrefs((p) => ({ ...p, shortcuts: { ...p.shortcuts, toggle_language: v } }))}
        />
        <ShortcutRow
          id="shortcut-force-english"
          label="Force English Word"
          description="Type the current word in English without switching modes permanently."
          value={prefs.shortcuts.force_english_word}
          onChange={(v) => updatePrefs((p) => ({ ...p, shortcuts: { ...p.shortcuts, force_english_word: v } }))}
        />
      </div>

      <div className="nb-card p-5">
        <div className="mb-3 text-sm font-medium text-ink">Candidate Bar Shortcuts</div>
        <p className="mb-4 text-[13px] text-muted">These shortcuts are active only when the candidate bar is visible.</p>
        <div className="space-y-2.5 text-[13px]">
          {[
            ["Tab", "Accept highlighted suggestion"],
            ["↑ / ↓", "Navigate candidate list"],
            ["← / →", "Navigate inline candidates"],
            ["1–5", "Select numbered candidate (when enabled)"],
            ["Enter", "Commit current composition"],
            ["Escape", "Dismiss bar / cancel composition"],
            ["Space", "Commit word and insert space"],
            ["Backspace", "Delete last phonetic character"],
          ].map(([k, d]) => (
            <div key={k} className="flex items-center gap-3 text-muted">
              <kbd className="min-w-[80px] rounded bg-border px-2 py-1 text-center text-[11px] font-mono text-ink shrink-0">{k}</kbd>
              <span>{d}</span>
            </div>
          ))}
        </div>
      </div>

      <div className="rounded-nb border border-amber-200 bg-amber-50 p-4 text-[13px] text-amber-800 dark:border-amber-800/40 dark:bg-amber-900/20 dark:text-amber-300">
        <strong>Note:</strong> Shortcuts must not conflict with system-reserved key combinations. Native Bangla respects
        Windows input boundaries and will not override shortcuts in elevated processes.
      </div>
    </div>
  );
}
