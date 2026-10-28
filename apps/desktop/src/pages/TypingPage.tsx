import { usePreferences } from "../lib/PreferencesContext";
import { SettingRow } from "../components/SettingRow";
import { Toggle } from "../components/Toggle";
import { SegmentedControl } from "../components/SegmentedControl";

export function TypingPage() {
  const { prefs, updatePrefs } = usePreferences();
  const { typing } = prefs;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Typing</h1>
        <p className="mt-1 text-[13px] text-muted">Configure how Native Bangla handles keyboard input.</p>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <SettingRow title="Default Language" description="The input language active when you open a new text field.">
          <SegmentedControl
            value={typing.default_language}
            onChange={(v) => updatePrefs((p) => ({ ...p, typing: { ...p.typing, default_language: v as typeof typing.default_language } }))}
            options={[
              { value: "bangla_phonetic", label: "Phonetic" },
              { value: "bangla_traditional", label: "Traditional" },
              { value: "english", label: "English" },
            ]}
          />
        </SettingRow>

        <SettingRow title="Keyboard Layout" description="Choose the active traditional or phonetic layout.">
          <select
            value={typing.layout_id}
            onChange={(e) => updatePrefs((p) => ({ ...p, typing: { ...p.typing, layout_id: e.target.value } }))}
            className="nb-input w-auto min-w-[160px]"
            id="layout-select"
          >
            <option value="phonetic">Phonetic (Avro-style)</option>
            <option value="probhat">Probhat</option>
            <option value="national">National</option>
            <option value="inscript">InScript</option>
          </select>
        </SettingRow>

        <SettingRow title="Auto Space" description="Automatically add a space after committing a word with Tab.">
          <Toggle
            checked={typing.auto_space}
            onChange={(v) => updatePrefs((p) => ({ ...p, typing: { ...p.typing, auto_space: v } }))}
            label="Auto Space"
          />
        </SettingRow>

        <SettingRow title="Bengali Digits" description="Convert ASCII digits (0-9) to Bengali digits (০-৯) while in Bangla mode.">
          <Toggle
            checked={typing.bengali_digits}
            onChange={(v) => updatePrefs((p) => ({ ...p, typing: { ...p.typing, bengali_digits: v } }))}
            label="Bengali Digits"
          />
        </SettingRow>

        <SettingRow title="Convert Period to Danda" description="Replace . with the Bengali full stop । when typing in Bangla mode.">
          <Toggle
            checked={typing.convert_danda}
            onChange={(v) => updatePrefs((p) => ({ ...p, typing: { ...p.typing, convert_danda: v } }))}
            label="Convert Danda"
          />
        </SettingRow>
      </div>

      <div className="nb-card p-5">
        <div className="mb-2 text-sm font-medium text-ink">Mixed Language Handling</div>
        <p className="text-[13px] text-muted leading-relaxed">
          Native Bangla automatically preserves English when it detects URLs, email addresses, code identifiers,
          CamelCase tokens, and common technical terms. You can force English for the current word at any time
          using the configured shortcut (<kbd className="rounded bg-border px-1.5 py-0.5 text-[11px] font-mono text-ink">Ctrl+E</kbd> by default).
        </p>
      </div>
    </div>
  );
}
