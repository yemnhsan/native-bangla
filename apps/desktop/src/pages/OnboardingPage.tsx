import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Check, ArrowRight, ShieldCheck, Zap } from "lucide-react";
import { usePreferences } from "../lib/PreferencesContext";
import { demoTransliterate } from "../lib/engine";
import { PRODUCT_NAME, TAGLINE } from "../lib/preferences";

export function OnboardingPage() {
  const { updatePrefs } = usePreferences();
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [selectedLayout, setSelectedLayout] = useState<"phonetic" | "probhat">("phonetic");
  const [testInput, setTestInput] = useState("ami banglay gaan gai");

  const finishOnboarding = () => {
    updatePrefs((p) => ({
      ...p,
      typing: {
        ...p.typing,
        layout_id: selectedLayout,
        default_language: selectedLayout === "phonetic" ? "bangla_phonetic" : "bangla_traditional",
      },
      onboarding_complete: true,
    }));
  };

  return (
    <div className="flex min-h-screen w-full items-center justify-center bg-[var(--nb-bg)] p-6">
      <div className="w-full max-w-xl">
        {/* Step indicator */}
        <div className="mb-8 flex items-center justify-center gap-2">
          {[1, 2, 3].map((i) => (
            <div
              key={i}
              className={`h-1.5 rounded-full transition-all duration-300 ${
                step === i
                  ? "w-8 bg-accent"
                  : step > i
                  ? "w-4 bg-accent/40"
                  : "w-4 bg-border"
              }`}
            />
          ))}
        </div>

        <div className="nb-card p-8 shadow-xl">
          <AnimatePresence mode="wait">
            {step === 1 && (
              <motion.div
                key="step1"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.2 }}
                className="space-y-6 text-center"
              >
                <div className="mx-auto flex h-16 w-16 items-center justify-center rounded-2xl bg-accent-soft text-accent">
                  <span className="font-serif text-3xl font-bold">ক</span>
                </div>
                <div>
                  <h1 className="text-2xl font-bold tracking-tight text-ink">
                    Welcome to {PRODUCT_NAME}
                  </h1>
                  <p className="mt-1.5 text-sm text-muted">{TAGLINE}</p>
                </div>

                <div className="grid grid-cols-2 gap-3 text-left">
                  <div className="rounded-nb border border-border bg-surface p-3.5 space-y-1">
                    <div className="flex items-center gap-2 text-accent text-sm font-semibold">
                      <Zap size={15} />
                      Fast & Responsive
                    </div>
                    <p className="text-[12px] text-muted">
                      Zero keystroke lag with our native C++ TSF engine and Rust prediction core.
                    </p>
                  </div>
                  <div className="rounded-nb border border-border bg-surface p-3.5 space-y-1">
                    <div className="flex items-center gap-2 text-emerald-600 dark:text-emerald-400 text-sm font-semibold">
                      <ShieldCheck size={15} />
                      100% Offline
                    </div>
                    <p className="text-[12px] text-muted">
                      Complete privacy. Everything runs on-device without any cloud connectivity.
                    </p>
                  </div>
                </div>

                <button
                  type="button"
                  onClick={() => setStep(2)}
                  className="nb-btn-primary flex w-full items-center justify-center gap-2 py-3 text-sm font-medium"
                >
                  Configure Keyboard
                  <ArrowRight size={16} />
                </button>
              </motion.div>
            )}

            {step === 2 && (
              <motion.div
                key="step2"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.2 }}
                className="space-y-6"
              >
                <div className="text-center">
                  <h2 className="text-xl font-bold tracking-tight text-ink">
                    Choose Your Preferred Layout
                  </h2>
                  <p className="mt-1 text-sm text-muted">
                    You can switch or customize layouts anytime in Settings.
                  </p>
                </div>

                <div className="space-y-3">
                  <label
                    onClick={() => setSelectedLayout("phonetic")}
                    className={`flex cursor-pointer items-start gap-4 rounded-nb border p-4 transition ${
                      selectedLayout === "phonetic"
                        ? "border-accent bg-accent-soft/30 shadow-sm"
                        : "border-border hover:bg-surface"
                    }`}
                  >
                    <div className="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full border border-border bg-surface">
                      {selectedLayout === "phonetic" && (
                        <div className="h-2.5 w-2.5 rounded-full bg-accent" />
                      )}
                    </div>
                    <div className="space-y-1">
                      <div className="flex items-center gap-2 text-sm font-semibold text-ink">
                        <span>Phonetic (Avro-style)</span>
                        <span className="rounded bg-accent-soft px-1.5 py-0.2 text-[10px] font-medium text-accent">
                          Recommended
                        </span>
                      </div>
                      <p className="text-[12px] leading-relaxed text-muted">
                        Type Bengali using English phonetics (e.g. type <code className="font-mono text-ink">ami</code> for <span className="bn font-medium text-ink">আমি</span>).
                      </p>
                    </div>
                  </label>

                  <label
                    onClick={() => setSelectedLayout("probhat")}
                    className={`flex cursor-pointer items-start gap-4 rounded-nb border p-4 transition ${
                      selectedLayout === "probhat"
                        ? "border-accent bg-accent-soft/30 shadow-sm"
                        : "border-border hover:bg-surface"
                    }`}
                  >
                    <div className="mt-0.5 flex h-5 w-5 shrink-0 items-center justify-center rounded-full border border-border bg-surface">
                      {selectedLayout === "probhat" && (
                        <div className="h-2.5 w-2.5 rounded-full bg-accent" />
                      )}
                    </div>
                    <div className="space-y-1">
                      <div className="text-sm font-semibold text-ink">Traditional (Probhat)</div>
                      <p className="text-[12px] leading-relaxed text-muted">
                        Fixed keyboard mapping standard inspired by Probhat and National Bengali layouts.
                      </p>
                    </div>
                  </label>
                </div>

                <div className="flex items-center gap-3">
                  <button
                    type="button"
                    onClick={() => setStep(1)}
                    className="nb-btn-ghost w-1/3 py-2.5 text-sm"
                  >
                    Back
                  </button>
                  <button
                    type="button"
                    onClick={() => setStep(3)}
                    className="nb-btn-primary flex w-2/3 items-center justify-center gap-2 py-2.5 text-sm"
                  >
                    Next: Test Drive
                    <ArrowRight size={16} />
                  </button>
                </div>
              </motion.div>
            )}

            {step === 3 && (
              <motion.div
                key="step3"
                initial={{ opacity: 0, y: 10 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -10 }}
                transition={{ duration: 0.2 }}
                className="space-y-6"
              >
                <div className="text-center">
                  <h2 className="text-xl font-bold tracking-tight text-ink">
                    Try Native Bangla
                  </h2>
                  <p className="mt-1 text-sm text-muted">
                    Test live transliteration right here before finishing setup.
                  </p>
                </div>

                <div className="space-y-3">
                  <div className="space-y-1.5">
                    <label htmlFor="onboard-test-input" className="text-xs font-medium text-muted">
                      Type Phonetic English:
                    </label>
                    <input
                      id="onboard-test-input"
                      type="text"
                      value={testInput}
                      onChange={(e) => setTestInput(e.target.value)}
                      className="nb-input"
                      placeholder="Type phonetic text here..."
                    />
                  </div>

                  <div className="space-y-1.5">
                    <span className="text-xs font-medium text-muted">Bengali Output:</span>
                    <div className="bn flex min-h-[50px] items-center rounded-nb border border-border bg-surface px-4 py-2.5 text-xl font-medium text-ink">
                      {demoTransliterate(testInput) || (
                        <span className="text-sm text-muted">Bengali text will appear here...</span>
                      )}
                    </div>
                  </div>
                </div>

                <div className="rounded-nb bg-surface p-3.5 space-y-2 border border-border text-[12px] text-muted">
                  <div className="font-semibold text-ink">Key Shortcuts to Remember:</div>
                  <div className="flex items-center justify-between">
                    <span>Toggle Language (Bangla / English)</span>
                    <kbd className="rounded bg-border px-1.5 py-0.5 font-mono text-[11px] text-ink">
                      Ctrl + Space
                    </kbd>
                  </div>
                  <div className="flex items-center justify-between">
                    <span>Force English Word</span>
                    <kbd className="rounded bg-border px-1.5 py-0.5 font-mono text-[11px] text-ink">
                      Ctrl + E
                    </kbd>
                  </div>
                  <div className="flex items-center justify-between">
                    <span>Accept Candidate Suggestion</span>
                    <kbd className="rounded bg-border px-1.5 py-0.5 font-mono text-[11px] text-ink">
                      Tab / 1–5
                    </kbd>
                  </div>
                </div>

                <div className="flex items-center gap-3">
                  <button
                    type="button"
                    onClick={() => setStep(2)}
                    className="nb-btn-ghost w-1/3 py-2.5 text-sm"
                  >
                    Back
                  </button>
                  <button
                    type="button"
                    onClick={finishOnboarding}
                    className="nb-btn-primary flex w-2/3 items-center justify-center gap-2 py-2.5 text-sm"
                  >
                    <Check size={16} />
                    Start Using {PRODUCT_NAME}
                  </button>
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
}
