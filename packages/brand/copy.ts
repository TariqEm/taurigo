/**
 * Taurigo product copy — app naming and identifiers used across the Tauri
 * config, installer metadata, and UI chrome (window title, about dialog, …).
 *
 * `legalName` and `bundleIdentifier` are placeholders until a real legal
 * entity / reverse-DNS domain is decided — update them before a public release
 * (Phase 14 reads `bundleIdentifier` into `tauri.conf.json`'s `identifier`).
 */

export const copy = {
  appName: "Taurigo",
  tagline: "A fast, native-feeling desktop app starter.",
  legalName: "Taurigo",
  bundleIdentifier: "com.taurigo.app",
} as const;

export default copy;
