/**
 * Taurigo design tokens — the single source of truth for "what Taurigo looks like".
 *
 * `packages/ui` mirrors these into Tailwind v4 `@theme` CSS variables (Phase 4);
 * everything else (Rust-rendered installer assets, icon generation, docs) should
 * read from here rather than hardcoding values.
 */

export const colors = {
  neutral: {
    50: "#f8fafc",
    100: "#f1f5f9",
    200: "#e2e8f0",
    300: "#cbd5e1",
    400: "#94a3b8",
    500: "#64748b",
    600: "#475569",
    700: "#334155",
    800: "#1e293b",
    900: "#0f172a",
    950: "#020617",
  },
  primary: {
    50: "#eef2ff",
    100: "#e0e7ff",
    200: "#c7d2fe",
    300: "#a5b4fc",
    400: "#818cf8",
    500: "#6366f1",
    600: "#4f46e5",
    700: "#4338ca",
    800: "#3730a3",
    900: "#312e81",
    950: "#1e1b4b",
  },
  semantic: {
    success: "#16a34a",
    warning: "#d97706",
    danger: "#dc2626",
    info: "#0284c7",
  },
} as const;

export const theme = {
  light: {
    background: colors.neutral[50],
    foreground: colors.neutral[900],
    muted: colors.neutral[100],
    mutedForeground: colors.neutral[500],
    border: colors.neutral[200],
    primary: colors.primary[600],
    primaryForeground: colors.neutral[50],
    ring: colors.primary[500],
  },
  dark: {
    background: colors.neutral[950],
    foreground: colors.neutral[50],
    muted: colors.neutral[900],
    mutedForeground: colors.neutral[400],
    border: colors.neutral[800],
    primary: colors.primary[500],
    primaryForeground: colors.neutral[50],
    ring: colors.primary[400],
  },
} as const;

export const radii = {
  none: "0px",
  sm: "0.25rem",
  md: "0.375rem",
  lg: "0.5rem",
  xl: "0.75rem",
  "2xl": "1rem",
  full: "9999px",
} as const;

export const fontStack = {
  sans: [
    "Inter",
    "-apple-system",
    "BlinkMacSystemFont",
    "Segoe UI",
    "Roboto",
    "Helvetica Neue",
    "Arial",
    "sans-serif",
  ].join(", "),
  mono: [
    "JetBrains Mono",
    "ui-monospace",
    "SFMono-Regular",
    "Menlo",
    "Consolas",
    "Liberation Mono",
    "monospace",
  ].join(", "),
} as const;

// 4px base unit, matching Tailwind's default scale so packages/ui's @theme
// mirror stays a 1:1 mapping rather than a second scale to keep in sync.
export const spacing = {
  0: "0px",
  1: "0.25rem",
  2: "0.5rem",
  3: "0.75rem",
  4: "1rem",
  5: "1.25rem",
  6: "1.5rem",
  8: "2rem",
  10: "2.5rem",
  12: "3rem",
  16: "4rem",
  20: "5rem",
  24: "6rem",
} as const;

export const tokens = { colors, theme, radii, fontStack, spacing } as const;

export default tokens;
