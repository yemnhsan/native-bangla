import { ShieldCheck, Cpu, Sparkles, Globe, Heart, ExternalLink, Code2 } from "lucide-react";
import { PRODUCT_NAME, TAGLINE } from "../lib/preferences";

export function AboutPage() {
  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-xl font-semibold tracking-tight text-ink">About {PRODUCT_NAME}</h1>
        <p className="mt-1 text-[13px] text-muted">{TAGLINE}</p>
      </div>

      <div className="nb-card p-6">
        <div className="flex items-start gap-4">
          <div className="flex h-12 w-12 shrink-0 items-center justify-center rounded-2xl bg-accent-soft text-accent">
            <span className="font-serif text-2xl font-bold">ক</span>
          </div>
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <span className="text-lg font-semibold text-ink">{PRODUCT_NAME}</span>
              <span className="rounded-full bg-accent-soft px-2.5 py-0.5 text-xs font-medium text-accent">
                v0.1.0-alpha
              </span>
            </div>
            <p className="text-[13px] leading-relaxed text-muted">
              {PRODUCT_NAME} is a free, offline-first, intelligent Bengali keyboard and Windows IME
              inspired by the usability of Avro Keyboard and Bijoy Bayanno, with modern design, fast
              typing, and intelligent word prediction.
            </p>
          </div>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-4">
        <div className="nb-card p-5 space-y-2">
          <div className="flex items-center gap-2 text-accent">
            <Cpu size={16} />
            <span className="text-sm font-semibold text-ink">Native Engine</span>
          </div>
          <p className="text-[12px] leading-relaxed text-muted">
            High-performance C++20 Windows Text Services Framework (TSF) IME. Communicates directly
            with Windows text input pipelines without simulated keystrokes.
          </p>
        </div>

        <div className="nb-card p-5 space-y-2">
          <div className="flex items-center gap-2 text-accent">
            <Sparkles size={16} />
            <span className="text-sm font-semibold text-ink">Native Predict</span>
          </div>
          <p className="text-[12px] leading-relaxed text-muted">
            Rust-powered phonetic transliteration, trie dictionary indexer, and local bigram next-word
            prediction engine operating strictly under 5ms.
          </p>
        </div>

        <div className="nb-card p-5 space-y-2">
          <div className="flex items-center gap-2 text-emerald-600 dark:text-emerald-400">
            <ShieldCheck size={16} />
            <span className="text-sm font-semibold text-ink">100% Offline & Private</span>
          </div>
          <p className="text-[12px] leading-relaxed text-muted">
            Zero telemetry. No network permissions required. Your keystrokes and vocabulary models never
            leave your device.
          </p>
        </div>

        <div className="nb-card p-5 space-y-2">
          <div className="flex items-center gap-2 text-accent">
            <Globe size={16} />
            <span className="text-sm font-semibold text-ink">Modern Standards</span>
          </div>
          <p className="text-[12px] leading-relaxed text-muted">
            Full Unicode 15 Bengali script adherence with canonical conjunct formation (য-ফলা, র-ফলা,
            যুক্তবর্ণ), danda conversion, and Bengali digit mapping.
          </p>
        </div>
      </div>

      <div className="nb-card divide-y divide-border px-5">
        <div className="flex items-center justify-between py-3.5 text-[13px]">
          <span className="text-muted">Short Name</span>
          <span className="font-mono text-ink">Native</span>
        </div>
        <div className="flex items-center justify-between py-3.5 text-[13px]">
          <span className="text-muted">Internal Engine</span>
          <span className="font-mono text-ink">Native Engine</span>
        </div>
        <div className="flex items-center justify-between py-3.5 text-[13px]">
          <span className="text-muted">Prediction Engine</span>
          <span className="font-mono text-ink">Native Predict</span>
        </div>
        <div className="flex items-center justify-between py-3.5 text-[13px]">
          <span className="text-muted">License</span>
          <span className="text-ink">MIT License</span>
        </div>
      </div>

      <div className="flex items-center justify-between px-2 text-[12px] text-muted">
        <div className="flex items-center gap-1.5">
          <span>Crafted with</span>
          <Heart size={13} className="text-red-500 fill-red-500" />
          <span>for Bengali speakers worldwide.</span>
        </div>
        <div className="flex items-center gap-4">
          <a
            href="https://github.com/native-bangla/native-bangla"
            target="_blank"
            rel="noreferrer"
            className="flex items-center gap-1 hover:text-ink transition"
          >
            <Code2 size={13} />
            <span>GitHub Repository</span>
            <ExternalLink size={11} />
          </a>
        </div>
      </div>
    </div>
  );
}
