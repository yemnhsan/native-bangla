/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: ["class", '[data-theme="dark"]'],
  theme: {
    extend: {
      colors: {
        surface: "var(--nb-surface)",
        elevated: "var(--nb-elevated)",
        ink: "var(--nb-ink)",
        muted: "var(--nb-muted)",
        border: "var(--nb-border)",
        accent: "var(--nb-accent)",
        "accent-soft": "var(--nb-accent-soft)",
      },
      fontFamily: {
        sans: ['"DM Sans"', "system-ui", "sans-serif"],
        bengali: ['"Noto Sans Bengali"', '"DM Sans"', "sans-serif"],
      },
      borderRadius: {
        nb: "var(--nb-radius)",
      },
      boxShadow: {
        nb: "var(--nb-shadow)",
      },
    },
  },
  plugins: [],
};
