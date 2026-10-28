import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Plus, Trash2, Search } from "lucide-react";
import { usePreferences } from "../lib/PreferencesContext";
import { Toggle } from "../components/Toggle";

const SAMPLE_WORDS = [
  { word: "বাংলাদেশ", phonetic: "bangladesh", frequency: 2341 },
  { word: "আমি", phonetic: "ami", frequency: 8821 },
  { word: "ভালোবাসা", phonetic: "valobasha", frequency: 1203 },
  { word: "শিক্ষা", phonetic: "shikkha", frequency: 987 },
  { word: "স্বাধীনতা", phonetic: "shadhinota", frequency: 756 },
  { word: "সুন্দর", phonetic: "shundor", frequency: 1544 },
];

export function DictionaryPage() {
  const { prefs, updatePrefs } = usePreferences();
  const [query, setQuery] = useState("");
  const [customWord, setCustomWord] = useState("");
  const [customPhonetic, setCustomPhonetic] = useState("");

  const filtered = SAMPLE_WORDS.filter(
    (w) => !query || w.word.includes(query) || w.phonetic.includes(query.toLowerCase())
  );

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Dictionary</h1>
        <p className="mt-1 text-[13px] text-muted">
          Manage the vocabulary used by Native Predict for suggestions.
        </p>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <div className="flex items-center justify-between py-4">
          <div>
            <div className="text-sm font-medium text-ink">Local Learning</div>
            <p className="mt-0.5 text-[13px] text-muted">Remember word choices to improve future suggestions.</p>
          </div>
          <Toggle
            checked={prefs.personalization.learning_enabled}
            onChange={(v) => updatePrefs((p) => ({ ...p, personalization: { ...p.personalization, learning_enabled: v } }))}
            label="Local learning"
          />
        </div>
        <div className="py-4">
          <div className="text-sm font-medium text-ink">Dictionary Info</div>
          <div className="mt-2 grid grid-cols-3 gap-3">
            {[["Core words", "128+"], ["User words", "0"], ["Frequency model", "Bigram"]].map(([k, v]) => (
              <div key={k} className="rounded-nb bg-surface p-3">
                <div className="text-lg font-semibold text-ink">{v}</div>
                <div className="text-[12px] text-muted">{k}</div>
              </div>
            ))}
          </div>
        </div>
      </div>

      <div className="nb-card p-5">
        <div className="mb-4 text-sm font-medium text-ink">Add Custom Word</div>
        <div className="flex gap-2">
          <input
            id="custom-bangla-input"
            type="text"
            value={customWord}
            onChange={(e) => setCustomWord(e.target.value)}
            placeholder="Bengali word (e.g. বাংলা)"
            className="nb-input bn flex-1 text-[16px]"
          />
          <input
            id="custom-phonetic-input"
            type="text"
            value={customPhonetic}
            onChange={(e) => setCustomPhonetic(e.target.value)}
            placeholder="Phonetic (e.g. bangla)"
            className="nb-input flex-1"
          />
          <button
            id="add-custom-word-btn"
            disabled={!customWord.trim()}
            onClick={() => { setCustomWord(""); setCustomPhonetic(""); }}
            className="nb-btn-primary flex items-center gap-1.5 disabled:opacity-50"
          >
            <Plus size={15} />
            Add
          </button>
        </div>
      </div>

      <div className="nb-card p-5">
        <div className="mb-4 flex items-center gap-3">
          <div className="text-sm font-medium text-ink">Word Browser</div>
          <div className="relative ml-auto w-52">
            <Search size={13} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted" />
            <input
              id="dict-search-input"
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search…"
              className="nb-input pl-8"
            />
          </div>
        </div>
        <div className="divide-y divide-border">
          <AnimatePresence>
            {filtered.map((entry, i) => (
              <motion.div
                key={entry.word}
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ delay: i * 0.03 }}
                className="flex items-center justify-between py-3"
              >
                <div className="flex items-center gap-3">
                  <span className="bn text-[18px] text-ink">{entry.word}</span>
                  <span className="text-[12px] text-muted">{entry.phonetic}</span>
                </div>
                <div className="flex items-center gap-3">
                  <span className="text-[12px] text-muted">f:{entry.frequency}</span>
                  <button
                    aria-label={`Remove ${entry.word}`}
                    className="text-muted hover:text-red-500 transition"
                    onClick={() => {}}
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              </motion.div>
            ))}
          </AnimatePresence>
          {filtered.length === 0 && (
            <p className="py-6 text-center text-[13px] text-muted">No words found.</p>
          )}
        </div>
      </div>

      <div className="flex gap-3">
        <button id="export-dict-btn" className="nb-btn-ghost text-[13px]">Export dictionary</button>
        <button id="reset-learning-btn" className="nb-btn-ghost text-[13px] text-red-500 hover:border-red-300">Reset learned data</button>
      </div>
    </div>
  );
}
