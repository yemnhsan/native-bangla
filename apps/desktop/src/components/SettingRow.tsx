import type { ReactNode } from "react";

interface SettingRowProps {
  title: string;
  description?: string;
  children: ReactNode;
}

export function SettingRow({ title, description, children }: SettingRowProps) {
  return (
    <div className="flex items-start justify-between gap-6 border-b border-border py-4 last:border-b-0">
      <div className="min-w-0 flex-1">
        <div className="text-sm font-medium text-ink">{title}</div>
        {description ? (
          <p className="mt-0.5 text-[13px] leading-relaxed text-muted">{description}</p>
        ) : null}
      </div>
      <div className="flex shrink-0 items-center pt-0.5">{children}</div>
    </div>
  );
}
