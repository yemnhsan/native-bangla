/** Preferences types matching Native Bangla / user-preferences crate. */

export type InputLanguage = "english" | "bangla_phonetic" | "bangla_traditional";
export type ThemeMode = "light" | "dark" | "system";

export interface TypingSettings {
  default_language: InputLanguage;
  layout_id: string;
  auto_space: boolean;
  bengali_digits: boolean;
  convert_danda: boolean;
}

export interface SuggestionSettings {
  enabled: boolean;
  max_suggestions: number;
  next_word: boolean;
  autocorrect: boolean;
  autocorrect_aggressive: boolean;
  animations: boolean;
  number_key_select: boolean;
}

export interface AppearanceSettings {
  theme: ThemeMode;
  accent: string;
  candidate_opacity: number;
  candidate_font_size: number;
  compact_mode: boolean;
  reduced_motion: boolean;
}

export interface PersonalizationSettings {
  learning_enabled: boolean;
}

export interface AdvancedSettings {
  start_with_windows: boolean;
  minimize_to_tray: boolean;
  performance_mode: boolean;
  diagnostics: boolean;
}

export interface ShortcutSettings {
  toggle_language: string;
  force_english_word: string;
}

export interface NativeBanglaPreferences {
  version: number;
  typing: TypingSettings;
  suggestions: SuggestionSettings;
  appearance: AppearanceSettings;
  personalization: PersonalizationSettings;
  advanced: AdvancedSettings;
  shortcuts: ShortcutSettings;
  onboarding_complete: boolean;
}

export const PRODUCT_NAME = "Native Bangla";
export const TAGLINE = "Type Naturally. Write Beautifully.";
export const PREFS_STORAGE_KEY = "native-bangla.preferences";

export function defaultPreferences(): NativeBanglaPreferences {
  return {
    version: 1,
    typing: {
      default_language: "bangla_phonetic",
      layout_id: "phonetic",
      auto_space: true,
      bengali_digits: false,
      convert_danda: true,
    },
    suggestions: {
      enabled: true,
      max_suggestions: 5,
      next_word: true,
      autocorrect: true,
      autocorrect_aggressive: false,
      animations: true,
      number_key_select: true,
    },
    appearance: {
      theme: "system",
      accent: "#4F6BED",
      candidate_opacity: 0.96,
      candidate_font_size: 14,
      compact_mode: false,
      reduced_motion: false,
    },
    personalization: {
      learning_enabled: true,
    },
    advanced: {
      start_with_windows: true,
      minimize_to_tray: true,
      performance_mode: false,
      diagnostics: false,
    },
    shortcuts: {
      toggle_language: "Ctrl+Space",
      force_english_word: "Ctrl+E",
    },
    onboarding_complete: false,
  };
}

export function loadLocalPreferences(): NativeBanglaPreferences {
  try {
    const raw = localStorage.getItem(PREFS_STORAGE_KEY);
    if (!raw) return defaultPreferences();
    return { ...defaultPreferences(), ...JSON.parse(raw) } as NativeBanglaPreferences;
  } catch {
    return defaultPreferences();
  }
}

export function saveLocalPreferences(prefs: NativeBanglaPreferences): void {
  localStorage.setItem(PREFS_STORAGE_KEY, JSON.stringify(prefs));
}

export function languageLabel(lang: InputLanguage): string {
  switch (lang) {
    case "english":
      return "English";
    case "bangla_phonetic":
      return "Bangla Phonetic";
    case "bangla_traditional":
      return "Bangla Traditional";
  }
}

export function layoutLabel(id: string): string {
  switch (id) {
    case "phonetic":
      return "Phonetic (Avro-style)";
    case "probhat":
      return "Probhat";
    case "national":
      return "National";
    case "inscript":
      return "InScript";
    default:
      return id;
  }
}
