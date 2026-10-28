/// <reference types="vite/client" />

declare module "lucide-react" {
  import { FC, SVGProps } from "react";
  export interface LucideProps extends SVGProps<SVGSVGElement> {
    size?: string | number;
    color?: string;
    strokeWidth?: string | number;
  }
  export type LucideIcon = FC<LucideProps>;
  export const Home: LucideIcon;
  export const Keyboard: LucideIcon;
  export const Sparkles: LucideIcon;
  export const BookOpen: LucideIcon;
  export const LayoutGrid: LucideIcon;
  export const Command: LucideIcon;
  export const Palette: LucideIcon;
  export const Settings2: LucideIcon;
  export const Info: LucideIcon;
  export const Activity: LucideIcon;
  export const Zap: LucideIcon;
  export const Globe: LucideIcon;
  export const Plus: LucideIcon;
  export const Trash2: LucideIcon;
  export const Search: LucideIcon;
  export const Check: LucideIcon;
  export const RotateCcw: LucideIcon;
  export const Download: LucideIcon;
  export const Upload: LucideIcon;
  export const AlertTriangle: LucideIcon;
  export const ShieldCheck: LucideIcon;
  export const Cpu: LucideIcon;
  export const Heart: LucideIcon;
  export const ExternalLink: LucideIcon;
  export const Code2: LucideIcon;
  export const ArrowRight: LucideIcon;
  const icons: Record<string, LucideIcon>;
  export default icons;
}
