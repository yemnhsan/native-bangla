import { useState } from "react";
import { usePreferences } from "../lib/PreferencesContext";
import { SettingRow } from "../components/SettingRow";
import { Toggle } from "../components/Toggle";
import { RotateCcw, Download, Upload, Check, AlertTriangle } from "lucide-react";
import { saveLocalPreferences, defaultPreferences } from "../lib/preferences";

export function AdvancedPage() {
  const { prefs, setPrefs, updatePrefs, resetPrefs } = usePreferences();
  const { advanced } = prefs;
  const [resetConfirmed, setResetConfirmed] = useState(false);
  const [exportNotice, setExportNotice] = useState(false);

  const handleExport = () => {
    const jsonStr = JSON.stringify(prefs, null, 2);
    const blob = new Blob([jsonStr], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "native-bangla-preferences.json";
    a.click();
    URL.revokeObjectURL(url);
    setExportNotice(true);
    setTimeout(() => setExportNotice(false), 2500);
  };

  const handleImport = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (event) => {
      try {
        const parsed = JSON.parse(event.target?.result as string);
        if (typeof parsed === "object" && parsed !== null) {
          const merged = { ...defaultPreferences(), ...parsed };
          setPrefs(merged);
          saveLocalPreferences(merged);
        }
      } catch (err) {
        console.error("Failed to parse imported preferences:", err);
      }
    };
    reader.readAsText(file);
  };

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Advanced</h1>
        <p className="mt-1 text-[13px] text-muted">
          System integration, background service behavior, and diagnostics.
        </p>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <SettingRow
          title="Start with Windows"
          description="Launch Native Bangla helper tray service automatically when Windows boots."
        >
          <Toggle
            checked={advanced.start_with_windows}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                advanced: { ...p.advanced, start_with_windows: v },
              }))
            }
            label="Start with Windows"
          />
        </SettingRow>

        <SettingRow
          title="Minimize to System Tray"
          description="Closing the window keeps Native Bangla active in the notification area."
        >
          <Toggle
            checked={advanced.minimize_to_tray}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                advanced: { ...p.advanced, minimize_to_tray: v },
              }))
            }
            label="Minimize to tray"
          />
        </SettingRow>

        <SettingRow
          title="Performance Mode"
          description="Keep linguistic tries and frequency caches compact to minimize RAM consumption."
        >
          <Toggle
            checked={advanced.performance_mode}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                advanced: { ...p.advanced, performance_mode: v },
              }))
            }
            label="Performance mode"
          />
        </SettingRow>

        <SettingRow
          title="Diagnostic Logging"
          description="Record input processing latency and engine events locally for troubleshooting."
        >
          <Toggle
            checked={advanced.diagnostics}
            onChange={(v) =>
              updatePrefs((p) => ({
                ...p,
                advanced: { ...p.advanced, diagnostics: v },
              }))
            }
            label="Diagnostic logging"
          />
        </SettingRow>
      </div>

      <div className="nb-card p-5 space-y-4">
        <div>
          <div className="text-sm font-medium text-ink">Backup & Restore</div>
          <p className="mt-0.5 text-[13px] text-muted">
            Export your settings and layout preferences as JSON, or import an existing configuration.
          </p>
        </div>
        <div className="flex flex-wrap items-center gap-3">
          <button
            type="button"
            onClick={handleExport}
            className="inline-flex items-center gap-2 rounded-nb border border-border bg-elevated px-3 py-2 text-[13px] font-medium text-ink hover:bg-surface transition"
          >
            {exportNotice ? <Check size={15} className="text-emerald-500" /> : <Download size={15} />}
            {exportNotice ? "Saved to file" : "Export Preferences"}
          </button>

          <label className="inline-flex cursor-pointer items-center gap-2 rounded-nb border border-border bg-elevated px-3 py-2 text-[13px] font-medium text-ink hover:bg-surface transition">
            <Upload size={15} />
            Import Preferences
            <input
              type="file"
              accept=".json"
              onChange={handleImport}
              className="hidden"
            />
          </label>
        </div>
      </div>

      <div className="nb-card border-red-500/20 p-5 space-y-3">
        <div className="flex items-center gap-2 text-red-600 dark:text-red-400">
          <AlertTriangle size={16} />
          <div className="text-sm font-semibold">Reset Configuration</div>
        </div>
        <p className="text-[13px] text-muted leading-relaxed">
          Reset all typing shortcuts, visual preferences, and suggestion parameters back to factory defaults.
          Custom learned words will be preserved.
        </p>
        <div className="pt-1">
          {resetConfirmed ? (
            <div className="flex items-center gap-3">
              <button
                type="button"
                onClick={() => {
                  resetPrefs();
                  setResetConfirmed(false);
                }}
                className="rounded-nb bg-red-600 px-3 py-1.5 text-[13px] font-medium text-white hover:bg-red-700 transition"
              >
                Yes, Reset All Defaults
              </button>
              <button
                type="button"
                onClick={() => setResetConfirmed(false)}
                className="rounded-nb border border-border px-3 py-1.5 text-[13px] font-medium text-muted hover:text-ink transition"
              >
                Cancel
              </button>
            </div>
          ) : (
            <button
              type="button"
              onClick={() => setResetConfirmed(true)}
              className="inline-flex items-center gap-2 rounded-nb border border-border px-3 py-1.5 text-[13px] font-medium text-ink hover:bg-surface transition"
            >
              <RotateCcw size={14} />
              Reset All Defaults…
            </button>
          )}
        </div>
      </div>
    </div>
  );
}
