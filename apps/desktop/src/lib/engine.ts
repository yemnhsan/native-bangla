/**
 * Engine bridge for Native Bangla.
 * Tries Tauri invoke when available; falls back to a JS phonetic demo for browser preview.
 */

import {
  defaultPreferences,
  loadLocalPreferences,
  saveLocalPreferences,
  type NativeBanglaPreferences,
} from "./preferences";

export interface EngineStatus {
  product: string;
  engine: string;
  predict: string;
  version: string;
  mode: string;
  dictionary_words: number;
  suggestions_enabled: boolean;
}

declare global {
  interface Window {
    __TAURI__?: {
      core?: {
        invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
      };
      invoke?: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
    };
  }
}

function getInvoke():
  | (<T>(cmd: string, args?: Record<string, unknown>) => Promise<T>)
  | null {
  const t = window.__TAURI__;
  if (!t) return null;
  if (t.core?.invoke) return t.core.invoke.bind(t.core);
  if (t.invoke) return t.invoke.bind(t);
  return null;
}

/** Common Avro-style word map for browser-only demo (ami → আমি). */
const WORD_MAP: Record<string, string> = {
  ami: "আমি",
  tumi: "তুমি",
  amar: "আমার",
  apni: "আপনি",
  apnar: "আপনার",
  bangla: "বাংলা",
  bangladesh: "বাংলাদেশ",
  bhalo: "ভালো",
  valo: "ভালো",
  shundor: "সুন্দর",
  sundor: "সুন্দর",
  valobasha: "ভালোবাসা",
  bhalobasha: "ভালোবাসা",
  kemon: "কেমন",
  acho: "আছো",
  achho: "আছো",
  ase: "আছে",
  ache: "আছে",
  hobe: "হবে",
  korbo: "করব",
  kore: "করে",
  kora: "করা",
  likhte: "লিখতে",
  chai: "চাই",
  ekta: "একটা",
  ekti: "একটি",
  aj: "আজ",
  aaj: "আজ",
  kal: "কাল",
  khub: "খুব",
  onek: "অনেক",
  shikkha: "শিক্ষা",
  shiksha: "শিক্ষা",
  nam: "নাম",
  ki: "কি",
  kena: "কেন",
  kothay: "কোথায়",
  dhonnobad: "ধন্যবাদ",
  native: "নেটিভ",
};

const CONSONANTS: [string, string][] = [
  ["ksh", "ক্ষ"],
  ["chh", "ছ"],
  ["ch", "চ"],
  ["jh", "ঝ"],
  ["th", "থ"],
  ["dh", "ধ"],
  ["ph", "ফ"],
  ["bh", "ভ"],
  ["sh", "শ"],
  ["kh", "খ"],
  ["gh", "ঘ"],
  ["ng", "ং"],
  ["k", "ক"],
  ["g", "গ"],
  ["c", "চ"],
  ["j", "জ"],
  ["t", "ত"],
  ["d", "দ"],
  ["n", "ন"],
  ["p", "প"],
  ["f", "ফ"],
  ["b", "ব"],
  ["v", "ভ"],
  ["m", "ম"],
  ["z", "জ"],
  ["r", "র"],
  ["l", "ল"],
  ["s", "স"],
  ["h", "হ"],
  ["y", "য়"],
  ["w", "ও"],
];

const VOWELS: [string, string][] = [
  ["ou", "ঔ"],
  ["oi", "ঐ"],
  ["oo", "উ"],
  ["ee", "ঈ"],
  ["aa", "আ"],
  ["a", "আ"],
  ["i", "ই"],
  ["u", "উ"],
  ["e", "এ"],
  ["o", "ও"],
];

const KAR: Record<string, string> = {
  আ: "া",
  ই: "ি",
  ঈ: "ী",
  উ: "ু",
  ঊ: "ূ",
  এ: "ে",
  ঐ: "ৈ",
  ও: "ো",
  ঔ: "ৌ",
};

function matchLongest(input: string, table: [string, string][]): [string, string] | null {
  for (const [pat, rep] of table) {
    if (input.toLowerCase().startsWith(pat.toLowerCase()) || input.startsWith(pat)) {
      if (input.slice(0, pat.length).toLowerCase() === pat.toLowerCase()) {
        return [pat, rep];
      }
    }
  }
  return null;
}

function demoTransliterateToken(input: string): string {
  const lower = input.toLowerCase();
  if (WORD_MAP[lower]) return WORD_MAP[lower];

  let out = "";
  let i = 0;
  let lastWasConsonant = false;
  const chars = [...input];

  while (i < chars.length) {
    const remaining = chars.slice(i).join("");

    if (remaining[0] === ".") {
      out += "।";
      i += 1;
      lastWasConsonant = false;
      continue;
    }

    const cons = matchLongest(remaining, CONSONANTS);
    if (cons) {
      out += cons[1];
      i += cons[0].length;
      lastWasConsonant = true;
      continue;
    }

    const vow = matchLongest(remaining, VOWELS);
    if (vow) {
      if (lastWasConsonant && KAR[vow[1]] !== undefined) {
        out += KAR[vow[1]];
      } else {
        out += vow[1];
      }
      i += vow[0].length;
      lastWasConsonant = false;
      continue;
    }

    out += remaining[0];
    i += 1;
    lastWasConsonant = false;
  }

  return out;
}

/** Browser-only phonetic demo used when Tauri is unavailable. */
export function demoTransliterate(input: string): string {
  if (!input) return "";
  return input
    .split(/(\s+)/)
    .map((part) => (/\s/.test(part) ? part : demoTransliterateToken(part)))
    .join("");
}

function demoStatus(prefs: NativeBanglaPreferences): EngineStatus {
  const mode =
    prefs.typing.default_language === "english"
      ? "English"
      : prefs.typing.default_language === "bangla_traditional"
        ? "Traditional"
        : "Phonetic";
  return {
    product: "Native Bangla",
    engine: "Native Engine",
    predict: "Native Predict",
    version: "0.1.0-demo",
    mode,
    dictionary_words: 128,
    suggestions_enabled: prefs.suggestions.enabled,
  };
}

export async function transliterate(input: string): Promise<string> {
  const invoke = getInvoke();
  if (invoke) {
    try {
      return await invoke<string>("transliterate", { input });
    } catch {
      /* fall through to demo */
    }
  }
  return demoTransliterate(input);
}

export async function getStatus(): Promise<EngineStatus> {
  const invoke = getInvoke();
  if (invoke) {
    try {
      return await invoke<EngineStatus>("get_status");
    } catch {
      /* fall through */
    }
  }
  return demoStatus(loadLocalPreferences());
}

export async function getPreferences(): Promise<NativeBanglaPreferences> {
  const invoke = getInvoke();
  if (invoke) {
    try {
      return await invoke<NativeBanglaPreferences>("get_preferences");
    } catch {
      /* fall through */
    }
  }
  return loadLocalPreferences();
}

export async function savePreferences(
  prefs: NativeBanglaPreferences,
): Promise<NativeBanglaPreferences> {
  const invoke = getInvoke();
  if (invoke) {
    try {
      await invoke("save_preferences", { prefs });
      return prefs;
    } catch {
      /* fall through */
    }
  }
  saveLocalPreferences(prefs);
  return prefs;
}

export function isTauriRuntime(): boolean {
  return getInvoke() !== null;
}

export { defaultPreferences };
