import { usePreferences } from "../lib/PreferencesContext";
import { SettingRow } from "../components/SettingRow";
import { Toggle } from "../components/Toggle";
import { SegmentedControl } from "../components/SegmentedControl";
import { CandidatePreview } from "../components/CandidatePreview";

const ACCENT_PRESETS = [
  { name: "Indigo", value: "#4F6BED" },
  { name: "Sky", value: "#0EA5E9" },
  { name: "Emerald", value: "#10B981" },
  { name: "Violet", value: "#8B5CF6" },
  { name: "Amber", value: "#F59E0B" },
  { name: "Rose", value: "#EC4899" },
];

export function AppearancePage() {
  const { prefs, updatePrefs } = usePreferences();
  const { appearance } = prefs;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Appearance</h1>
        <p className="mt-1 text-[13px] text-muted">
          Customize the visual theme, colors, and candidate bar for Native Bangla.
        </p>
      </div>

      <div className="nb-card p-5">
        <div className="mb-2 text-sm font-medium text-ink">Candidate Bar Preview</div>
        <p className="mb-4 text-[13px] text-muted">
          Real-time preview of the candidate suggestion bar with your current appearance settings.
        </p>
        <div className="flex items-center justify-center rounded-nb border border-border bg-surface/50 p-6">
          <CandidatePreview
            candidates={["বাংলাদেশ", "বাঙালি", "বাংলা", "বাঙাল"]}
            selectedIndex={0}
            opacity={appearance.candidate_opacity}
            fontSize={appearance.candidate_font_size}
            compact={appearance.compact_mode}
            animate={!appearance.reduced_motion}
          />
        </div>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <SettingRow
          title="Color Theme"
          description="Choose between light, dark, or follow your Windows system setting."
        >
          <SegmentedControl
            value={appearance.theme}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                appearance: { ...p.appearance, theme: v as typeof appearance.theme },
              }))
            }
            options={[
              { value: "light", label: "Light" },
              { value: "dark", label: "Dark" },
              { value: "system", label: "System" },
            ]}
          />
        </SettingRow>

        <SettingRow
          title="Accent Color"
          description="Select an accent color for highlights, active candidates, and badges."
        >
          <div className="flex items-center gap-2">
            {ACCENT_PRESETS.map((preset) => (
              <button
                key={preset.value}
                type="button"
                aria-label={`Select ${preset.name} accent`}
                onClick={() =>
                  updatePrefs((p) => ({
                    ...p,
                    appearance: { ...p.appearance, accent: preset.value },
                  }))
                }
                className="h-6 w-6 rounded-full transition-transform hover:scale-110 focus:outline-none focus:ring-2 focus:ring-offset-2"
                style={{
                  backgroundColor: preset.value,
                  boxShadow:
                    appearance.accent.toLowerCase() === preset.value.toLowerCase()
                      ? "0 0 0 2px var(--nb-surface), 0 0 0 4px " + preset.value
                      : "none",
                }}
              />
            ))}
          </div>
        </SettingRow>

        <SettingRow
          title="Candidate Bar Font Size"
          description={`${appearance.candidate_font_size}px — font size of Bengali candidates.`}
        >
          <div className="flex items-center gap-3">
            <input
              id="font-size-slider"
              type="range"
              min={12}
              max={22}
              step={1}
              value={appearance.candidate_font_size}
              onChange={(e) =>
                updatePrefs((p) => ({
                  ...p,
                  appearance: {
                    ...p.appearance,
                    candidate_font_size: Number(e.target.value),
                  },
                }))
              }
              className="h-2 w-32 cursor-pointer rounded-lg bg-border accent-accent"
            />
            <span className="w-8 text-right font-mono text-xs text-ink">
              {appearance.candidate_font_size}px
            </span>
          </div>
        </SettingRow>

        <SettingRow
          title="Window Opacity"
          description={`${Math.round(appearance.candidate_opacity * 100)}% — candidate bar background transparency.`}
        >
          <div className="flex items-center gap-3">
            <input
              id="opacity-slider"
              type="range"
              min={0.7}
              max={1.0}
              step={0.02}
              value={appearance.candidate_opacity}
              onChange={(e) =>
                updatePrefs((p) => ({
                  ...p,
                  appearance: {
                    ...p.appearance,
                    candidate_opacity: Number(e.target.value),
                  },
                }))
              }
              className="h-2 w-32 cursor-pointer rounded-lg bg-border accent-accent"
            />
            <span className="w-10 text-right font-mono text-xs text-ink">
              {Math.round(appearance.candidate_opacity * 100)}%
            </span>
          </div>
        </SettingRow>

        <SettingRow
          title="Compact Mode"
          description="Use smaller padding and minimal borders for a denser candidate window."
        >
          <Toggle
            checked={appearance.compact_mode}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                appearance: { ...p.appearance, compact_mode: v },
              }))
            }
            label="Compact mode"
          />
        </SettingRow>

        <SettingRow
          title="Reduced Motion"
          description="Disable animations and transitions for instant candidate display."
        >
          <Toggle
            checked={appearance.reduced_motion}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                appearance: { ...p.appearance, reduced_motion: v },
              }))
            }
            label="Reduced motion"
          />
        </SettingRow>
      </div>
    </div>
  );
}
