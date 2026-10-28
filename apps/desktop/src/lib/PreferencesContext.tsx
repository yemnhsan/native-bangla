import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { getPreferences, savePreferences } from "./engine";
import {
  defaultPreferences,
  type NativeBanglaPreferences,
} from "./preferences";

interface PreferencesContextValue {
  prefs: NativeBanglaPreferences;
  ready: boolean;
  setPrefs: (next: NativeBanglaPreferences) => void;
  updatePrefs: (patch: (p: NativeBanglaPreferences) => NativeBanglaPreferences) => void;
  resetPrefs: () => void;
}

const PreferencesContext = createContext<PreferencesContextValue | null>(null);

function resolveTheme(theme: NativeBanglaPreferences["appearance"]["theme"]): "light" | "dark" {
  if (theme === "system") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return theme;
}

function applyAppearance(prefs: NativeBanglaPreferences) {
  const resolved = resolveTheme(prefs.appearance.theme);
  document.documentElement.setAttribute("data-theme", resolved);
  document.documentElement.style.setProperty("--nb-accent", prefs.appearance.accent);
  document.documentElement.style.setProperty(
    "--nb-accent-soft",
    `${prefs.appearance.accent}22`,
  );
}

export function PreferencesProvider({ children }: { children: ReactNode }) {
  const [prefs, setPrefsState] = useState<NativeBanglaPreferences>(defaultPreferences);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let cancelled = false;
    getPreferences().then((loaded) => {
      if (cancelled) return;
      setPrefsState(loaded);
      applyAppearance(loaded);
      setReady(true);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const setPrefs = useCallback((next: NativeBanglaPreferences) => {
    setPrefsState(next);
    applyAppearance(next);
    void savePreferences(next);
  }, []);

  const updatePrefs = useCallback(
    (patch: (p: NativeBanglaPreferences) => NativeBanglaPreferences) => {
      setPrefsState((prev) => {
        const next = patch(prev);
        applyAppearance(next);
        void savePreferences(next);
        return next;
      });
    },
    [],
  );

  const resetPrefs = useCallback(() => {
    const next = defaultPreferences();
    next.onboarding_complete = prefs.onboarding_complete;
    setPrefs(next);
  }, [prefs.onboarding_complete, setPrefs]);

  const value = useMemo(
    () => ({ prefs, ready, setPrefs, updatePrefs, resetPrefs }),
    [prefs, ready, setPrefs, updatePrefs, resetPrefs],
  );

  return (
    <PreferencesContext.Provider value={value}>{children}</PreferencesContext.Provider>
  );
}

export function usePreferences(): PreferencesContextValue {
  const ctx = useContext(PreferencesContext);
  if (!ctx) throw new Error("usePreferences must be used within PreferencesProvider");
  return ctx;
}
