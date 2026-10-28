import clsx from "clsx";
import { motion } from "framer-motion";

interface CandidatePreviewProps {
  candidates: string[];
  selectedIndex?: number;
  opacity?: number;
  fontSize?: number;
  compact?: boolean;
  animate?: boolean;
}

export function CandidatePreview({
  candidates,
  selectedIndex = 0,
  opacity = 0.96,
  fontSize = 14,
  compact = false,
  animate = true,
}: CandidatePreviewProps) {
  const list = candidates.length ? candidates : ["আমি", "আমার", "আমরা"];

  return (
    <motion.div
      initial={animate ? { opacity: 0, y: 4 } : false}
      animate={{ opacity, y: 0 }}
      className={clsx(
        "nb-card overflow-hidden",
        compact ? "min-w-[160px]" : "min-w-[220px]",
      )}
      style={{ opacity }}
    >
      <ul className="py-1">
        {list.map((text, i) => (
          <li
            key={`${text}-${i}`}
            className={clsx(
              "bn flex items-center gap-2 px-3 transition",
              compact ? "py-1" : "py-1.5",
              i === selectedIndex
                ? "bg-accent-soft text-accent"
                : "text-ink",
            )}
            style={{ fontSize }}
          >
            <span className="w-4 text-[11px] font-medium text-muted">{i + 1}</span>
            <span className="font-medium">{text}</span>
          </li>
        ))}
      </ul>
    </motion.div>
  );
}
