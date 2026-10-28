import { NavLink } from "react-router-dom";
import clsx from "clsx";
import {
  Home,
  Keyboard,
  Sparkles,
  BookOpen,
  LayoutGrid,
  Command,
  Palette,
  Settings2,
  Info,
} from "lucide-react";
import { PRODUCT_NAME, TAGLINE } from "../lib/preferences";

const NAV = [
  { to: "/", label: "Home", icon: Home, end: true },
  { to: "/typing", label: "Typing", icon: Keyboard },
  { to: "/suggestions", label: "Suggestions", icon: Sparkles },
  { to: "/dictionary", label: "Dictionary", icon: BookOpen },
  { to: "/layouts", label: "Keyboard Layouts", icon: LayoutGrid },
  { to: "/shortcuts", label: "Shortcuts", icon: Command },
  { to: "/appearance", label: "Appearance", icon: Palette },
  { to: "/advanced", label: "Advanced", icon: Settings2 },
  { to: "/about", label: "About", icon: Info },
] as const;

export function Sidebar() {
  return (
    <aside className="flex w-[232px] shrink-0 flex-col border-r border-border bg-[var(--nb-sidebar)]">
      <div className="border-b border-border px-5 py-5">
        <div className="text-[15px] font-semibold tracking-tight text-ink">{PRODUCT_NAME}</div>
        <div className="mt-0.5 text-[11px] leading-snug text-muted">{TAGLINE}</div>
      </div>
      <nav className="flex flex-1 flex-col gap-0.5 overflow-y-auto p-2.5">
        {NAV.map(({ to, label, icon: Icon, ...rest }) => (
          <NavLink
            key={to}
            to={to}
            end={"end" in rest ? rest.end : false}
            className={({ isActive }) =>
              clsx(
                "flex items-center gap-2.5 rounded-nb px-3 py-2 text-[13px] font-medium transition",
                isActive
                  ? "bg-accent-soft text-accent"
                  : "text-muted hover:bg-[var(--nb-sidebar-hover)] hover:text-ink",
              )
            }
          >
            <Icon size={16} strokeWidth={1.75} />
            {label}
          </NavLink>
        ))}
      </nav>
    </aside>
  );
}
