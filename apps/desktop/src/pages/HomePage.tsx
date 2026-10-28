import { useEffect, useState } from "react";
import { motion } from "framer-motion";
import { Keyboard, Sparkles, BookOpen, Activity, Zap, Globe } from "lucide-react";
import { usePreferences } from "../lib/PreferencesContext";
import { demoTransliterate, getStatus, type EngineStatus } from "../lib/engine";
import { languageLabel } from "../lib/preferences";

const CARD_VARIANTS = {
  hidden: { opacity: 0, y: 10 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.06, duration: 0.22, ease: "easeOut" },
  }),
};

function StatusBadge({ active }: { active: boolean }) {
  return (
    <span
      className={`inline-flex items-center gap-1.5 rounded-full px-2.5 py-0.5 text-[11px] font-medium ${
        active ? "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400" : "bg-muted/10 text-muted"
      }`}
    >
      <span
        className={`h-1.5 w-1.5 rounded-full ${active ? "bg-emerald-500" : "bg-muted"}`}
      />
      {active ? "Active" : "Inactive"}
    </span>
  );
}

interface StatCardProps {
  icon: React.ReactNode;
  label: string;
  value: string | number;
  sub?: string;
  index: number;
}

function StatCard({ icon, label, value, sub, index }: StatCardProps) {
  return (
    <motion.div
      custom={index}
      variants={CARD_VARIANTS}
      initial="hidden"
      animate="visible"
      className="nb-card flex flex-col gap-3 p-5"
    >
      <div className="flex h-9 w-9 items-center justify-center rounded-nb bg-accent-soft text-accent">
        {icon}
      </div>
      <div>
        <div className="text-2xl font-semibold tracking-tight text-ink">{value}</div>
        <div className="mt-0.5 text-[13px] text-muted">{label}</div>
        {sub && <div className="mt-0.5 text-[11px] text-muted">{sub}</div>}
      </div>
    </motion.div>
  );
}

export function HomePage() {
  const { prefs } = usePreferences();
  const [status, setStatus] = useState<EngineStatus | null>(null);
  const [testInput, setTestInput] = useState("");
  const [testOutput, setTestOutput] = useState("");

  useEffect(() => {
    getStatus().then(setStatus).catch(console.error);
  }, []);

  useEffect(() => {
    if (!testInput) { setTestOutput(""); return; }
    const id = setTimeout(() => setTestOutput(demoTransliterate(testInput)), 30);
    return () => clearTimeout(id);
  }, [testInput]);

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">Native Bangla</h1>
        <p className="mt-1 text-[13px] text-muted">Type Naturally. Write Beautifully.</p>
      </div>
      <motion.div
        initial={{ opacity: 0, y: 6 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.2 }}
        className="nb-card flex items-center justify-between px-5 py-4"
      >
        <div className="flex items-center gap-3">
          <div className="flex h-8 w-8 items-center justify-center rounded-full bg-accent-soft">
            <Keyboard size={15} className="text-accent" />
          </div>
          <div>
            <div className="text-sm font-medium text-ink">Native Engine</div>
            <div className="text-[12px] text-muted">{status?.version ?? "0.1.0"} · {languageLabel(prefs.typing.default_language)}</div>
          </div>
        </div>
        <StatusBadge active />
      </motion.div>
      <div className="grid grid-cols-3 gap-4">
        <StatCard index={0} icon={<Keyboard size={16} />} label="Active Layout" value={prefs.typing.layout_id === "phonetic" ? "Phonetic" : prefs.typing.layout_id} sub="Avro-style" />
        <StatCard index={1} icon={<Sparkles size={16} />} label="Suggestions" value={prefs.suggestions.enabled ? "On" : "Off"} sub={`Max ${prefs.suggestions.max_suggestions}`} />
        <StatCard index={2} icon={<BookOpen size={16} />} label="Dictionary" value={status?.dictionary_words ?? "—"} sub="words loaded" />
      </div>
      <div className="grid grid-cols-3 gap-4">
        <StatCard index={3} icon={<Activity size={16} />} label="Next-word" value={prefs.suggestions.next_word ? "Enabled" : "Off"} />
        <StatCard index={4} icon={<Zap size={16} />} label="Autocorrect" value={prefs.suggestions.autocorrect ? "On" : "Off"} />
        <StatCard index={5} icon={<Globe size={16} />} label="Language switch" value={prefs.shortcuts.toggle_language} />
      </div>
      <motion.div initial={{ opacity: 0, y: 8 }} animate={{ opacity: 1, y: 0 }} transition={{ duration: 0.25, delay: 0.3 }} className="nb-card p-5">
        <div className="mb-3 text-sm font-medium text-ink">Live Typing Test</div>
        <p className="mb-4 text-[13px] text-muted">Try: <em>ami banglay likhte chai</em></p>
        <div className="space-y-2">
          <input id="home-typing-test" type="text" value={testInput} onChange={(e) => setTestInput(e.target.value)} placeholder="Type phonetic English…" className="nb-input" autoComplete="off" spellCheck={false} />
          <div className="bn min-h-[44px] rounded-nb border border-border bg-surface px-3 py-2.5 text-[18px] leading-relaxed text-ink" aria-label="Bengali output">
            {testOutput || <span className="text-[14px] text-muted">Bengali output appears here…</span>}
          </div>
        </div>
      </motion.div>
    </div>
  );
}
