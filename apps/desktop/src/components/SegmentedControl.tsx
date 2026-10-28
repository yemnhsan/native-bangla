import clsx from "clsx";

export interface SegmentOption<T extends string> {
  value: T;
  label: string;
}

interface SegmentedControlProps<T extends string> {
  value: T;
  options: SegmentOption<T>[];
  onChange: (value: T) => void;
  size?: "sm" | "md";
}

export function SegmentedControl<T extends string>({
  value,
  options,
  onChange,
  size = "md",
}: SegmentedControlProps<T>) {
  return (
    <div
      role="radiogroup"
      className="inline-flex rounded-nb border border-border bg-[var(--nb-surface)] p-0.5"
    >
      {options.map((opt) => {
        const active = opt.value === value;
        return (
          <button
            key={opt.value}
            type="button"
            role="radio"
            aria-checked={active}
            onClick={() => onChange(opt.value)}
            className={clsx(
              "rounded-[8px] font-medium transition",
              size === "sm" ? "px-2.5 py-1 text-xs" : "px-3 py-1.5 text-[13px]",
              active
                ? "bg-elevated text-ink shadow-sm"
                : "text-muted hover:text-ink",
            )}
          >
            {opt.label}
          </button>
        );
      })}
    </div>
  );
}
