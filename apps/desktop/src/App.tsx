import { Navigate, Route, Routes, useLocation } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { Sidebar } from "./components/Sidebar";
import { usePreferences } from "./lib/PreferencesContext";
import { HomePage } from "./pages/HomePage";
import { TypingPage } from "./pages/TypingPage";
import { SuggestionsPage } from "./pages/SuggestionsPage";
import { DictionaryPage } from "./pages/DictionaryPage";
import { LayoutsPage } from "./pages/LayoutsPage";
import { ShortcutsPage } from "./pages/ShortcutsPage";
import { AppearancePage } from "./pages/AppearancePage";
import { AdvancedPage } from "./pages/AdvancedPage";
import { AboutPage } from "./pages/AboutPage";
import { OnboardingPage } from "./pages/OnboardingPage";

function SettingsShell() {
  const location = useLocation();

  return (
    <div className="flex h-full min-h-0">
      <Sidebar />
      <main className="min-w-0 flex-1 overflow-y-auto">
        <AnimatePresence mode="wait">
          <motion.div
            key={location.pathname}
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -4 }}
            transition={{ duration: 0.18, ease: "easeOut" }}
            className="mx-auto max-w-3xl px-8 py-8"
          >
            <Routes location={location}>
              <Route path="/" element={<HomePage />} />
              <Route path="/typing" element={<TypingPage />} />
              <Route path="/suggestions" element={<SuggestionsPage />} />
              <Route path="/dictionary" element={<DictionaryPage />} />
              <Route path="/layouts" element={<LayoutsPage />} />
              <Route path="/shortcuts" element={<ShortcutsPage />} />
              <Route path="/appearance" element={<AppearancePage />} />
              <Route path="/advanced" element={<AdvancedPage />} />
              <Route path="/about" element={<AboutPage />} />
              <Route path="*" element={<Navigate to="/" replace />} />
            </Routes>
          </motion.div>
        </AnimatePresence>
      </main>
    </div>
  );
}

export default function App() {
  const { prefs, ready } = usePreferences();

  if (!ready) {
    return (
      <div className="flex h-full items-center justify-center text-sm text-muted">
        Loading Native Bangla…
      </div>
    );
  }

  if (!prefs.onboarding_complete) {
    return <OnboardingPage />;
  }

  return <SettingsShell />;
}
