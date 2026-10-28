import { useState } from "react";
import { motion } from "framer-motion";
import { Check } from "lucide-react";
import { usePreferences } from "../lib/PreferencesContext";

const LAYOUTS = [
  {
    id: "phonetic",
    name: "Smart Phonetic",
    description: "Type Bengali using English phonetics. ami → আমি. Avro-style rules with intelligent word overrides.",
    badge: "Recommended",
    preview: ["a → আ", "sh → শ", "kh → খ", "bh/v → ভ", "ng → ং"],
  },
  {
    id: "probhat",
    name: "Probhat",
    description: "Open-source fixed layout designed for Bengali. Uses dedicated key positions for each character.",
    badge: "Fixed",
    preview: ["j → ব", "w → ই", "f → আ", "d → ম", "s → ন"],
  },
  {
    id: "national",
    name: "National",
    description: "Bangladesh national standard keyboard layout (BSTI). Compatible with official government usage.",
    badge: "Standard",
    preview: ["Fixed layout", "BSTI standard", "For official use"],
  },
  {
    id: "inscript",
    name: "InScript",
    description: "Indian government standardised layout for Indic scripts. Common in Indian Bengali typing.",
    badge: "InScript",
    preview: ["ক on k", "ল on l", "Standard for India"],
  },
];

export function LayoutsPage() {
  const { prefs, updatePrefs } = usePreferences();
  const [activeTab, setActiveTab] = useState<"select" | "custom">("select");

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Keyboard Layouts</h1>
        <p className="mt-1 text-[13px] text-muted">Choose a built-in layout or create a custom key mapping.</p>
      </div>

      <div className="flex gap-1 rounded-nb border border-border bg-surface p-1">
        {(["select", "custom"] as const).map((t) => (
          <button
            key={t}
            id={`layout-tab-${t}`}
            onClick={() => setActiveTab(t)}
            className={`flex-1 rounded-[6px] py-1.5 text-[13px] font-medium transition ${
              activeTab === t ? "bg-elevated shadow-sm text-ink" : "text-muted hover:text-ink"
            }`}
          >
            {t === "select" ? "Built-in Layouts" : "Custom Layout"}
          </button>
        ))}
      </div>

      {activeTab === "select" && (
        <div className="space-y-3">
          {LAYOUTS.map((layout) => {
            const isActive = prefs.typing.layout_id === layout.id;
            return (
              <motion.button
                key={layout.id}
                id={`layout-option-${layout.id}`}
                initial={{ opacity: 0, y: 6 }}
                animate={{ opacity: 1, y: 0 }}
                onClick={() => updatePrefs((p) => ({ ...p, typing: { ...p.typing, layout_id: layout.id } }))}
                className={`w-full text-left rounded-nb border p-4 transition ${
                  isActive
                    ? "border-accent bg-accent-soft"
                    : "border-border bg-elevated hover:border-accent/40"
                }`}
              >
                <div className="flex items-start justify-between">
                  <div className="flex-1">
                    <div className="flex items-center gap-2">
                      <span className="text-[15px] font-medium text-ink">{layout.name}</span>
                      <span className="rounded-full bg-border px-2 py-0.5 text-[10px] text-muted">{layout.badge}</span>
                    </div>
                    <p className="mt-1 text-[13px] text-muted leading-relaxed">{layout.description}</p>
                    <div className="mt-3 flex flex-wrap gap-1.5">
                      {layout.preview.map((p) => (
                        <span key={p} className="rounded bg-border/60 px-2 py-0.5 font-mono text-[11px] text-muted">{p}</span>
                      ))}
                    </div>
                  </div>
                  {isActive && (
                    <div className="ml-4 flex h-6 w-6 shrink-0 items-center justify-center rounded-full bg-accent">
                      <Check size={13} className="text-white" />
                    </div>
                  )}
                </div>
              </motion.button>
            );
          })}
        </div>
      )}

      {activeTab === "custom" && (
        <div className="nb-card p-6">
          <p className="text-[13px] text-muted leading-relaxed">
            Custom layout editor lets you remap individual keys, configure Shift combinations, and save named layouts. 
            You can export layouts as JSON files and import them on other machines.
          </p>
          <div className="mt-6 rounded-nb border border-dashed border-border p-8 text-center">
            <div className="mb-2 text-[13px] font-medium text-ink">Visual Key Mapper</div>
            <p className="text-[12px] text-muted">Click any key to assign a Bengali character. Shift+key mapping supported.</p>
            <button id="create-custom-layout-btn" className="mt-4 nb-btn-primary text-[13px]">
              Create New Layout
            </button>
          </div>
          <div className="mt-4 flex gap-2">
            <button id="import-layout-btn" className="nb-btn-ghost text-[13px]">Import layout (.json)</button>
            <button id="export-layout-btn" className="nb-btn-ghost text-[13px]">Export current layout</button>
          </div>
        </div>
      )}
    </div>
  );
}
