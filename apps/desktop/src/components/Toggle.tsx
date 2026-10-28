import clsx from "clsx";

interface ToggleProps {
  checked: boolean;
  onChange: (next: boolean) => void;
  label?: string;
  disabled?: boolean;
}

export function Toggle({ checked, onChange, label, disabled }: ToggleProps) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={label}
      disabled={disabled}
      onClick={() => onChange(!checked)}
      className={clsx(
        "relative h-6 w-11 rounded-full transition focus:outline-none focus-visible:ring-2 focus-visible:ring-accent-soft",
        checked ? "bg-accent" : "bg-border",
        disabled && "cursor-not-allowed opacity-50",
      )}
    >
      <span
        className={clsx(
          "absolute top-0.5 h-5 w-5 rounded-full bg-white shadow-sm transition",
          checked ? "left-[22px]" : "left-0.5",
        )}
      />
    </button>
  );
}
